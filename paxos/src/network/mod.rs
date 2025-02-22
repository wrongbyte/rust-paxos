use anyhow::Result;

use crate::domain::message::Message;
pub mod acceptor;
pub mod channels;
pub mod proposer;

#[async_trait::async_trait]
pub trait Network {
    async fn broadcast(&self, message: Message) -> Result<usize>;
    async fn send(&self, message: Message) -> Result<()>;
    async fn receive(&self) -> Result<Message>;
    async fn active_listeners(&self) -> Result<usize>;
}
