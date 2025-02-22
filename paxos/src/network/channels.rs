use tokio::sync::{broadcast, mpsc};

use crate::domain::message::Message;

pub struct AcceptorChannels {
    /// Interface to send messages **to** the proposer. This is mpsc (multiple senders
    /// send to a single consumer, which in this case is the proposer).
    pub sender: mpsc::Sender<Message>,
    /// Interface to receive messages **from** the proposer. Remember, the proposer
    /// broadcasts proposals.
    pub receiver: broadcast::Receiver<Message>,
}

pub struct ProposerChannels {
    /// Interface to broadcast messages to the acceptors.
    pub sender: broadcast::Sender<Message>,
    /// Interface to receive messages **from** the acceptors.
    pub receiver: mpsc::Receiver<Message>,
}
