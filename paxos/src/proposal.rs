use crate::proposal::id::ProposalId;

/// A proposal is a message sent by a **proposer** to the **acceptors**,
/// containing the id of the proposal and a value.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Copy)]
pub struct Proposal {
    pub id: ProposalId,
    pub value: u64,
}

impl Proposal {
    pub fn new(value: u64, id: ProposalId) -> Self {
        Self { value, id }
    }
}

pub mod id {
    use std::ops::Deref;

    use uuid::Uuid;
    #[derive(
        PartialEq,
        PartialOrd,
        Eq,
        Ord,
        Hash,
        Debug,
        Clone,
        Copy,
        serde::Serialize,
        serde::Deserialize,
    )]
    pub struct ProposalId(pub Uuid);

    pub trait BrandedUuid {
        fn formatted(&self) -> String;
    }

    impl BrandedUuid for ProposalId {
        //Get only the last 6 characters of the string representation of the uuid.
        fn formatted(&self) -> String {
            let uuid_str = self.0.to_string();
            uuid_str[uuid_str.len() - 6..].to_string()
        }
    }

    impl Deref for ProposalId {
        type Target = Uuid;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
