use derive_new::new;

use crate::kernel::model::work::{SearchWork, Work};

use super::{artist_view::ArtistView, tag_view::TagView};

pub struct WorkView {
    pub id: String,
    pub title: String,
    pub dir_path: String,
    pub artist: ArtistView,
    pub tags: Vec<TagView>,
    pub updated_at: String,
}

#[derive(new)]
pub struct SearchWorkView {
    pub limit: u8,
    pub offset: u8,
    pub sort_col: String,
    pub sort_desc: bool,
    pub title: String,
}

impl TryFrom<SearchWorkView> for SearchWork {
    type Error = anyhow::Error;

    fn try_from(c: SearchWorkView) -> anyhow::Result<Self> {
        Ok(SearchWork::new(
            c.limit,
            c.offset,
            c.sort_col,
            c.sort_desc,
            c.title,
        ))
    }
}

impl WorkView {
    pub fn new(work: Work, dir_path: String, artist: ArtistView, tags: Vec<TagView>) -> Self {
        Self {
            id: work.id.value.to_string(),
            title: work.title,
            dir_path,
            artist,
            tags,
            updated_at: work.updated_at.to_string(),
        }
    }
}
