use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::model::work_history::WorkHistory;
use crate::kernel::repository::work_history::WorkHistoryRepository;
use derive_new::new;

use crate::app::model::work_history::CreateWorkHistory;

#[derive(new)]
pub struct WorkHistoryUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkHistoryUseCase<R> {
    pub async fn register_work_history(&self, source: CreateWorkHistory) -> anyhow::Result<()> {
        self.repositories
            .work_history_repository()
            .insert(source.try_into()?)
            .await
    }
}
