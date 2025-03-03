use std::time::Duration;

use actors::{acceptor::Acceptor, proposer::Proposer};
use clap::Parser;
use config::Args;
use domain::{message::Message, node::AcceptorNode, proposer_node::ProposerNode};
use network::{
    acceptor::channels::AcceptorChannels, proposer::channels::ProposerChannels, Network,
};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing_subscriber::EnvFilter;

mod actors;
mod config;
mod domain;
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

    let proposer_channels = ProposerChannels {
        sender: broadcast_tx.clone(),
        receiver: proposer_rx,
    };

    for i in 0..nodes {
        let acceptor_channels = AcceptorChannels {
            sender: proposer_tx.clone(),
            receiver: broadcast_tx.subscribe(),
        };

        let mut acceptor = AcceptorNode::new(i as u64, Box::new(acceptor_channels));

        tokio::spawn(async move {
            acceptor.run().await.expect("could not run acceptor {i}");
        });
    }

    for i in 0..rounds {
        let message = Message::ClientRequest { value: i as u64 };
        proposer_channels.send(message).await.expect("");
        sleep(Duration::from_millis(300)).await;
    }

    let mut proposer = ProposerNode::new(Box::new(proposer_channels));

    tokio::spawn(async move {
        proposer.run().await.expect("could not run `proposer");
    });
}

impl Drop for ProposerNode {
    fn drop(&mut self) {
        println!("Proposer dropped");
    }
}

impl Drop for AcceptorNode {
    fn drop(&mut self) {
        println!("Acceptor dropped");
    }
}
