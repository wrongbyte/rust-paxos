use std::time::Duration;

use clap::Parser;
use config::Args;
use domain::message::Message;
use network::{
    acceptor::channels::AcceptorChannels, proposer::channels::ProposerChannels,
};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing::debug;

use crate::{
    acceptor::{Acceptor, AcceptorNode},
    proposer::{Proposer, ProposerNode},
};
mod acceptor;
mod config;
mod domain;
mod network;
mod proposer;
mod repository;

/// General rules:
/// Only a value that has been proposed may be chosen.
/// A process never learns that a value has been chosen unless it actually has been.
#[tokio::main]
async fn main() {
    let Args { nodes, rounds } = Args::parse();

    config::init_logging();

    // FIXME: this number should (probably?) be the same as the number of nodes.
    // Decrease this and handle `Lagged` error.
    let (broadcast_tx, _) = broadcast::channel::<Message>(1000);
    let (proposer_tx, proposer_rx) = mpsc::channel::<Message>(nodes);

    let proposer_channels = ProposerChannels {
        sender: broadcast_tx.clone(),
        receiver: proposer_rx,
    };

    let mut proposer = ProposerNode::new(Box::new(proposer_channels));

    tokio::spawn(async move {
        proposer.run().await.expect("could not run `proposer");
    });

    // Create all nodes
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
        debug!("sending value {i} to acceptors");
        proposer_tx.clone().send(message).await.expect("");
        sleep(Duration::from_millis(100)).await;
    }
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
