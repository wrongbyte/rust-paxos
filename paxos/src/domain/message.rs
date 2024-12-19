use super::{id::NodeId, proposal::Proposal};

/// A message is how we communicate between proposers, acceptors and learners.
#[derive(Debug)]
pub enum MessageType {
    /// Proposer sends a message to all nodes with a proposed value.
    PrepareRequest,
    /// Message sent by the acceptors. The acceptor agrees not to accept any
    /// value older than the one sent by the proposer, and returns the last
    /// accepted value to the proposer.
    PrepareResponse,
    /// Proposer sends a message to all nodes with a value to be committed.
    AcceptRequest,
    // Message sent by the acceptors. The acceptor can agree or reject to
    // commit the value sent by the proposer.
    AcceptResponse,
}

/// A message is how we communicate between proposers, acceptors and learners.
#[derive(Debug)]
pub struct Message {
    pub r#type: MessageType,
    pub issuer_id: NodeId,
    pub proposal: Proposal,
}
