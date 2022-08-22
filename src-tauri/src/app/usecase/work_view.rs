use std::sync::Arc;
use std::{fs, path};

use crate::app::model::artist_view::ArtistView;
use crate::app::model::tag_view::TagView;
use crate::kernel::model::work::Work;
use crate::kernel::repository::tag::TagRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use crate::{
    adapter::modules::RepositoriesModuleExt, kernel::repository::artist::ArtistRepository,
};
use derive_new::new;

use crate::app::model::work_view::{
    GetWorkView, SearchWorkView, WorkView,
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

        let dir_path = std::env::current_dir()?;
        let dir_path = dir_path.join(path::Path::new("tamar_content"));
        let dir_path = dir_path.join(path::Path::new(&artist.name));
        let dir_path = dir_path.join(path::Path::new(&work.title));

        let paths = fs::read_dir(dir_path)?;
        let mut image_paths = Vec::new();
        for path in paths {
            image_paths.push(
                path?
                    .path()
                    .to_str()
                    .ok_or(anyhow::anyhow!("can't encode pathbuf -> str"))?
                    .to_string(),
            );
        }

        Ok(WorkView::new(
            work,
            image_paths,
            ArtistView::new(artist),
            tags,
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

    pub async fn search(&self, source: SearchWorkView) -> anyhow::Result<Vec<WorkView>> {
        let works = self
            .repositories
            .work_repository()
            .search(source.try_into()?)
            .await?;
        let mut work_views = Vec::new();
        for work in works.into_iter() {
            work_views.push(self.get_work_view_from_work(work).await?);
        }
        Ok(work_views)
    }
}
