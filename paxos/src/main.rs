use std::time::Duration;

use clap::Parser;
use config::Args;
use domain::{message::Message, node::Node, proposer_node::ProposerNode};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing_subscriber::EnvFilter;

mod actors;
mod config;
mod domain;
mod handlers;
mod network;
mod repository;

/// General rules:
/// Only a value that has been proposed may be chosen.
/// A process never learns that a value has been chosen unless it actually has been.
#[tokio::main]
async fn main() {
    let Args { nodes, rounds } = Args::parse();

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));
    tracing_subscriber::fmt()
        .with_env_filter(filter_layer)
        .without_time()
        .with_target(false)
        .init();

    // FIXME: this number should (probably?) be the same as the number of nodes.
    // Decrease this and handle `Lagged` error.
    let (broadcast_tx, _) = broadcast::channel::<Message>(1000);
    let (proposer_tx, proposer_rx) = mpsc::channel::<Message>(nodes);
    let (client_tx, client_rx) = mpsc::channel::<u64>(nodes);

    let mut proposer = ProposerNode::new(broadcast_tx.clone(), proposer_rx, client_rx);

    tokio::spawn(async move {
        proposer.run().await.expect("could not run proposer");
    });

    for i in 0..nodes {
        let node_subscriber = broadcast_tx.subscribe();
        let mut acceptor = Node::new(i as u64, proposer_tx.clone(), node_subscriber);

        tokio::spawn(async move {
            acceptor.run().await.expect("could not run acceptor {i}");
        });
    }

    for i in 0..rounds {
        client_tx
            .send(i as u64)
            .await
            .expect("could not send value to proposer");
        sleep(Duration::from_millis(300)).await;
    }
}

impl Drop for ProposerNode {
    fn drop(&mut self) {
        println!("Proposer dropped");
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Acceptor dropped");
    }
}
