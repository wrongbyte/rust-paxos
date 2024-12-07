use super::id::{NodeId, ProposalId};

/// A proposal is a message sent by a **proposer** to the **acceptors**, containing the id of the proposal and a value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Proposal {
    #[serde(flatten)]
    metadata: ProposeMetadata,
    value: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProposeMetadata {
    id: ProposalId,
    issuer: NodeId,
}
