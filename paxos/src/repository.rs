use crate::domain::{id::ProposalId, node::NodeError, proposal::Proposal};
pub struct ValueRepositoryImpl;

impl Proposal {
    pub fn new(value: u64, id: ProposalId) -> Self {
        Self { value, id }
    }
}

#[async_trait::async_trait]
pub trait ValueRepository {
    async fn get_latest_value(&self) -> Result<Option<Proposal>, NodeError>;
    async fn write_latest_value(
        &self,
        value: Proposal,
    ) -> Result<Option<Proposal>, NodeError>;
}

#[async_trait::async_trait]
impl ValueRepository for ValueRepositoryImpl {
    async fn get_latest_value(&self) -> Result<Option<Proposal>, NodeError> {
        todo!()
    }

    async fn write_latest_value(
        &self,
        value: Proposal,
    ) -> Result<Option<Proposal>, NodeError> {
        todo!()
    }
}
