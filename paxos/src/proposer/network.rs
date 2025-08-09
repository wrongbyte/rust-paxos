use anyhow::Result;
use tokio::sync::{broadcast, mpsc};

use crate::{message::Message, network::Network};

pub struct ProposerChannels {
    /// Interface to broadcast messages to the acceptors.
    pub sender: broadcast::Sender<Message>,
    /// Interface to receive messages **from** the acceptors.
    pub receiver: mpsc::Receiver<Message>,
}

#[async_trait::async_trait]
impl Network for ProposerChannels {
    async fn broadcast(&self, message: Message) -> Result<usize> {
        Ok(self.sender.send(message)?)
    }

    async fn send(&self, message: Message) -> Result<()> {
        self.sender.send(message)?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<Message>> {
        Ok(self.receiver.recv().await)
    }

    async fn active_listeners(&self) -> Result<usize> {
        Ok(self.sender.receiver_count())
    }
}
