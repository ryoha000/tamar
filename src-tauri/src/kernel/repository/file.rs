use crate::kernel::model::{
    file::CopyFiles,
    file::{ResizeImage, SaveThumbnail, SaveWorkFiles},
    work::Work,
    Id,
};
use async_trait::async_trait;
use sqlx::types::chrono::NaiveDateTime;

#[async_trait]
pub trait FileRepository {
    fn get_data_root_dir_path(&self) -> &str;
    fn get_thumbnail_root_dir_path(&self) -> &str;
    fn get_file_name(&self, path_str: &str) -> anyhow::Result<String>;
    fn extract_zip_file(&self, file_path_str: &str, dir_path_str: &str) -> anyhow::Result<()>;
    fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>>;
    fn get_work_dir_path(&self, id: &Id<Work>) -> anyhow::Result<String>;
    fn save_work_files(&self, source: SaveWorkFiles) -> anyhow::Result<()>;
    fn copy_files(&self, source: CopyFiles) -> anyhow::Result<()>;
    fn delete_work_files(&self, id: &Id<Work>) -> anyhow::Result<()>;
    fn delete_file(&self, file: String) -> anyhow::Result<()>;
    fn delete_dir(&self, dir: String) -> anyhow::Result<()>;
    fn rotate_90_image_file(&self, file: String) -> anyhow::Result<()>;
    fn get_modified_at(&self, file: String) -> anyhow::Result<NaiveDateTime>;
    fn resize_image(&self, source: ResizeImage) -> anyhow::Result<()>;
    fn get_work_list_thumbnail(&self, id: &Id<Work>) -> anyhow::Result<String>;
    fn get_artist_list_thumbnail(&self, id: &Id<Work>) -> anyhow::Result<String>;
    fn save_thumbnail(&self, source: SaveThumbnail) -> anyhow::Result<()>;
}
