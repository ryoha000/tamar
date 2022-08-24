use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::model::artist::Artist;
use crate::kernel::repository::artist::ArtistRepository;
use derive_new::new;

use crate::app::model::artist::{CreateArtist, GetByNameArtist};

#[derive(new)]
pub struct ArtistUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> ArtistUseCase<R> {
    pub async fn register_artist(&self, source: CreateArtist) -> anyhow::Result<()> {
        let existed = self
            .repositories
            .artist_repository()
            .find_by_name(source.name.clone())
            .await?;
        if existed.is_some() {
            return Ok(()); // TODO: Err にしなくていいか考える
        }
        self.repositories
            .artist_repository()
            .insert(source.try_into()?)
            .await
    }

    pub async fn find_artist_by_name(
        &self,
        source: GetByNameArtist,
    ) -> anyhow::Result<Option<Artist>> {
        self.repositories
            .artist_repository()
            .find_by_name(source.name)
            .await
    }
}
