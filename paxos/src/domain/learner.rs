use tokio::sync::mpsc;
use uuid::Uuid;

use super::{id::NodeId, message::Message, node::NodeError, proposal::Proposal};
use crate::repository::ValueRepository;

/// The learner is a node that has the role of receiving messages from acceptors and
/// committing the value upon a majority of nodes has agreed on the value.
pub struct Learner {
    pub id: NodeId,
    pub receiver: mpsc::Receiver<Message>,
    /// Buffer that stores the value for the node temporarily before
    /// commiting the changes in the database.
    pub repository: Box<dyn ValueRepository>,
}

impl Learner {
    pub fn new(
        receiver: mpsc::Receiver<Message>,
        repository: Box<dyn ValueRepository>,
    ) -> Self {
        let id = NodeId::unchecked_from_inner(Uuid::now_v7());
        Self {
            id,
            receiver,
            repository,
        }
    }

    /// Given we have a value accepted by any majority of nodes, we commit this value to
    /// the database.
    pub async fn commit_learned_value(
        &mut self,
        received_message: Message,
    ) -> Result<(), NodeError<Message>> {
        if let Message::CommitRequest { body } = received_message {
            self.repository
                .write_latest_value(Proposal {
                    value: body.value,
                    id: body.proposal_id,
                })
                .await?;
        }
        // TODO
        Ok(())
    }
}
