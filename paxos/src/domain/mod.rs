pub mod message;
pub mod node;
pub mod proposal;
pub mod proposer_node;

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

    pub struct NodeId(pub Uuid);

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

    impl Deref for NodeId {
        type Target = Uuid;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
