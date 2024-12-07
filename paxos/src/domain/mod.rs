pub mod message;
pub mod proposal;

pub mod id {
    bty::brand!(
        pub type ProposalId = uuid::Uuid;
        pub type NodeId = uuid::Uuid;
    );
}
