use crate::kernel::model::{
    artist::Artist,
    work::{NewWork, Work},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait WorkRepository {
    async fn find(&self, id: &Id<Work>) -> anyhow::Result<Option<Work>>;
    async fn find_by_title_and_artist(
        &self,
        title: String,
        artist_id: &Id<Artist>,
    ) -> anyhow::Result<Option<Work>>;
    async fn insert(&self, source: NewWork) -> anyhow::Result<()>;
}
