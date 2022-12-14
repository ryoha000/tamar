use std::sync::Arc;

use crate::kernel::model::work::{
    NewImportWork, NewWork, NewerArtistIdWork, NewerTitleWork, SearchAroundViewTimeWork,
};
use crate::kernel::model::Id;
use crate::kernel::repository::file::FileRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::repository::work_history::WorkHistoryRepository;
use crate::{adapter::modules::RepositoriesModuleExt, kernel::model::work::Work};
use derive_new::new;
use ulid::Ulid;

use crate::app::model::work::{
    CreateWork, DeleteWork, GetByTitleWork, ImportWork, SearchAroundTitleWorkView,
    SearchAroundUpdatedAtWorkView, SearchAroundViewTimeWorkView, UpdateArtistIdWork,
    UpdateTitleWork,
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

    pub async fn import_work(&self, source: ImportWork) -> anyhow::Result<()> {
        let file_path = source.file_path.clone();
        let existed = self
            .repositories
            .work_repository()
            .find_by_title_and_artist(source.title.clone(), &source.artist_id)
            .await?;
        if existed.is_some() {
            return Ok(()); // TODO: Err にしなくていいか考える
        }

        let new_work: NewWork = source.try_into()?;
        let datetime = self
            .repositories
            .file_repository()
            .get_modified_at(file_path)?;
        let new_import_work = NewImportWork::new(
            new_work.id,
            new_work.title,
            new_work.artist_id,
            datetime.clone(),
            datetime,
        );
        self.repositories
            .work_repository()
            .insert_import(new_import_work)
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

    pub async fn search_around_view_time(
        &self,
        source: SearchAroundViewTimeWorkView,
    ) -> anyhow::Result<Vec<Id<Work>>> {
        let latest = self
            .repositories
            .work_history_repository()
            .find_latest(&Id::new(Ulid::from_string(&source.work_id)?))
            .await?;
        match latest {
            Some(latest) => {
                let work_ids = self
                    .repositories
                    .work_repository()
                    .search_around_view_time(SearchAroundViewTimeWork::new(
                        source.limit,
                        source.is_before,
                        latest.updated_at,
                    ))
                    .await?
                    .into_iter()
                    .map(|v| v.id)
                    .collect();

                Ok(work_ids)
            }
            None => Ok(vec![]),
        }
    }
}
