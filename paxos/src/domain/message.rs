use super::id::{NodeId, ProposalId};

#[derive(Debug)]
pub enum Message {
    /// Message sent by the proposer to all the acceptors. It is the first message of
    /// the protocol.
    PrepareRequest {
        body: PreparePhaseBody,
    },
    /// Message sent by the acceptors, containing the latest proposal set to be
    /// accepted, if any.
    PrepareResponse {
        body: PreparePhaseBody,
    },
    /// Proposer sends a message to all nodes telling them to accept a value.
    AcceptRequest {
        body: AcceptPhaseBody,
    },
    // Message sent by the acceptors **iff the value has been accepted**.
    AcceptResponse,
    /// Message sent to the learners, containing the value accepted by the acceptor.
    CommitRequest {
        body: AcceptPhaseBody,
    },
}

#[derive(Debug)]
pub struct PreparePhaseBody {
    pub issuer_id: NodeId,
    pub proposal_id: ProposalId,
}

#[derive(Debug)]
pub struct AcceptPhaseBody {
    pub issuer_id: NodeId,
    pub proposal_id: ProposalId,
    pub value: u64,
}
