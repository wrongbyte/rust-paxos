use crate::domain::{
    message::{Message, MessageType},
    node::{Node, NodeError},
    proposal::Proposal,
};

impl Node {
    pub async fn reply_prepare_request(
        &mut self,
        received_message: Message,
    ) -> Result<Message, NodeError> {
        if let MessageType::PrepareRequest = received_message.r#type {
            let Message {
                proposal: received_proposal,
                ..
            } = received_message;
            // Get latest value that is set to be accepted in this node.
            if let Some(proposal_in_buffer) = self.buffer.as_ref() {
                // The value stored in the node buffer is more up-to-date than the one
                // received in the message. **Don't** update the buffer, and reply with
                // the up-to-date value stored.
                if proposal_in_buffer.id > received_proposal.id {
                    Ok(Message {
                        issuer_id: self.id,
                        r#type: MessageType::PrepareResponse,
                        proposal: Proposal {
                            value: proposal_in_buffer.value,
                            id: proposal_in_buffer.id,
                        },
                    })
                // The value received is more up-to-date than the one we have stored in
                // the buffer. Update the buffer and reply with the proposal received.
                } else {
                    self.buffer = Some(received_proposal);
                    Ok(Message {
                        issuer_id: self.id,
                        proposal: received_proposal,
                        r#type: MessageType::PrepareResponse,
                    })
                }
            // This node has not set any value to be accepted, so according to the
            // algorithm, we set the first value received to be accepted.
            } else {
                self.buffer = Some(received_proposal);

                Ok(Message {
                    r#type: MessageType::PrepareResponse,
                    issuer_id: self.id,
                    proposal: received_proposal,
                })
            }
        } else {
            Err(NodeError::InvalidStateError { error: "".into() }) // TODO: describe
                                                                   // error
        }
    }

    pub async fn reply_accept_request(
        &mut self,
        received_message: Message,
    ) -> Result<Message, NodeError> {
        if let MessageType::AcceptRequest = received_message.r#type {
            let Message {
                proposal: received_proposal,
                ..
            } = received_message;
            // Get latest value that is set to be accepted in this node.
            if let Some(proposal_in_buffer) = self.buffer.as_ref() {
                // The value stored in the node buffer is more up-to-date than the one
                // received in the message. **Don't** update the buffer, and reply with
                // the up-to-date value stored.
                if proposal_in_buffer.id > received_proposal.id {
                    Ok(Message {
                        issuer_id: self.id,
                        r#type: MessageType::AcceptResponse, /* TODO: the accept phase has a response? */
                        proposal: Proposal {
                            value: proposal_in_buffer.value,
                            id: proposal_in_buffer.id,
                        },
                    })
                // The value received is more up-to-date than the one we have stored in
                // the buffer. **Accept** the proposal (commit it to the database) and
                // reply with it.
                } else {
                    self.repository
                        .write_latest_value(received_proposal)
                        .await?;

                    // Clear the buffer after accepting the value.
                    self.buffer = None;

                    Ok(Message {
                        issuer_id: self.id,
                        proposal: received_proposal,
                        r#type: MessageType::PrepareResponse,
                    })
                }

            // This node has not set any value to be accepted, so according to the
            // algorithm, we accept it. There is no need to clear the buffer because it
            // is already empty.
            } else {
                self.repository
                    .write_latest_value(received_proposal)
                    .await?;

                Ok(Message {
                    r#type: MessageType::PrepareResponse,
                    issuer_id: self.id,
                    proposal: received_proposal,
                })
            }
        } else {
            Err(NodeError::InvalidStateError { error: "".into() }) // TODO: describe
                                                                   // error
        }
    }
}
