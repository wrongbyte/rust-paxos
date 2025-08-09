use anyhow::Result;

use crate::proposal::Proposal;

pub struct ValueRepositoryImpl;

#[expect(dead_code)]
#[async_trait::async_trait]
pub trait ValueRepository {
    async fn get_latest_value(&self) -> Result<Option<Proposal>>;
    async fn write_latest_value(&self, value: Proposal) -> Result<()>;
}

#[async_trait::async_trait]
impl ValueRepository for ValueRepositoryImpl {
    async fn get_latest_value(&self) -> Result<Option<Proposal>> {
        todo!()
    }

    async fn write_latest_value(&self, _value: Proposal) -> Result<()> {
        todo!()
    }
}
