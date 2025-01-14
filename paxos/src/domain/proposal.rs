use super::id::ProposalId;

/// A proposal is a message sent by a **proposer** to the **acceptors**,
/// containing the id of the proposal and a value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Copy)]
pub struct Proposal {
    pub id: ProposalId,
    pub value: u64,
}
