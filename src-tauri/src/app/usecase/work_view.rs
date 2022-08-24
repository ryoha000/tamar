use std::path::Path;
use std::sync::Arc;
use std::{fs, path};

use crate::app::model::artist_view::ArtistView;
use crate::app::model::tag_view::TagView;
use crate::kernel::model::artist::Artist;
use crate::kernel::model::work::Work;
use crate::kernel::model::Id;
use crate::kernel::repository::tag::TagRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use crate::{
    adapter::modules::RepositoriesModuleExt, kernel::repository::artist::ArtistRepository,
};
use derive_new::new;

use crate::app::model::work_view::{
    CopyFiles, GetWorkView, SaveWorkFiles, SearchWorkView, SelectByArtistView, WorkView,
};

#[derive(new)]
pub struct WorkViewUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> WorkViewUseCase<R> {
    pub fn get_data_root_dir_path(&self) -> &str {
        // TODO: `${appDir}/data` とかにする
        "../tamar_content"
    }

    pub fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>> {
        let dir_path = Path::new(self.get_data_root_dir_path());
        let dir_path = dir_path.join(path::Path::new(&id.value.to_string()));

        let paths = fs::read_dir(dir_path)?;
        let mut image_paths = Vec::new();
        for path in paths {
            image_paths.push(
                fs::canonicalize(path?.path())?
                    .to_str()
                    .ok_or(anyhow::anyhow!("can't encode pathbuf -> str"))?
                    .to_string(),
            );
        }
        Ok(image_paths)
    }

    pub fn save_work_files(&self, source: SaveWorkFiles) -> anyhow::Result<()> {
        let copy_root_dir = self.get_data_root_dir_path();

        let dir_path = path::Path::new(copy_root_dir);
        let dst_work_dir_path_buf = dir_path.join(path::Path::new(&source.id.value.to_string()));
        let dst_work_dir_path = dst_work_dir_path_buf.as_path();
        // コピー先のディレクトリをつくる
        fs::create_dir_all(dst_work_dir_path)?;

        let src_work_dir_path = path::Path::new(&source.src_path);

        // TODO: 全然並列じゃない
        self.copy_files(CopyFiles::new(dst_work_dir_path, src_work_dir_path, vec![]))?;
        Ok(())
    }

    pub fn copy_files(&self, source: CopyFiles) -> anyhow::Result<()> {
        let children = fs::read_dir(source.dir_path)?;
        for child in children {
            let child = child?;

            let is_dir_child = child.file_type()?.is_dir();

            let child_path_buf = child.path();
            let child_path = child_path_buf.as_path();

            let child_name = child_path
                .file_name()
                .ok_or(anyhow::anyhow!("failed to get file_name"))?
                .to_str()
                .ok_or(anyhow::anyhow!("failed to get &str"))?
                .to_string();

            if is_dir_child {
                let mut new_dirs = source.dirs.to_vec();
                new_dirs.push(child_name);
                self.copy_files(CopyFiles::new(
                    source.dst_work_dir_path,
                    child_path,
                    new_dirs,
                ))?;
            } else {
                // child が ファイルの時は callback
                let dst_filename;
                match source.dirs.len() {
                    0 => dst_filename = child_name,
                    _ => dst_filename = format!("{}-{}", source.dirs.join("-"), child_name),
                }

                let dst_path_buf = source
                    .dst_work_dir_path
                    .join(path::Path::new(&dst_filename));
                let dst_path = dst_path_buf.as_path();
                fs::copy(child_path, dst_path)?;
            }
        }
        Ok(())
    }

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

        let image_paths = self.get_work_paths(&work.id)?;
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

    pub async fn select_by_artist(
        &self,
        source: SelectByArtistView,
    ) -> anyhow::Result<Vec<WorkView>> {
        let id = Id::<Artist>::new(ulid::Ulid::from_string(&source.id)?);

        let works = self
            .repositories
            .work_repository()
            .find_by_artist(&id)
            .await?;
        let mut work_views = Vec::new();
        for work in works.into_iter() {
            work_views.push(self.get_work_view_from_work(work).await?);
        }
        Ok(work_views)
    }
}
