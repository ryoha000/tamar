use std::path;

use derive_new::new;

use crate::kernel::model::{work::Work, Id};

#[derive(new)]
pub struct SaveWorkFiles {
    pub id: Id<Work>,
    pub src_path: String,
}

#[derive(new)]
pub struct CopyFiles<'a> {
    pub dst_work_dir_path: &'a path::Path,
    pub dir_path: &'a path::Path,
    pub dirs: Vec<String>,
}
