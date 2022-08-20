use crate::kernel::model::{
    tag::Tag,
    work::Work,
    work_tag_map::{NewWorkTagMap, WorkTagMap},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkTagMapRepository {
    async fn find(&self, work_id: &Id<Work>, tag_id: &Id<Tag>) -> anyhow::Result<Vec<WorkTagMap>>;
    async fn insert(&self, source: NewWorkTagMap) -> anyhow::Result<()>;
}
