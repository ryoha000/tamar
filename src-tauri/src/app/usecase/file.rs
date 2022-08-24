use derive_new::new;
use std::sync::Arc;

use crate::{
    adapter::modules::RepositoriesModuleExt,
    app::model::file::SaveOriginalFiles,
    kernel::model::{work::Work, Id},
    kernel::repository::file::FileRepository,
};

#[derive(new)]
pub struct FileUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> FileUseCase<R> {
    pub fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>> {
        self.repositories.file_repository().get_work_paths(id)
    }
    pub fn save_original_files(&self, source: SaveOriginalFiles) -> anyhow::Result<()> {
        self.repositories.file_repository().save_work_files(source)
    }
    pub fn rotate_image_file(&self, file: String) -> anyhow::Result<()> {
        self.repositories
            .file_repository()
            .rotate_90_image_file(file)
    }
}
