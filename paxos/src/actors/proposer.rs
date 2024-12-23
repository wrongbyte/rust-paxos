use std::collections::HashMap;

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

    // TODO: how do we guarantee the value proposed is the correct one, aside of the
    // proposal id? I guess using a key-value store (can it be in memory?)
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
                // If there's a node that received a more updated proposal, we use it to
                // update the proposed value for next iterations.
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
                    error: "received prepare response without having sent a propose"
                        .into(),
                });
            }
        }
        Ok(())
    }
}
