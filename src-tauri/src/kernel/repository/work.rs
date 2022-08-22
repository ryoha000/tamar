use crate::kernel::model::{
    artist::Artist,
    work::{NewWork, SearchAroundTitleWork, SearchAroundUpdatedAtWork, SearchWork, Work},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkRepository {
    async fn search(&self, source: SearchWork) -> anyhow::Result<Vec<Work>>;
    async fn search_around_title(&self, source: SearchAroundTitleWork)
        -> anyhow::Result<Vec<Work>>;
    async fn search_around_updated_at(
        &self,
        source: SearchAroundUpdatedAtWork,
    ) -> anyhow::Result<Vec<Work>>;
    async fn find(&self, id: &Id<Work>) -> anyhow::Result<Option<Work>>;
    async fn find_by_title_and_artist(
        &self,
        title: String,
        artist_id: &Id<Artist>,
    ) -> anyhow::Result<Option<Work>>;
    async fn insert(&self, source: NewWork) -> anyhow::Result<()>;
}
