use std::sync::Arc;

use crate::adapter::modules::RepositoriesModuleExt;
use crate::kernel::repository::tag::TagRepository;
use derive_new::new;

use crate::app::model::tag::CreateTag;

#[derive(new)]
pub struct TagUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> TagUseCase<R> {
    pub async fn register_tag(&self, source: CreateTag) -> anyhow::Result<()> {
        let existed = self
            .repositories
            .tag_repository()
            .find_by_name(source.name.clone())
            .await?;
        if existed.is_some() {
            return Ok(()); // TODO: Err にしなくていいか考える
        }
        self.repositories
            .tag_repository()
            .insert(source.try_into()?)
            .await
    }
}
