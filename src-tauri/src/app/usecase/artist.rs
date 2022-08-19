use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::repository::artist::ArtistRepository;
use derive_new::new;

use crate::app::model::artist::CreateArtist;

#[derive(new)]
pub struct ArtistUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> ArtistUseCase<R> {
    pub async fn register_artist(&self, source: CreateArtist) -> anyhow::Result<()> {
        self.repositories
            .artist_repository()
            .insert(source.try_into()?)
            .await
    }
}
