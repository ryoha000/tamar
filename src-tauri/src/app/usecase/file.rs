use std::{
    fs,
    path::{self, Path},
    sync::Arc,
};

use crate::{
    adapter::modules::RepositoriesModuleExt,
    kernel::model::{work::Work, Id},
};
use derive_new::new;

#[derive(new)]
pub struct FileUseCase<R: RepositoriesModuleExt> {
    repositories: Arc<R>,
}

impl<R: RepositoriesModuleExt> FileUseCase<R> {
    pub fn get_data_root_dir_path(&self) -> &str {
        // TODO: `${appDir}/data` とかにする
        "../tamar_content"
    }
    pub async fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>> {
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
}
