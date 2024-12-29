use std::io::Error;

use tokio::sync::{
    broadcast,
    broadcast::error as broadcast_error,
    mpsc::{
        error as mpsc_error, {self},
    },
};

use super::{id::ProposalId, learner::LearnMessage, message::Message};

#[derive(Debug)]
pub enum NodeError<T> {
    RepositoryError { error: rusqlite::Error },
    InvalidStateError { error: String }, // TODO: improve
    CommunicationError { error: String },
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
    /// Interface to send the accepted value to the learner.
    pub learner_sender: mpsc::Sender<LearnMessage>,
    /// Buffer that stores temporarily the id of the latest proposal set to be
    /// accepted in this node.
    pub buffer: Option<ProposalId>,
}

impl Node {
    pub fn new(
        id: u64,
        proposer_sender: mpsc::Sender<Message>,
        proposer_receiver: broadcast::Receiver<Message>,
        learner_sender: mpsc::Sender<LearnMessage>,
    ) -> Self {
        Self {
            id,
            proposer_sender,
            proposer_receiver,
            learner_sender,
            buffer: None,
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        // It has to be a infinite loop because otherwise, Nodes are dropped after
        // receiving the first message and the channel closes.
        loop {
            let received_message = self
                .proposer_receiver
                .recv()
                .await
                .expect("error receiving message");

            match received_message {
                Message::PrepareRequest { body } => self
                    .reply_prepare_request(body)
                    .await
                    .expect("could not reply to prepare request, node {self.id}"),
                Message::AcceptRequest { body } => self
                    .reply_accept_request(body)
                    .await
                    .expect("could not reply to accept request, node {self.id}"),
                _ => (),
            };
            // TODO: improve.
            if false {
                break;
            }
        }
        Ok(())
    }
}
