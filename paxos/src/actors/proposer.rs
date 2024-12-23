use tokio::sync::{broadcast, mpsc};
use tracing::info;
use uuid::Uuid;

use crate::domain::{
    id::{NodeId, ProposalId},
    message::{Message, PreparePhaseBody},
    node::NodeError,
    proposal::Proposal,
};

pub struct Proposer {
    /// Identifier of the node.
    pub id: NodeId,
    /// Interface to broadcast messages to the acceptors.
    pub acceptor_sender: broadcast::Sender<Message>,
    /// Interface to receive messages **from** the acceptors.
    pub acceptor_receiver: mpsc::Receiver<Message>,
    /// Buffer that stores temporarily the id and value of the latest proposal set to
    /// be accepted by any acceptor.
    pub latest_proposal: Option<Proposal>,
}

impl Proposer {
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
            self.id.into_inner().to_string(),
            value,
            active_acceptors
        );
        info!(msg);

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    pub fn handle_prepare_request_response(
        &mut self,
        received_message: Message,
    ) -> Result<(), NodeError<Message>> {
        if let Message::PrepareResponse {
            body: received_proposal,
        } = received_message
        {}
        Ok(())
    }
}
