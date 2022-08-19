use async_trait::async_trait;
use crate::kernel::model::{
    artist::{Artist, NewArtist},
    Id,
};

#[async_trait]
pub trait ArtistRepository {
    async fn find(&self, id: &Id<Artist>) -> anyhow::Result<Option<Artist>>;
    async fn insert(&self, source: NewArtist) -> anyhow::Result<()>;
}
