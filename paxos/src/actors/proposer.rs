use anyhow::Result;
use tracing::{debug, info};
use uuid::Uuid;

use crate::domain::{
    id::{BrandedUuid, ProposalId},
    message::{Message, MessageMetadata},
    proposal::Proposal,
    proposer_node::ProposerNode,
};

// TODO: probably does not need to be mutable.
#[async_trait::async_trait]
pub trait Proposer {
    async fn run(&mut self) -> Result<()>;
    async fn send_prepare_request(&mut self, value: u64) -> Result<()>;
    async fn send_accept_request(&mut self) -> Result<()>;
    async fn handle_prepare_response(
        &mut self,
        received_response: Message,
    ) -> Result<()>;
    async fn handle_accept_response(&mut self, metadata: MessageMetadata)
        -> Result<()>;
}

#[async_trait::async_trait]
impl Proposer for ProposerNode {
    #[tracing::instrument(skip(self))]
    async fn run(&mut self) -> Result<()> {
        loop {
            match self.network_interface.receive().await? {
                Some(Message::ClientRequest { value }) => {
                    debug!("received client request");
                    self.send_prepare_request(value).await?;
                }
                Some(Message::PrepareResponse { metadata }) => {
                    self.handle_prepare_response(Message::PrepareResponse { metadata })
                        .await?;
                }
                Some(Message::AcceptResponse { metadata }) => {
                    self.handle_accept_response(metadata).await?;
                }
                _ => (),
            }
        }
    }

    #[tracing::instrument(skip(self))]
    async fn send_prepare_request(&mut self, value: u64) -> Result<()> {
        let proposal_id = ProposalId(Uuid::now_v7());
        let new_proposal = Proposal::new(value, proposal_id);
        self.proposal_history.entry(proposal_id).or_insert(value);
        debug!("current proposal history {:?}", &self.proposal_history);

        self.latest_proposal = Some(new_proposal);
        let active_acceptors_count = self
            .network_interface
            .broadcast(Message::new_prepare(self.id, proposal_id))
            .await?;

        debug!("proposing for {} acceptors", active_acceptors_count);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn send_accept_request(&mut self) -> Result<()> {
        let latest_proposal_id = self.latest_proposal.unwrap().id;
        let active_acceptors_count = self
            .network_interface
            .broadcast(Message::new_accept_request(self.id, latest_proposal_id))
            .await?;
        debug!("accept sent for {} acceptors", active_acceptors_count);

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn handle_prepare_response(
        &mut self,
        received_response: Message,
    ) -> Result<()> {
        let Message::PrepareResponse { metadata } = received_response else {
            return Ok(());
        };
        let MessageMetadata {
            issuer_id,
            proposal_id: received_proposal_id,
        } = metadata;

        if let Some(latest_proposal) = self.latest_proposal {
            // If there's a node that received a more up-to-date proposal, we use it
            // to update the proposed value for the next iterations.
            if received_proposal_id > latest_proposal.id {
                let proposal_value = self
                    .proposal_history
                    .get(&received_proposal_id)
                    .ok_or(anyhow::anyhow!(
                    "could not find proposal {} in history",
                    received_proposal_id.to_string()
                ))?;
                self.latest_proposal = Some(Proposal {
                    id: received_proposal_id,
                    value: *proposal_value,
                })
            }
        }

        self.prepared_nodes.insert(issuer_id);

        if self.prepared_nodes.len()
            > self.network_interface.active_listeners().await? / 2
        {
            self.send_accept_request().await?;
        }

        debug!("received prepare response from node {}", issuer_id);
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn handle_accept_response(
        &mut self,
        metadata: MessageMetadata,
    ) -> Result<()> {
        let MessageMetadata {
            issuer_id,
            proposal_id: received_proposal_id,
        } = metadata;

        let value = self
            .proposal_history
            .get(&received_proposal_id)
            .expect("TODO");

        debug!(
            value,
            issuer_id,
            proposal_id = received_proposal_id.formatted(),
            "received accepted value",
        );
        self.accepted_value_nodes.insert(issuer_id);

        if self.accepted_value_nodes.len()
            > self.network_interface.active_listeners().await? / 2
        {
            // At this point, we reached consensus. However, there will still be some
            // remaining accept responses to be received by the proposer.
            info!(
                "quorum reached by {}, value {} accepted",
                self.accepted_value_nodes.len(),
                value
            );
        }

        Ok(())
    }
}
