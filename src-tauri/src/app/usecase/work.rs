use std::sync::Arc;

use crate::kernel::repository::work::WorkRepository;
use crate::{adapter::modules::RepositoriesModuleExt, kernel::model::work::Work};
use derive_new::new;

use crate::app::model::work::{CreateWork, SearchEqualWork};

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

    pub async fn search_equal_work(&self, source: SearchEqualWork) -> anyhow::Result<Option<Work>> {
        self.repositories
            .work_repository()
            .find_by_title_and_artist(source.title, &source.artist_id)
            .await
    }
}
