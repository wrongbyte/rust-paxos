use actors::proposer::Proposer;
use domain::{learner::Learner, message::Message, node::Node};
use repository::ValueRepositoryImpl;
use tokio::sync::{broadcast, mpsc};

pub mod actors;
pub mod domain;
pub mod repository;

/// General rules
/// Only a value that has been proposed may be chosen.
/// A process never learns that a value has been chosen unless it actually has been.
#[tokio::main]
async fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let fibonacci = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let number_nodes = 5;
    let (broadcast_tx, _) = broadcast::channel::<Message>(number_nodes);
    let (proposer_tx, proposer_rx) = mpsc::channel::<Message>(number_nodes);
    let (learner_tx, learner_rx) = mpsc::channel::<Message>(number_nodes);

    let mock_repository = ValueRepositoryImpl;
    let proposer = Proposer::new(broadcast_tx.clone(), proposer_rx);
    let learner = Learner::new(learner_rx, Box::new(mock_repository));

    for i in 0..=number_nodes {
        let node_subscriber = broadcast_tx.subscribe();
        let acceptor = Node::new(
            i as u64,
            proposer_tx.clone(),
            node_subscriber,
            learner_tx.clone(),
        );

        // Each participant has its own thread.
        tokio::spawn(async move {
            acceptor.run().await.expect("could not run acceptor {i}");
        });
    }
}
