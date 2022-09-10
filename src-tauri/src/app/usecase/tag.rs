use std::sync::Arc;

use crate::app::model::tag_view::{SearchByNameTagView, SelectTagView};
use crate::kernel::model::tag::Tag;
use crate::kernel::repository::tag::TagRepository;
use crate::{adapter::modules::RepositoriesModuleExt, app::model::tag_view::TagView};
use derive_new::new;

use crate::app::model::tag::{CreateTag, GetByNameTag, GetTag};

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

    pub async fn find_tag(&self, source: GetTag) -> anyhow::Result<TagView> {
        Ok(TagView::new(
            self.repositories
                .tag_repository()
                .find(&source.id)
                .await?
                .ok_or(anyhow::anyhow!("tag is not found"))?,
        ))
    }

    pub async fn find_tag_by_name(&self, source: GetByNameTag) -> anyhow::Result<Option<Tag>> {
        self.repositories
            .tag_repository()
            .find_by_name(source.name)
            .await
    }

    pub async fn select(&self, source: SelectTagView) -> anyhow::Result<Vec<TagView>> {
        let tags = self
            .repositories
            .tag_repository()
            .select(source.limit)
            .await?
            .into_iter()
            .map(|v| TagView::new(v))
            .collect();

        Ok(tags)
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
