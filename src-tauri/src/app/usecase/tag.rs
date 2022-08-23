use std::sync::Arc;

use crate::app::model::tag_view::SearchByNameTagView;
use crate::kernel::model::tag::Tag;
use crate::kernel::repository::tag::TagRepository;
use crate::{adapter::modules::RepositoriesModuleExt, app::model::tag_view::TagView};
use derive_new::new;

use crate::app::model::tag::{CreateTag, SearchEqualTag};

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

    pub async fn search_equal_tag(&self, source: SearchEqualTag) -> anyhow::Result<Option<Tag>> {
        self.repositories
            .tag_repository()
            .find_by_name(source.name)
            .await
    }

    pub async fn search_by_name(
        &self,
        source: SearchByNameTagView<'_>,
    ) -> anyhow::Result<Vec<TagView>> {
        let tags = self
            .repositories
            .tag_repository()
            .search_by_name(source.name)
            .await?
            .into_iter()
            .map(|v| TagView::new(v))
            .collect();

        Ok(tags)
    }
}
