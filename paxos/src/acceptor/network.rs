use anyhow::Result;
use tokio::sync::{broadcast, mpsc};

use crate::{message::Message, network::Network};

pub struct AcceptorChannels {
    /// Interface to send messages **to** the proposer. This is mpsc (multiple senders
    /// send to a single consumer, which in this case is the proposer).
    pub sender: mpsc::Sender<Message>,
    /// Interface to receive messages **from** the proposer. Remember, the proposer
    /// broadcasts proposals.
    pub receiver: broadcast::Receiver<Message>,
}

#[async_trait::async_trait]
impl Network for AcceptorChannels {
    async fn broadcast(&self, _: Message) -> Result<usize> {
        unimplemented!("broadcasting is not supported for acceptors")
    }

    async fn send(&self, message: Message) -> Result<()> {
        self.sender.send(message).await?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<Option<Message>> {
        self.receiver.recv().await.map(Some).map_err(Into::into)
    }

    async fn active_listeners(&self) -> Result<usize> {
        unimplemented!("active_listeners is not supported for acceptors")
    }
}
