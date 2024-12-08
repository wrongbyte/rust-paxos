use super::proposal::Proposal;

/// A message is how we communicate between proposers, acceptors and learners.
#[derive(Debug)]
pub enum Message {
    /// Proposer sends a message to all nodes with a proposed value.
    PrepareRequest(Proposal),
    /// Message sent by the acceptors. The acceptor agrees not to accept any value older than the one sent by the proposer, and returns the last accepted value to the proposer.
    PrepareResponse(Proposal),
    /// Proposer sends a message to all nodes with a value to be committed.
    CommitRequest,
    // Message sent by the acceptors. The acceptor can agree or reject to commit the value sent by the proposer.
    CommitResponse,
}
