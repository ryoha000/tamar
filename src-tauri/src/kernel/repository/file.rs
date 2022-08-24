use crate::kernel::model::{file::CopyFiles, file::SaveWorkFiles, work::Work, Id};
use async_trait::async_trait;

#[async_trait]
pub trait FileRepository {
    fn get_data_root_dir_path(&self) -> &str;
    fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>>;
    fn save_work_files(&self, source: SaveWorkFiles) -> anyhow::Result<()>;
    fn copy_files(&self, source: CopyFiles) -> anyhow::Result<()>;
    fn delete_work_files(&self, id: &Id<Work>) -> anyhow::Result<()>;
    fn rotate_90_image_file(&self, file: String) -> anyhow::Result<()>;
}
