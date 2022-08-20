use crate::kernel::model::{
    tag::{NewTag, Tag},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait TagRepository {
    async fn find(&self, id: &Id<Tag>) -> anyhow::Result<Option<Tag>>;
    async fn insert(&self, source: NewTag) -> anyhow::Result<()>;
}
