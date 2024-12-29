use super::id::ProposalId;

// TODO: separate acceptor and proposer messages.
#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct PreparePhaseBody {
    pub issuer_id: u64,
    pub proposal_id: ProposalId,
}

#[derive(Debug, Clone)]
pub struct AcceptPhaseBody {
    pub issuer_id: u64,
    pub proposal_id: ProposalId,
    pub value: u64,
}
