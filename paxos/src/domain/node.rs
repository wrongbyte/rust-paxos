use std::io::Error;

use tokio::sync::{
    broadcast,
    broadcast::error as broadcast_error,
    mpsc::{
        error as mpsc_error, {self},
    },
};

use super::{
    id::{NodeId, ProposalId},
    message::Message,
};

pub enum NodeError<T> {
    RepositoryError { error: rusqlite::Error },
    InvalidStateError { error: String }, // TODO: improve
    ProposerSenderError { error: mpsc_error::SendError<T> },
    ProposerReceiverError { error: broadcast_error::RecvError },
    LearnerSenderError { error: mpsc_error::SendError<T> },
}

pub struct Node {
    /// Identifier of the node.
    pub id: NodeId,
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
    async fn run(&mut self) -> Result<(), Error> {
        // while let Some(message) = self.receiver.recv().await {
        //     match message.r#type {
        //         MessageType::PrepareRequest => todo!(),
        //         MessageType::PrepareResponse => todo!(),
        //         MessageType::AcceptRequest => todo!(),
        //         MessageType::AcceptResponse => todo!(),
        //     }
        // }
        Ok(())
    }
}
