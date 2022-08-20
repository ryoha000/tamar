use crate::kernel::model::{
    work::{NewWork, Work},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkRepository {
    async fn find(&self, id: &Id<Work>) -> anyhow::Result<Option<Work>>;
    async fn insert(&self, source: NewWork) -> anyhow::Result<()>;
}
