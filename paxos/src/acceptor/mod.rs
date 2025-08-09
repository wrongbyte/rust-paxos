use anyhow::Result;
use tracing::debug;

use crate::{
    domain::{
        id::{BrandedUuid, ProposalId},
        message::{Message, MessageMetadata},
    },
    network::Network,
};
pub struct AcceptorNode {
    /// Identifier of the node.
    // TODO: this should probably be an uuid, that will be stored in non-volatile
    // memory to keep track of nodes, especially those thay may die and then restart.
    pub id: u64,
    /// Interface to communicate with other nodes.
    pub network_interface: Box<dyn Network + Send + Sync>,
    /// Buffer that stores temporarily the id of the latest proposal set to be
    /// accepted in this node.
    pub buffer: Option<ProposalId>,
}

impl AcceptorNode {
    pub fn new(id: u64, network_interface: Box<dyn Network + Send + Sync>) -> Self {
        Self {
            id,
            network_interface,
            buffer: None,
        }
    }
}

#[async_trait::async_trait]
pub trait Acceptor {
    async fn run(&mut self) -> Result<()>;
    async fn reply_prepare_request(
        &mut self,
        message_metadata: MessageMetadata,
    ) -> Result<()>;
    async fn reply_accept_request(
        &mut self,
        message_metadata: MessageMetadata,
    ) -> Result<()>;
}

#[async_trait::async_trait]
impl Acceptor for AcceptorNode {
    #[tracing::instrument(skip_all, fields(
        node_id = self.id,
    ))]
    async fn run(&mut self) -> Result<()> {
        loop {
            match self.network_interface.receive().await? {
                Some(Message::PrepareRequest { metadata }) => {
                    self.reply_prepare_request(metadata).await?;
                }
                Some(Message::AcceptRequest { metadata, .. }) => {
                    self.reply_accept_request(metadata).await?;
                }
                _ => (),
            }
        }
    }

    #[tracing::instrument(skip_all, fields(
        node_id = self.id,
        proposal_id = message_metadata.proposal_id.formatted()
    ))]
    async fn reply_prepare_request(
        &mut self,
        message_metadata: MessageMetadata,
    ) -> Result<()> {
        debug!("received proposal");
        let MessageMetadata { proposal_id, .. } = message_metadata;

        // Get latest value that is set to be accepted in this node.
        if let Some(proposal_in_buffer) = self.buffer {
            let up_to_date_proposal = if proposal_in_buffer > proposal_id {
                debug!("proposal in node {} buffer is more updated", self.id);
                proposal_in_buffer
            } else {
                debug!("proposal received in node {} is more updated", self.id);
                proposal_id
            };
            self.buffer = Some(up_to_date_proposal);

            self.network_interface
                .send(Message::PrepareResponse {
                    metadata: MessageMetadata {
                        issuer_id: self.id,
                        proposal_id: up_to_date_proposal,
                    },
                })
                .await
                .map_err(anyhow::Error::from)?;

        // This node has not set any value to be accepted, so according to the
        // algorithm, we set the first value received to be accepted.
        } else {
            self.buffer = Some(proposal_id);

            self.network_interface
                .send(Message::PrepareResponse {
                    metadata: MessageMetadata {
                        issuer_id: self.id,
                        proposal_id,
                    },
                })
                .await
                .map_err(anyhow::Error::from)?;
        }

        Ok(())
    }

    /// If the value is accepted:
    ///  - reply to the proposer with an ACK message
    ///  - send the accepted value to the learner
    /// If the value is not accepted, simply ignore the message received and do nothing.
    #[tracing::instrument(skip_all, fields(
        node_id = self.id,
        proposal_id = message_metadata.proposal_id.formatted()
    ))]
    async fn reply_accept_request(
        &mut self,
        message_metadata: MessageMetadata,
    ) -> Result<()> {
        let MessageMetadata {
            proposal_id,
            issuer_id,
        } = message_metadata;
        debug!(issuer_id, "received accept request");

        let accept_response = Message::AcceptResponse {
            metadata: MessageMetadata {
                issuer_id: self.id,
                proposal_id,
            },
        };
        if let Some(proposal_in_buffer) = self.buffer {
            // Do not accept the value if the one in buffer is more updated.
            if proposal_in_buffer > proposal_id {
                return Ok(());
            } else {
                // The value received is more up-to-date than the one we have stored in
                // the buffer. **Accept** the proposal (answer the proposer and
                // send the accepted value to learners)
                // Clear the buffer after accepting the value.
                self.buffer = None;

                self.network_interface
                    .send(accept_response)
                    .await
                    .map_err(anyhow::Error::from)?;

                debug!("node is ready for the next decree");
                return Ok(());
            }

        // This node has not yet set any value to be accepted, so according to the
        // algorithm, we accept the first value received. There is no need to clear the
        // buffer because it is already empty.
        } else {
            self.network_interface
                .send(accept_response)
                .await
                .map_err(anyhow::Error::from)?;
        }

        Ok(())
    }
}
