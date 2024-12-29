use std::{collections::HashMap, io::Error, sync::Arc};

use tokio::sync::mpsc;
use tracing::debug;

use super::{id::ProposalId, message::Message, node::NodeError, proposal::Proposal};
use crate::repository::ValueRepository;

#[derive(Debug, Clone)]
pub struct LearnMessage {
    pub value: u64,
    pub node_id: u64,
    pub proposal_id: ProposalId,
}

pub type DynValueRepository = dyn ValueRepository + Send + Sync + 'static;

/// The learner is a node that has the role of receiving messages from acceptors and
/// committing the value upon a majority of nodes has agreed on the value.
pub struct Learner {
    pub receiver: mpsc::Receiver<LearnMessage>,
    /// A map of node ids and their current accepted values.
    pub ballot_counter: HashMap<u64, u64>,
    /// Buffer that stores the value for the node temporarily before
    /// commiting the changes in the database.
    pub repository: Arc<DynValueRepository>,
}

impl Learner {
    pub fn new(
        receiver: mpsc::Receiver<LearnMessage>,
        repository: Arc<DynValueRepository>,
    ) -> Self {
        let ballot_counter = HashMap::new();
        Self {
            receiver,
            repository,
            ballot_counter,
        }
    }

    #[tracing::instrument(skip_all)]
    pub async fn run(&mut self) -> Result<(), Error> {
        loop {
            while let Some(msg) = self.receiver.recv().await {
                let LearnMessage {
                    value,
                    node_id,
                    proposal_id,
                } = msg;
                let span = tracing::span!(tracing::Level::DEBUG, "learner");
                let _ = span.enter();
                
                debug!(
                    value,
                    node_id,
                    proposal_id = proposal_id.into_inner().to_string(),
                    "received accepted value",
                );
                //TODO: verify if the majority has accepted, if so, commit to database.
            }
        }
    }

    /// Given we have a value accepted by any majority of nodes, we commit this value to
    /// the database.
    #[tracing::instrument(skip(self))]
    pub async fn commit_learned_value(
        &mut self,
        received_message: LearnMessage,
    ) -> Result<(), NodeError<Message>> {
        let LearnMessage {
            value,
            node_id,
            proposal_id,
        } = received_message;
        debug!("commiting value received from node {node_id}");

        self.repository
            .write_latest_value(Proposal {
                value,
                id: proposal_id,
            })
            .await?;

        Ok(())
    }
}
