use crate::domain::{
    id::ProposalId, message::Message, node::NodeError, proposal::Proposal,
};
pub struct ValueRepositoryImpl;

impl Proposal {
    pub fn new(value: u64, id: ProposalId) -> Self {
        Self { value, id }
    }
}

#[async_trait::async_trait]
pub trait ValueRepository {
    async fn get_latest_value(&self) -> Result<Option<Proposal>, NodeError<Message>>;
    async fn write_latest_value(
        &self,
        value: Proposal,
    ) -> Result<Option<Proposal>, NodeError<Message>>;
}

#[async_trait::async_trait]
impl ValueRepository for ValueRepositoryImpl {
    async fn get_latest_value(&self) -> Result<Option<Proposal>, NodeError<Message>> {
        todo!()
    }

    async fn write_latest_value(
        &self,
        value: Proposal,
    ) -> Result<Option<Proposal>, NodeError<Message>> {
        todo!()
    }
}
