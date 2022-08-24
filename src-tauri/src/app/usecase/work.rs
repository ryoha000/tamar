use std::sync::Arc;

use crate::kernel::model::work::NewerTitleWork;
use crate::kernel::model::Id;
use crate::kernel::repository::work::WorkRepository;
use crate::{adapter::modules::RepositoriesModuleExt, kernel::model::work::Work};
use derive_new::new;

use crate::app::model::work::{
    CreateWork, SearchAroundTitleWorkView, SearchAroundUpdatedAtWorkView, SearchEqualWork,
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

    pub async fn search_equal_work(&self, source: SearchEqualWork) -> anyhow::Result<Option<Work>> {
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
