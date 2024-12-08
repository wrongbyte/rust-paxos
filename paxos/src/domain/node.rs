use std::{
    io::Error,
    sync::mpsc::{Receiver, Sender},
};

use super::{id::NodeId, message::Message};

/// A node contains its unique ID as well as the sender and receiver interfaces to communicate with other nodes using messages.
pub struct Node {
    pub id: NodeId,
    pub sender: Sender<Message>,
    pub receiver: Receiver<Message>,
}

impl Node {
    async fn run(&mut self) -> Result<(), Error> {
        while let Some(message) = self.receiver.recv().await {}
        Ok(())
    }
}
