use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::model::search_history::SearchHistory;
use crate::kernel::repository::search_history::SearchHistoryRepository;
use derive_new::new;

use crate::app::model::search_history::CreateSearchHistory;

#[derive(new)]
pub struct SearchHistoryUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> SearchHistoryUseCase<R> {
    pub async fn register_search_history(&self, source: CreateSearchHistory) -> anyhow::Result<()> {
        self.repositories
            .search_history_repository()
            .insert(source.try_into()?)
            .await
    }

    pub async fn get_recent_search_history(
        &self,
        limit: u32,
    ) -> anyhow::Result<Vec<SearchHistory>> {
        self.repositories
            .search_history_repository()
            .select_recent(limit)
            .await
    }
}
