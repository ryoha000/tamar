use derive_new::new;
use std::sync::Arc;

use crate::{
    adapter::modules::RepositoriesModuleExt,
    app::model::artist_view::{ArtistView, SearchArtistView},
    kernel::repository::artist::ArtistRepository,
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
}
