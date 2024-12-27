use std::io::Error;

use tokio::sync::{
    broadcast,
    broadcast::error as broadcast_error,
    mpsc::{
        error as mpsc_error, {self},
    },
};

use super::{id::ProposalId, message::Message};

pub enum NodeError<T> {
    RepositoryError { error: rusqlite::Error },
    InvalidStateError { error: String }, // TODO: improve
    ProposerSenderError { error: mpsc_error::SendError<T> },
    ProposerReceiverError { error: broadcast_error::RecvError },
    LearnerSenderError { error: mpsc_error::SendError<T> },
}

pub struct Node {
    /// Identifier of the node.
    // TODO: this should probably be an uuid, that will be stored in non-volatile
    // memory to keep track of nodes, especially those thay may die and then restart.
    pub id: u64,
    /// Interface to send messages **to** the proposer. This is mpsc (multiple senders
    /// send to a single consumer, which in this case is the proposer).
    pub proposer_sender: mpsc::Sender<Message>,
    /// Interface to receive messages **from** the proposer. Remember, the proposer
    /// broadcasts proposals.
    pub proposer_receiver: broadcast::Receiver<Message>,
    /// Interface to send the accepted value to the learner. TODO: we should have more
    /// than one learner.
    pub learner_sender: mpsc::Sender<Message>,
    /// Buffer that stores temporarily the id of the latest proposal set to be
    /// accepted in this node.
    pub buffer: Option<ProposalId>,
}

impl Node {
    pub fn new(
        id: u64,
        proposer_sender: mpsc::Sender<Message>,
        proposer_receiver: broadcast::Receiver<Message>,
        learner_sender: mpsc::Sender<Message>,
    ) -> Self {
        Self {
            id,
            proposer_sender,
            proposer_receiver,
            learner_sender,
            buffer: None,
        }
    }

    async fn run(&mut self) -> Result<(), Error> {
        todo!();
    }
}
