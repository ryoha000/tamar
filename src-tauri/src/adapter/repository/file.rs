use std::{
    fs,
    path::{self, Path},
};

use crate::kernel::{
    model::{
        file::{CopyFiles, File, SaveWorkFiles},
        work::Work,
        Id,
    },
    repository::file::FileRepository,
};
use async_trait::async_trait;

use super::RepositoryImpl;

#[async_trait]
impl FileRepository for RepositoryImpl<File> {
    fn get_data_root_dir_path(&self) -> &str {
        // TODO: `${appDir}/data` とかにする
        "../tamar_content"
    }

    fn get_file_name(&self, path_str: &str) -> anyhow::Result<String> {
        let path = Path::new(path_str);
        let name = path
            .file_name()
            .ok_or(anyhow::anyhow!("osstr unicode is invalid"))?
            .to_str()
            .ok_or(anyhow::anyhow!("failed osstr -> str"))?;
        Ok(name.to_string())
    }

    fn get_work_paths(&self, id: &Id<Work>) -> anyhow::Result<Vec<String>> {
        let dir_path = path::Path::new(self.get_data_root_dir_path());
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

    fn save_work_files(&self, source: SaveWorkFiles) -> anyhow::Result<()> {
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

    fn copy_files(&self, source: CopyFiles) -> anyhow::Result<()> {
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

    fn delete_work_files(&self, id: &Id<Work>) -> anyhow::Result<()> {
        let dir_path = path::Path::new(self.get_data_root_dir_path());
        let dir_path = dir_path.join(path::Path::new(&id.value.to_string()));

        fs::remove_dir_all(dir_path)?;
        Ok(())
    }

    fn delete_file(&self, file: String) -> anyhow::Result<()> {
        let file_path = path::Path::new(&file);

        fs::remove_file(file_path)?;
        Ok(())
    }

    fn rotate_90_image_file(&self, file: String) -> anyhow::Result<()> {
        let img = image::open(file.clone())?;
        let img = img.rotate90();
        img.save(file)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {}
