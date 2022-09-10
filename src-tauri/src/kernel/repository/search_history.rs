use crate::kernel::model::{
    search_history::{NewSearchHistory, SearchHistory},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait SearchHistoryRepository {
    async fn find(&self, id: &Id<SearchHistory>) -> anyhow::Result<Option<SearchHistory>>;
    async fn insert(&self, source: NewSearchHistory) -> anyhow::Result<()>;
    async fn select_recent(&self, limit: u32) -> anyhow::Result<Vec<SearchHistory>>;
}
