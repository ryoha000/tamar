use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use derive_new::new;

use crate::app::model::work_tag_map::CreateWorkTagMap;

#[derive(new)]
pub struct WorkTagMapUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkTagMapUseCase<R> {
    pub async fn register_work_tag_map(&self, source: CreateWorkTagMap) -> anyhow::Result<()> {
        self.repositories
            .work_tag_map_repository()
            .insert(source.try_into()?)
            .await
    }
}
