use std::time::Duration;

use actors::proposer::Proposer;
use domain::{message::Message, node::Node};
use repository::ValueRepositoryImpl;
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing_subscriber::EnvFilter;

pub mod actors;
pub mod domain;
pub mod repository;

/// General rules:
/// Only a value that has been proposed may be chosen.
/// A process never learns that a value has been chosen unless it actually has been.
#[tokio::main]
async fn main() {
    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .without_time()
        .with_target(false)
        .init();
    // let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let fibonacci = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
    let number_nodes = 10;
    let (broadcast_tx, _) = broadcast::channel::<Message>(1000);
    let (proposer_tx, proposer_rx) = mpsc::channel::<Message>(number_nodes);
    let (client_tx, client_rx) = mpsc::channel::<u64>(number_nodes);

    let _value_repository = ValueRepositoryImpl; // TODO: use real impl
    let mut proposer =
        Proposer::new(broadcast_tx.clone(), proposer_rx, client_rx, number_nodes);

    tokio::spawn(async move {
        proposer.run().await.expect("could not run proposer");
    });

    for i in 0..number_nodes {
        let node_subscriber = broadcast_tx.subscribe();
        let mut acceptor = Node::new(i as u64, proposer_tx.clone(), node_subscriber);

        tokio::spawn(async move {
            acceptor.run().await.expect("could not run acceptor {i}");
        });
    }

    for i in 0..5 {
        client_tx
            .send(i)
            .await
            .expect("could not send value to proposer");
    }
    sleep(Duration::from_secs(10)).await;
}

// impl Drop for Proposer {
//     fn drop(&mut self) {
//         println!("Proposer dropped");
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         println!("Acceptor dropped");
//     }
// }
