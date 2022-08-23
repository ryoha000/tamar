use derive_new::new;
use std::sync::Arc;

use crate::{
    adapter::modules::RepositoriesModuleExt,
    app::model::artist_view::{
        ArtistView, GetArtistView, SearchArtistView, SearchByNameArtistView,
    },
    kernel::{
        model::{artist::Artist, Id},
        repository::artist::ArtistRepository,
    },
};

#[derive(new)]
pub struct ArtistViewUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> ArtistViewUseCase<R> {
    pub async fn search(&self, source: SearchArtistView) -> anyhow::Result<Vec<ArtistView>> {
        let artists = self
            .repositories
            .artist_repository()
            .search_also_using_work(source.try_into()?)
            .await?
            .into_iter()
            .map(|v| ArtistView::new(v))
            .collect();

        Ok(artists)
    }

    pub async fn search_by_name(
        &self,
        source: SearchByNameArtistView<'_>,
    ) -> anyhow::Result<Vec<ArtistView>> {
        let artists = self
            .repositories
            .artist_repository()
            .search_by_name(source.name)
            .await?
            .into_iter()
            .map(|v| ArtistView::new(v))
            .collect();

        Ok(artists)
    }

    pub async fn get(&self, source: GetArtistView) -> anyhow::Result<ArtistView> {
        let id = Id::<Artist>::new(ulid::Ulid::from_string(&source.id)?);

        let artist = self
            .repositories
            .artist_repository()
            .find(&id)
            .await?
            .ok_or(anyhow::anyhow!("artist is not found"))?;

        Ok(ArtistView::new(artist))
    }
}
