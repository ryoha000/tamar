use crate::kernel::model::{
    work::Work,
    work_history::{NewWorkHistory, WorkHistory},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkHistoryRepository {
    async fn find(&self, id: &Id<WorkHistory>) -> anyhow::Result<Option<WorkHistory>>;
    async fn find_latest(&self, id: &Id<Work>) -> anyhow::Result<Option<WorkHistory>>;
    async fn insert(&self, source: NewWorkHistory) -> anyhow::Result<()>;
}
