use std::collections::{HashMap, HashSet};

use tokio::sync::{broadcast, mpsc};
use uuid::Uuid;

use crate::domain::{
    id::ProposalId,
    message::{AcceptPhaseBody, Message, PreparePhaseBody},
    node::NodeError,
    proposal::Proposal,
};

/// Node that broadcast proposals to all the acceptors. All the information stored in
/// this struct is ephemeral, being erased once the round completes.
pub struct Proposer {
    /// Identifier of the node.
    pub id: u64,
    /// Interface to receive values from the client, that are assigned an unique id  to
    /// be broadcast to all the nodes as a proposal.
    pub client_receiver: mpsc::Receiver<u64>,
    /// Interface to broadcast messages to the acceptors.
    pub acceptor_sender: broadcast::Sender<Message>,
    /// Interface to receive messages **from** the acceptors.
    pub acceptor_receiver: mpsc::Receiver<Message>,
    /// Buffer that stores temporarily the id and value of the latest proposal set to
    /// be accepted by any acceptor.
    pub latest_proposal: Option<Proposal>,
    /// History of proposals sent by this proposer, and their respective values.
    pub proposal_history: HashMap<ProposalId, u64>,
    pub active_nodes: u64,
    /// Number of nodes that replied to the prepare request.
    pub prepared_nodes: HashSet<u64>,
}

impl Proposer {
    pub fn new(
        acceptor_sender: broadcast::Sender<Message>,
        acceptor_receiver: mpsc::Receiver<Message>,
        client_receiver: mpsc::Receiver<u64>,
    ) -> Self {
        let id = 1; // TODO: change when there's more than one proposer
        let proposal_history = HashMap::new();
        let active_nodes = 0;
        let prepared_nodes = HashSet::new();
        Self {
            id,
            acceptor_sender,
            acceptor_receiver,
            client_receiver,
            latest_proposal: None,
            proposal_history,
            active_nodes,
            prepared_nodes,
        }
    }

    pub async fn run(&mut self) -> Result<(), NodeError<Message>> {
        // Listen to both channels simultaneously.
        loop {
            tokio::select! {
                Some(client_value) = self.client_receiver.recv() => {
                    self.send_prepare_request(client_value)?;
                },
                Some(received_message) = self.acceptor_receiver.recv() => {
                    match received_message {
                        Message::PrepareResponse { body } => {
                            self.handle_prepare_response(body)?;
                        }
                        Message::AcceptResponse => self.handle_accept_response(),
                        _ => (),
                    }
                },
                else => {
                    // Both channels are closed, so we break out of the loop.
                    break;
                }
            }
        }
        Ok(())
    }

    /// The beginning of the protocol. The proposer broadcasts a proposal to all the
    /// acceptors, using a value it received from the client.
    /// In this step, we also store how many nodes are active. This information is then
    /// later used for computations that rely on quorum.
    #[tracing::instrument(skip(self))]
    pub fn send_prepare_request(
        &mut self,
        value: u64,
    ) -> Result<(), NodeError<Message>> {
        let proposal_id = ProposalId::unchecked_from_inner(Uuid::now_v7());
        let new_proposal = Proposal::new(value, proposal_id);
        self.proposal_history.entry(proposal_id).or_insert(value);
        dbg!(&self.proposal_history);

        self.latest_proposal = Some(new_proposal);

        let acceptor_sender_clone = self.acceptor_sender.clone();

        let active_acceptors = acceptor_sender_clone
            .send(Message::PrepareRequest {
                body: PreparePhaseBody {
                    issuer_id: self.id,
                    proposal_id,
                },
            })
            .expect("could not broadcast proposals");

        self.active_nodes = active_acceptors as u64;
        let msg = format!(
            "proposer {} proposed value {} for {} acceptors",
            self.id, value, active_acceptors
        );
        println!("{msg}");
        Ok(())
    }

    ///
    #[tracing::instrument(skip(self))]
    pub fn handle_prepare_response(
        &mut self,
        received_proposal: PreparePhaseBody,
    ) -> Result<(), NodeError<Message>> {
        let received_proposal_id = received_proposal.proposal_id;
        let node_id = received_proposal.issuer_id;
        println!(
            "received prepare response from node {}",
            received_proposal.issuer_id
        );

        if let Some(latest_proposal) = self.latest_proposal {
            // If there's a node that received a more up-to-date proposal, we use it
            // to update the proposed value for the next iterations.
            if received_proposal_id > latest_proposal.id {
                let proposal_value = self
                    .proposal_history
                    .get(&received_proposal_id)
                    .ok_or(NodeError::InvalidStateError {
                    error: format!(
                        "could not find proposal {} in history",
                        received_proposal_id.into_inner().to_string()
                    ),
                })?;
                self.latest_proposal = Some(Proposal {
                    id: received_proposal_id,
                    value: *proposal_value,
                })
            }
        }

        self.prepared_nodes.insert(node_id);
        if self.prepared_nodes.iter().count() as u64 > self.active_nodes / 2 {
            self.send_accept_request()?;
        }

        Ok(())
    }

    /// The
    #[tracing::instrument(skip(self))]
    pub fn send_accept_request(&mut self) -> Result<(), NodeError<Message>> {
        let latest_proposal_id = self.latest_proposal.unwrap().id;
        let proposal_value = self.proposal_history.get(&latest_proposal_id).ok_or(
            NodeError::InvalidStateError {
                error: format!(
                    "could not find proposal {} in history",
                    latest_proposal_id.into_inner().to_string()
                ),
            },
        )?;

        // self.acceptor_sender =
        let active_acceptors = self
            .acceptor_sender
            .send(Message::AcceptRequest {
                body: AcceptPhaseBody {
                    issuer_id: self.id,
                    proposal_id: latest_proposal_id,
                    value: *proposal_value,
                },
            })
            .inspect_err(|e| println!("error: {e}"))
            .expect("could not broadcast accept messages");

        let msg = format!(
            "proposer {} sent accept request for {} acceptors",
            self.id, active_acceptors
        );
        println!("{msg}");

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn handle_accept_response(&self) {}
}
