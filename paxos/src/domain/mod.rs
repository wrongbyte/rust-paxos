pub mod learner;
pub mod message;
pub mod node;
pub mod proposal;

pub mod id {
    bty::brand!(
        pub type ProposalId = uuid::Uuid;
        pub type NodeId = uuid::Uuid;
    );
}
