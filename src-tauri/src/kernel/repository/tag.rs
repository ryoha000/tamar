use crate::kernel::model::{
    tag::{NewTag, Tag},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait TagRepository {
    async fn find(&self, id: &Id<Tag>) -> anyhow::Result<Option<Tag>>;
    async fn find_by_ids(&self, ids: &Vec<Id<Tag>>) -> anyhow::Result<Vec<Tag>>;
    async fn insert(&self, source: NewTag) -> anyhow::Result<()>;
    async fn find_by_name(&self, name: String) -> anyhow::Result<Option<Tag>>;
    async fn select(&self, limit: u16) -> anyhow::Result<Vec<Tag>>;
    async fn search_by_name(&self, name: &str) -> anyhow::Result<Vec<Tag>>;
}
