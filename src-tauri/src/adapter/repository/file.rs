use std::{
    fs,
    path::{self, Path},
};

use crate::kernel::{
    model::{
        file::{CopyFiles, File, ResizeImage, SaveThumbnail, SaveWorkFiles},
        work::Work,
        Id,
    },
    repository::file::FileRepository,
};
use async_trait::async_trait;
use sqlx::types::chrono::{DateTime, NaiveDateTime, Utc};

use std::io::BufWriter;
use std::num::NonZeroU32;

use image::codecs::png::PngEncoder;
use image::io::Reader as ImageReader;
use image::{ColorType, ImageEncoder};

use fast_image_resize as fr;

use super::RepositoryImpl;

const IMAGE_EXTENTION: [&'static str; 7] = ["gif", "jpg", "jpeg", "jpe", "jfif", "png", "webp"];

#[async_trait]
impl FileRepository for RepositoryImpl<File> {
    fn get_data_root_dir_path(&self) -> &str {
        // TODO: `${appDir}/data` とかにする
        "../tamar_content"
    }

    fn get_thumbnail_root_dir_path(&self) -> &str {
        // TODO: `${appDir}/data` とかにする
        "../tamar_content/thumbnail"
    }

    fn get_work_dir_path(&self, id: &Id<Work>) -> anyhow::Result<String> {
        let root_dir = self.get_data_root_dir_path();

        let dir_path = Path::new(root_dir);
        let work_dir_path_buf = dir_path.join(Path::new(&id.value.to_string()));
        let work_dir_path = work_dir_path_buf
            .as_path()
            .to_str()
            .ok_or(anyhow::anyhow!("failed osstr -> str"))?;
        Ok(work_dir_path.to_string())
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

    fn extract_zip_file(&self, file_path_str: &str, dir_path_str: &str) -> anyhow::Result<()> {
        let fname = Path::new(file_path_str);
        let file = fs::File::open(&fname)?;
        let dir_path = Path::new(dir_path_str);

        let mut archive = zip::ZipArchive::new(file)?;
        fs::create_dir_all(dir_path)?;
        archive.extract(dir_path)?;
        Ok(())
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

    fn save_thumbnail(&self, source: SaveThumbnail) -> anyhow::Result<()> {
        let dir_path = path::Path::new(self.get_thumbnail_root_dir_path());
        let dst_work_dir_path_buf = dir_path.join(path::Path::new(&source.id.value.to_string()));
        let dst_work_dir_path = dst_work_dir_path_buf.as_path();
        // コピー先のディレクトリをつくる
        fs::create_dir_all(dst_work_dir_path)?;

        let src_files = fs::read_dir(&source.src_path)?;
        let mut file_names = vec![];
        for entry in src_files {
            file_names.push(
                entry?
                    .file_name()
                    .to_str()
                    .ok_or(anyhow::anyhow!("can't encode osstr -> str"))?
                    .to_string(),
            );
        }
        file_names.sort_by_key(|v| v.to_lowercase());
        if file_names.len() == 0 {
            return Ok(()); // TODO: エラーにしなくていいか考える
        }

        let mut fname: Option<&str> = None;
        {
            for v in file_names.iter() {
                let mut is_target_extension = false;
                for ext in IMAGE_EXTENTION {
                    if v.ends_with(ext) {
                        is_target_extension = true;
                    }
                }
                if is_target_extension {
                    fname = Some(v);
                    break;
                }
            }
        }
        if let None = fname {
            return Ok(()); // TODO: エラーにしなくていいか考える
        }
        let fname = fname.unwrap();

        let artist_list_thumbnail_path = self.get_artist_list_thumbnail(&source.id)?;
        let work_list_thumbnail_path = self.get_work_list_thumbnail(&source.id)?;

        let src_work_dir_path = path::Path::new(&source.src_path);
        let thumbnail_original_path = src_work_dir_path
            .join(path::Path::new(fname))
            .as_path()
            .to_str()
            .ok_or(anyhow::anyhow!("can't encode osstr -> str"))?
            .to_string();

        self.resize_image(ResizeImage {
            dst_width: 160,
            dst_file: artist_list_thumbnail_path,
            src_file: thumbnail_original_path.clone(),
        })?;

        self.resize_image(ResizeImage {
            dst_width: 400,
            dst_file: work_list_thumbnail_path,
            src_file: thumbnail_original_path.clone(),
        })?;

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

    fn delete_dir(&self, dir: String) -> anyhow::Result<()> {
        let dir_path = path::Path::new(&dir);

        fs::remove_dir_all(dir_path)?;
        Ok(())
    }

    fn rotate_90_image_file(&self, file: String) -> anyhow::Result<()> {
        let img = image::open(file.clone())?;
        let img = img.rotate90();
        img.save(file)?;
        Ok(())
    }

    fn get_modified_at(&self, file: String) -> anyhow::Result<NaiveDateTime> {
        let metadata = fs::metadata(&file)?;
        let time = metadata.modified()?;
        let dt: DateTime<Utc> = time.into();
        Ok(dt.naive_utc())
    }

    fn resize_image(&self, source: ResizeImage) -> anyhow::Result<()> {
        // Read source image from file
        let img = ImageReader::open(source.src_file)?.decode()?;
        let width =
            NonZeroU32::new(img.width()).ok_or(anyhow::anyhow!("failed NonZeroU32::new"))?;
        let height =
            NonZeroU32::new(img.height()).ok_or(anyhow::anyhow!("failed NonZeroU32::new"))?;
        let mut src_image = fr::Image::from_vec_u8(
            width,
            height,
            img.to_rgba8().into_raw(),
            fr::PixelType::U8x4,
        )?;

        // Multiple RGB channels of source image by alpha channel
        // (not required for the Nearest algorithm)
        let alpha_mul_div = fr::MulDiv::default();
        alpha_mul_div.multiply_alpha_inplace(&mut src_image.view_mut())?;

        // Create container for data of destination image
        let dst_width =
            NonZeroU32::new(source.dst_width).ok_or(anyhow::anyhow!("failed NonZeroU32::new"))?;
        let dst_height = NonZeroU32::new(
            (height.get() as f32 / width.get() as f32 * source.dst_width as f32) as u32,
        )
        .ok_or(anyhow::anyhow!("failed NonZeroU32::new"))?;

        let mut dst_image = fr::Image::new(dst_width, dst_height, src_image.pixel_type());

        // Get mutable view of destination image data
        let mut dst_view = dst_image.view_mut();

        // Create Resizer instance and resize source image
        // into buffer of destination image
        let mut resizer = fr::Resizer::new(fr::ResizeAlg::Convolution(fr::FilterType::Lanczos3));
        resizer.resize(&src_image.view(), &mut dst_view)?;

        // Divide RGB channels of destination image by alpha
        alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

        let mut result_buf = BufWriter::new(fs::File::create(&source.dst_file)?);

        // Write destination image as PNG-file
        PngEncoder::new(&mut result_buf).write_image(
            dst_image.buffer(),
            dst_width.get(),
            dst_height.get(),
            ColorType::Rgba8,
        )?;

        Ok(())
    }

    fn get_work_list_thumbnail(&self, id: &Id<Work>) -> anyhow::Result<String> {
        let dir_path = Path::new(self.get_thumbnail_root_dir_path());
        let work_dir_path_buf = dir_path.join(Path::new(&id.value.to_string()));
        let work_dir_path_buf = work_dir_path_buf.join(Path::new("work_list.png"));
        let work_dir_path = work_dir_path_buf
            .as_path()
            .to_str()
            .ok_or(anyhow::anyhow!("failed osstr -> str"))?;
        Ok(work_dir_path.to_string())
    }

    fn get_artist_list_thumbnail(&self, id: &Id<Work>) -> anyhow::Result<String> {
        let work_dir_path_buf = Path::new(self.get_thumbnail_root_dir_path());
        let work_dir_path_buf = work_dir_path_buf.join(Path::new(&id.value.to_string()));
        let work_dir_path_buf = work_dir_path_buf.join(Path::new("artist_list.png"));
        let work_dir_path = work_dir_path_buf
            .as_path()
            .to_str()
            .ok_or(anyhow::anyhow!("failed osstr -> str"))?;
        Ok(work_dir_path.to_string())
    }
}

#[cfg(test)]
mod test {}
