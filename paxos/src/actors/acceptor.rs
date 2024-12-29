use crate::domain::{
    message::{AcceptPhaseBody, Message, PreparePhaseBody},
    node::{Node, NodeError},
};

impl Node {
    #[tracing::instrument(skip(self))]
    pub async fn reply_prepare_request(
        &mut self,
        received_proposal: PreparePhaseBody,
    ) -> Result<(), NodeError<Message>> {
        let proposal_id_string = received_proposal.proposal_id.into_inner().to_string();

        let msg = format!(
            "received proposal {} in node {}",
            proposal_id_string, self.id
        );
        println!("{msg}");

        // Get latest value that is set to be accepted in this node.
        if let Some(proposal_in_buffer) = self.buffer {
            let up_to_date_proposal =
                if proposal_in_buffer > received_proposal.proposal_id {
                    println!("proposal in buffer is more updated");
                    proposal_in_buffer
                } else {
                    println!("proposal received is more updated");
                    received_proposal.proposal_id
                };
            // The value stored in the node buffer is more up-to-date than the one
            // received in the message. **Don't** update the buffer, and reply with
            // the up-to-date value stored.
            // The proposal received is more up-to-date than the one we have stored
            // in the buffer. Update the buffer and reply with the
            // proposal received.
            self.buffer = Some(up_to_date_proposal);

            self.proposer_sender
                .send(Message::PrepareResponse {
                    body: PreparePhaseBody {
                        issuer_id: self.id,
                        proposal_id: up_to_date_proposal,
                    },
                })
                .await
                .map_err(|e| NodeError::ProposerSenderError { error: e })

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
    }

    /// If the value is accepted:
    ///  - reply to the proposer with an ACK message
    ///  - send the accepted value to the learner
    /// If the value is not accepted, simply ignore the message received and do nothing.
    #[tracing::instrument(skip(self))]
    pub async fn reply_accept_request(
        &mut self,
        received_proposal: AcceptPhaseBody,
    ) -> Result<(), NodeError<Message>> {
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
    }
}
