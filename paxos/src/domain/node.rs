use std::io::Error;

use tokio::sync::mpsc::{Receiver, Sender};

use super::{id::NodeId, message::Message};

/// A node contains its unique ID as well as the sender and receiver interfaces
/// to communicate with other nodes using messages.
pub struct Node {
    pub id: NodeId,
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
    /// Buffer that stores the value for the node temporarily before
    /// commiting the changes in the database.
    pub buffer: Option<u64>,
}

impl Node {
    async fn run(&mut self) -> Result<(), Error> {
        while let Some(message) = self.receiver.recv().await {
            match message {
                Message::PrepareRequest(proposal) => todo!(),
                Message::PrepareResponse(proposal) => todo!(),
                Message::CommitRequest => todo!(),
                Message::CommitResponse => todo!(),
            }
        }
        Ok(())
    }
}
