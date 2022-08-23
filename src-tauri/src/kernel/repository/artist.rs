use crate::kernel::model::{
    artist::{Artist, NewArtist, SearchAlsoUsingWorkArtist},
    Id,
};
use async_trait::async_trait;

#[async_trait]
pub trait ArtistRepository {
    async fn find(&self, id: &Id<Artist>) -> anyhow::Result<Option<Artist>>;
    async fn find_by_name(&self, name: String) -> anyhow::Result<Option<Artist>>;
    async fn search_also_using_work(
        &self,
        source: SearchAlsoUsingWorkArtist,
    ) -> anyhow::Result<Vec<Artist>>;
    async fn insert(&self, source: NewArtist) -> anyhow::Result<()>;
}
