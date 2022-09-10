use std::sync::Arc;

use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use crate::{adapter::modules::RepositoriesModuleExt, kernel::model::work_tag_map::WorkTagMap};
use derive_new::new;

use crate::app::model::work_tag_map::{CreateWorkTagMap, DetachWorkTagMap, GetWorkAttachedTags};

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

    pub async fn delete_work_tag_map(&self, source: DetachWorkTagMap) -> anyhow::Result<()> {
        self.repositories
            .work_tag_map_repository()
            .delete(source.try_into()?)
            .await
    }

    pub async fn get_work_attached_tags(
        &self,
        source: GetWorkAttachedTags,
    ) -> anyhow::Result<Vec<WorkTagMap>> {
        self.repositories
            .work_tag_map_repository()
            .find_by_tag_ids(source.tag_ids)
            .await
    }
}
