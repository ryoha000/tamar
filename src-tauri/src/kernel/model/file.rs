use derive_new::new;
use std::path;

use crate::kernel::model::{work::Work, Id};

pub struct File {}

#[derive(new)]
pub struct SaveWorkFiles {
    pub id: Id<Work>,
    pub src_path: String,
}

#[derive(new)]
pub struct SaveThumbnail {
    pub id: Id<Work>,
    pub src_path: String,
}

#[derive(new)]
pub struct CopyFiles<'a> {
    pub dst_work_dir_path: &'a path::Path,
    pub dir_path: &'a path::Path,
    pub dirs: Vec<String>,
}

#[derive(new)]
pub struct ResizeImages {
    pub src_file: String,
    pub dst: Vec<ResizeImageDst>,
}

#[derive(new)]
pub struct ResizeImageDst {
    pub dst_file: String,
    pub dst_width: u32,
}
