use tracing::info;

use crate::domain::{
    message::{Message, PreparePhaseBody},
    node::{Node, NodeError},
};

impl Node {
    #[tracing::instrument(skip(self))]
    pub async fn reply_prepare_request(
        &mut self,
        received_message: Message,
    ) -> Result<(), NodeError<Message>> {
        if let Message::PrepareRequest {
            body: received_proposal,
        } = received_message
        {
            let proposal_id_string =
                received_proposal.proposal_id.into_inner().to_string();
            let node_id_string = self.id.into_inner().to_string();

            let msg = format!(
                "received proposal {} in node {}",
                proposal_id_string, node_id_string
            );
            info!(msg);

            // Get latest value that is set to be accepted in this node.
            if let Some(proposal_in_buffer) = self.buffer {
                // The value stored in the node buffer is more up-to-date than the one
                // received in the message. **Don't** update the buffer, and reply with
                // the up-to-date value stored.
                if proposal_in_buffer > received_proposal.proposal_id {
                    self.proposer_sender
                        .send(Message::PrepareResponse {
                            body: PreparePhaseBody {
                                issuer_id: self.id,
                                proposal_id: proposal_in_buffer,
                            },
                        })
                        .await
                        .map_err(|e| NodeError::ProposerSenderError { error: e })
                // The proposal received is more up-to-date than the one we have stored
                // in the buffer. Update the buffer and reply with the
                // proposal received.
                } else {
                    self.buffer = Some(received_proposal.proposal_id);
                    self.proposer_sender
                        .send(Message::PrepareResponse {
                            body: PreparePhaseBody {
                                issuer_id: self.id,
                                proposal_id: received_proposal.proposal_id,
                            },
                        })
                        .await
                        .map_err(|e| NodeError::ProposerSenderError { error: e })
                }
            // This node has not set any value to be accepted, so according to the
            // algorithm, we set the first value received to be accepted.
            } else {
                self.buffer = Some(received_proposal.proposal_id);

                self.proposer_sender
                    .send(Message::PrepareResponse {
                        body: PreparePhaseBody {
                            issuer_id: self.id,
                            proposal_id: received_proposal.proposal_id,
                        },
                    })
                    .await
                    .map_err(|e| NodeError::ProposerSenderError { error: e })
            }
        } else {
            Err(NodeError::InvalidStateError { error: "".into() }) // TODO
        }
    }

    /// If the value is accepted:
    ///  - reply to the proposer with an ACK message
    ///  - send the accepted value to the learner
    /// If the value is not accepted, simply ignore the message received and do nothing.
    #[tracing::instrument(skip(self))]
    pub async fn reply_accept_request(
        &mut self,
        received_message: Message,
    ) -> Result<(), NodeError<Message>> {
        if let Message::AcceptRequest {
            body: received_proposal,
        } = received_message
        {
            // Get latest value that is set to be accepted in this node.
            if let Some(proposal_in_buffer) = self.buffer {
                // The value stored in the node buffer is more up-to-date than the one
                // received in the message. **Don't** update the buffer, and reply with
                // the up-to-date proposal stored.
                if proposal_in_buffer > received_proposal.proposal_id {
                    Ok(())
                // The value received is more up-to-date than the one we have stored in
                // the buffer. **Accept** the proposal (answer the proposer and
                // send the accepted value to learners)
                } else {
                    // Clear the buffer after accepting the value.
                    self.buffer = None;

                    self.proposer_sender
                        .send(Message::AcceptResponse)
                        .await
                        .map_err(|e| NodeError::ProposerSenderError { error: e })
                }

            // This node has not set any value to be accepted, so according to the
            // algorithm, we accept it. There is no need to clear the buffer because it
            // is already empty.
            } else {
                self.proposer_sender
                    .send(Message::AcceptResponse)
                    .await
                    .map_err(|e| NodeError::ProposerSenderError { error: e })
            }
        } else {
            Err(NodeError::InvalidStateError { error: "".into() }) // TODO: describe
                                                                   // error
        }
    }
}
