use crate::proposal::id::ProposalId;

#[derive(Debug, Clone)]
pub struct MessageMetadata {
    pub issuer_id: u64,
    pub proposal_id: ProposalId,
}

#[derive(Debug, Clone)]
pub enum Message {
    /// Message sent by the client and received by the proposer node, containing a new
    /// value.
    ClientRequest {
        value: u64,
    },
    /// Message sent by the proposer to all the acceptors. It is the first exchange
    /// between proposer and acceptors of the protocol.
    PrepareRequest {
        metadata: MessageMetadata,
    },
    /// Message sent by the acceptors, containing the latest proposal set to be
    /// accepted, if any.
    PrepareResponse {
        metadata: MessageMetadata,
    },
    /// Message sent by the proposer to all nodes asking them to accept a value.
    AcceptRequest {
        metadata: MessageMetadata,
    },
    // Message sent by the acceptors **iff the value has been accepted**.
    AcceptResponse {
        metadata: MessageMetadata,
    },
}

impl Message {
    pub fn new_prepare(issuer_id: u64, proposal_id: ProposalId) -> Self {
        Self::PrepareRequest {
            metadata: MessageMetadata {
                issuer_id,
                proposal_id,
            },
        }
    }

    pub fn new_accept_request(issuer_id: u64, proposal_id: ProposalId) -> Self {
        Self::AcceptRequest {
            metadata: MessageMetadata {
                issuer_id,
                proposal_id,
            },
        }
    }
}
