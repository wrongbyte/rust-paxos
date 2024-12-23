use std::sync::mpsc::Receiver;

use super::{id::NodeId, message::Message, node::NodeError};
use crate::repository::ValueRepository;

/// The learner is a node that has the role of receiving messages from acceptors and
/// committing the value upon a majority of nodes has agreed on the value.
pub struct Learner {
    pub id: NodeId,
    pub receiver: Receiver<Message>,
    /// Buffer that stores the value for the node temporarily before
    /// commiting the changes in the database.
    pub repository: Box<dyn ValueRepository>,
}

impl Learner {
    /// Given we have a value accepted by any majority of nodes, we commit this value to
    /// the database.
    pub async fn commit_learned_value(
        &mut self,
        received_message: Message,
    ) -> Result<(), NodeError<Message>> {
        // TODO
        Ok(())
    }
}
