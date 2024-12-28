use std::collections::HashMap;

use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};
use uuid::Uuid;

use crate::domain::{
    id::ProposalId,
    message::{AcceptPhaseBody, Message, PreparePhaseBody},
    node::NodeError,
    proposal::Proposal,
};

pub struct Proposer {
    /// Identifier of the node.
    pub id: u64,
    /// Interface to receive values from the client, that are assigned an unique id  to
    /// be broadcast to all the nodes as a proposal.
    pub client_receiver: mpsc::Receiver<u64>,
    /// Interface to broadcast messages to the accaeptors.
    pub acceptor_sender: broadcast::Sender<Message>,
    /// Interface to receive messages **from** the acceptors.
    pub acceptor_receiver: mpsc::Receiver<Message>,
    /// Buffer that stores temporarily the id and value of the latest proposal set to
    /// be accepted by any acceptor.
    pub latest_proposal: Option<Proposal>,
}

impl Proposer {
    pub fn new(
        acceptor_sender: broadcast::Sender<Message>,
        acceptor_receiver: mpsc::Receiver<Message>,
        client_receiver: mpsc::Receiver<u64>,
    ) -> Self {
        let id = 1; // TODO: change when there's more than one proposer
        Self {
            id,
            acceptor_sender,
            acceptor_receiver,
            client_receiver,
            latest_proposal: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), NodeError<Message>> {
        while let Some(value) = self.client_receiver.recv().await {
            let proposal_id = ProposalId::unchecked_from_inner(Uuid::now_v7());
            let new_proposal = Proposal::new(value, proposal_id);
            self.latest_proposal = Some(new_proposal);
            self.send_prepare_request(value)?;
            // self.acceptor_sender
            //     .send(Message::PrepareRequest {
            //         body: PreparePhaseBody {
            //             issuer_id: self.id,
            //             proposal_id: proposal_id,
            //         },
            //     })
            //     .map_err(|e| {
            //         error!("could not broadcast");
            //         NodeError::<String>::CommunicationError {
            //             error: e.to_string(),
            //         }
            //     });
        }
        Ok(())
    }

    /// TODO: a proposer proposes on a client request
    #[tracing::instrument(skip(self))]
    pub fn send_prepare_request(
        &mut self,
        value: u64,
    ) -> Result<(), NodeError<Message>> {
        let proposal_id = ProposalId::unchecked_from_inner(Uuid::now_v7());
        let proposal = Proposal {
            id: proposal_id,
            value,
        };

        // If `None`, it means this proposal is our first and therefore the most
        // up-to-date.
        if self.latest_proposal.is_none() {
            self.latest_proposal = Some(proposal);
        }

        let active_acceptors = self
            .acceptor_sender
            .send(Message::PrepareRequest {
                body: PreparePhaseBody {
                    issuer_id: self.id,
                    proposal_id,
                },
            })
            .expect("could not broadcast proposals"); // TODO: use a decent error

        let msg = format!(
            "proposer {} proposed value {} for {} acceptors",
            self.id, value, active_acceptors
        );
        info!(msg);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn handle_prepare_response(
        &mut self,
        received_message: Message,
        proposals_history: &HashMap<ProposalId, u64>,
    ) -> Result<(), NodeError<Message>> {
        if let Message::PrepareResponse {
            body: received_proposal,
        } = received_message
        {
            let received_proposal_id = received_proposal.proposal_id;
            if let Some(latest_proposal) = self.latest_proposal {
                // If there's a node that received a more up-to-date proposal, we use it
                // to update the proposed value for the next iterations.
                if received_proposal_id > latest_proposal.id {
                    let proposal_value = proposals_history
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
            } else {
                // TODO: not sure if this should be an error
                return Err(NodeError::InvalidStateError {
                    error: "received prepare response without having sent a propose
                            first"
                        .into(),
                });
            }
        }
        Ok(())
    }

    /// VERIFY: it should only be sent when the majority of the acceptors have replied
    /// the prepare request. How do we check it?
    #[tracing::instrument(skip(self))]
    pub fn send_accept_request(
        &mut self,
        proposals_history: &HashMap<ProposalId, u64>,
    ) -> Result<(), NodeError<Message>> {
        let latest_proposal_id = self.latest_proposal.unwrap().id;
        let proposal_value = proposals_history.get(&latest_proposal_id).ok_or(
            NodeError::InvalidStateError {
                error: format!(
                    "could not find proposal {} in history",
                    latest_proposal_id.into_inner().to_string()
                ),
            },
        )?;

        let active_acceptors = self
            .acceptor_sender
            .send(Message::AcceptRequest {
                body: AcceptPhaseBody {
                    issuer_id: self.id,
                    proposal_id: latest_proposal_id,
                    value: *proposal_value,
                },
            })
            .expect("could not send accept messages");

        let msg = format!(
            "proposer {} sent accept request for {} acceptors",
            self.id, active_acceptors
        );
        info!(msg);

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn handle_accept_response(&self) {}
}
