use std::io::Error;

use tokio::sync::mpsc::{Receiver, Sender};

use super::{
    id::NodeId,
    message::{Message, MessageType},
    proposal::Proposal,
};
use crate::repository::ValueRepository;

pub enum NodeError {
    RepositoryError { error: rusqlite::Error },
    InvalidStateError { error: String }, // TODO: improve
}

/// A node contains its unique ID as well as the sender and receiver interfaces
/// to communicate with other nodes using messages.
pub struct Node {
    pub id: NodeId,
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
    /// Buffer that stores the value for the node temporarily before
    /// commiting the changes in the database.
    pub buffer: Option<Proposal>,
    pub repository: Box<dyn ValueRepository>,
}

impl Node {
    async fn run(&mut self) -> Result<(), Error> {
        while let Some(message) = self.receiver.recv().await {
            match message.r#type {
                MessageType::PrepareRequest => todo!(),
                MessageType::PrepareResponse => todo!(),
                MessageType::AcceptRequest => todo!(),
                MessageType::AcceptResponse => todo!(),
            }
        }
        Ok(())
    }
}
