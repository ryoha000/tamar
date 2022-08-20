use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::repository::work::WorkRepository;
use derive_new::new;

use crate::app::model::work::CreateWork;

#[derive(new)]
pub struct WorkUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkUseCase<R> {
    pub async fn register_work(&self, source: CreateWork) -> anyhow::Result<()> {
        self.repositories
            .work_repository()
            .insert(source.try_into()?)
            .await
    }
}
