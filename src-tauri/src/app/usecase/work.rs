use std::sync::Arc;

use crate::kernel::model::work::{NewerArtistIdWork, NewerTitleWork};
use crate::kernel::model::Id;
use crate::kernel::repository::file::FileRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::{adapter::modules::RepositoriesModuleExt, kernel::model::work::Work};
use derive_new::new;

use crate::app::model::work::{
    CreateWork, DeleteWork, GetByTitleWork, SearchAroundTitleWorkView,
    SearchAroundUpdatedAtWorkView, UpdateArtistIdWork, UpdateTitleWork,
};

#[derive(new)]
pub struct WorkUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkUseCase<R> {
    pub async fn register_work(&self, source: CreateWork) -> anyhow::Result<()> {
        let existed = self
            .repositories
            .work_repository()
            .find_by_title_and_artist(source.title.clone(), &source.artist_id)
            .await?;
        if existed.is_some() {
            return Ok(()); // TODO: Err にしなくていいか考える
        }
        self.repositories
            .work_repository()
            .insert(source.try_into()?)
            .await
    }

    pub async fn delete_work(&self, source: DeleteWork) -> anyhow::Result<()> {
        let work_id = source.to_work_id()?;

        self.repositories.work_repository().delete(&work_id).await?;
        self.repositories
            .file_repository()
            .delete_work_files(&work_id)
    }

    pub async fn update_work_title(&self, source: UpdateTitleWork) -> anyhow::Result<()> {
        let source: NewerTitleWork = source.try_into()?;
        let _ = self
            .repositories
            .work_repository()
            .find(&source.id)
            .await?
            .ok_or(anyhow::anyhow!("work is not found"));

        self.repositories
            .work_repository()
            .update_title(source)
            .await
    }

    pub async fn update_work_artist_id(&self, source: UpdateArtistIdWork) -> anyhow::Result<()> {
        let source: NewerArtistIdWork = source.try_into()?;
        let _ = self
            .repositories
            .work_repository()
            .find(&source.id)
            .await?
            .ok_or(anyhow::anyhow!("work is not found"));

        self.repositories
            .work_repository()
            .update_artist_id(source)
            .await
    }

    pub async fn get_by_title_work(&self, source: GetByTitleWork) -> anyhow::Result<Option<Work>> {
        self.repositories
            .work_repository()
            .find_by_title_and_artist(source.title, &source.artist_id)
            .await
    }

    pub async fn search_around_title(
        &self,
        source: SearchAroundTitleWorkView,
    ) -> anyhow::Result<Vec<Id<Work>>> {
        let work_ids = self
            .repositories
            .work_repository()
            .search_around_title(source.try_into()?)
            .await?
            .into_iter()
            .map(|v| v.id)
            .collect();

        Ok(work_ids)
    }

    pub async fn search_around_updated_at(
        &self,
        source: SearchAroundUpdatedAtWorkView,
    ) -> anyhow::Result<Vec<Id<Work>>> {
        let work_ids = self
            .repositories
            .work_repository()
            .search_around_updated_at(source.try_into()?)
            .await?
            .into_iter()
            .map(|v| v.id)
            .collect();

        Ok(work_ids)
    }
}
