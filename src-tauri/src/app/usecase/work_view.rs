use std::sync::Arc;

use crate::app::model::artist_view::ArtistView;
use crate::app::model::tag_view::TagView;
use crate::kernel::model::artist::Artist;
use crate::kernel::model::work::Work;
use crate::kernel::model::Id;
use crate::kernel::repository::file::FileRepository;
use crate::kernel::repository::tag::TagRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use crate::{
    adapter::modules::RepositoriesModuleExt, kernel::repository::artist::ArtistRepository,
};
use derive_new::new;

use crate::app::model::work_view::{
    GetWorkView, SearchWorkView, SelectByArtistView, WorkView, WorkViewSummary,
};

#[derive(new)]
pub struct WorkViewUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkViewUseCase<R> {
    async fn get_work_view_from_work(&self, work: Work) -> anyhow::Result<WorkView> {
        let artist = self
            .repositories
            .artist_repository()
            .find(&work.artist_id)
            .await?
            .ok_or(anyhow::anyhow!("artist is not found(internal error)"))?;

        let tag_ids = self
            .repositories
            .work_tag_map_repository()
            .find_by_work_id(&work.id)
            .await?
            .into_iter()
            .map(|v| v.tag_id)
            .collect();
        let tags = self
            .repositories
            .tag_repository()
            .find_by_ids(&tag_ids)
            .await?
            .into_iter()
            .map(|v| TagView::new(v))
            .collect();

        let image_paths = self
            .repositories
            .file_repository()
            .get_work_paths(&work.id)?;

        Ok(WorkView::new(
            work,
            image_paths,
            ArtistView::new(artist),
            tags,
        ))
    }

    async fn get_work_view_summary_from_work(&self, work: Work) -> anyhow::Result<WorkViewSummary> {
        let artist = self
            .repositories
            .artist_repository()
            .find(&work.artist_id)
            .await?
            .ok_or(anyhow::anyhow!("artist is not found(internal error)"))?;

        let work_list_thumbnail = self
            .repositories
            .file_repository()
            .get_work_list_thumbnail(&work.id)?;
        let artist_list_thumbnail = self
            .repositories
            .file_repository()
            .get_artist_list_thumbnail(&work.id)?;

        Ok(WorkViewSummary::new(
            work,
            work_list_thumbnail,
            artist_list_thumbnail,
            ArtistView::new(artist),
        ))
    }

    pub async fn get_work(&self, source: GetWorkView) -> anyhow::Result<WorkView> {
        let work = self
            .repositories
            .work_repository()
            .find(&source.id)
            .await?
            .ok_or(anyhow::anyhow!("work is not found"))?;

        self.get_work_view_from_work(work).await
    }

    pub async fn search(&self, source: SearchWorkView) -> anyhow::Result<Vec<WorkViewSummary>> {
        let works = self
            .repositories
            .work_repository()
            .search(source.try_into()?)
            .await?;
        let mut work_views = Vec::new();
        for work in works.into_iter() {
            work_views.push(self.get_work_view_summary_from_work(work).await?);
        }
        Ok(work_views)
    }

    pub async fn select_by_artist(
        &self,
        source: SelectByArtistView,
    ) -> anyhow::Result<Vec<WorkViewSummary>> {
        let id = Id::<Artist>::new(ulid::Ulid::from_string(&source.id)?);

        let works = self
            .repositories
            .work_repository()
            .find_by_artist(&id)
            .await?;
        let mut work_views = Vec::new();
        for work in works.into_iter() {
            work_views.push(self.get_work_view_summary_from_work(work).await?);
        }
        Ok(work_views)
    }
}
