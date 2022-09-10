use crate::kernel::model::{
    tag::Tag,
    work::Work,
    work_tag_map::{DeleteWorkTagMap, NewWorkTagMap, WorkTagMap},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkTagMapRepository {
    async fn find(
        &self,
        work_id: &Id<Work>,
        tag_id: &Id<Tag>,
    ) -> anyhow::Result<Option<WorkTagMap>>;
    async fn find_by_work_id(&self, work_id: &Id<Work>) -> anyhow::Result<Vec<WorkTagMap>>;
    async fn find_by_tag_ids(&self, tag_ids: Vec<Id<Tag>>) -> anyhow::Result<Vec<WorkTagMap>>;
    async fn insert(&self, source: NewWorkTagMap) -> anyhow::Result<()>;
    async fn delete(&self, source: DeleteWorkTagMap) -> anyhow::Result<()>;
}
