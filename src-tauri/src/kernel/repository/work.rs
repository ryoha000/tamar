use crate::kernel::model::{
    artist::Artist,
    work::{
        NewWork, NewerTitleWork, SearchAroundTitleWork, SearchAroundUpdatedAtWork, SearchWork, Work, NewerArtistIdWork,
    },
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
    async fn find_by_artist(&self, id: &Id<Artist>) -> anyhow::Result<Vec<Work>>;
    async fn find_by_title_and_artist(
        &self,
        title: String,
        artist_id: &Id<Artist>,
    ) -> anyhow::Result<Option<Work>>;
    async fn insert(&self, source: NewWork) -> anyhow::Result<()>;
    async fn update_title(&self, source: NewerTitleWork) -> anyhow::Result<()>;
    async fn update_artist_id(&self, source: NewerArtistIdWork) -> anyhow::Result<()>;
}
