use crate::kernel::model::work::Work;

use super::{artist_view::ArtistView, tag_view::TagView};

pub struct WorkView {
    pub id: String,
    pub title: String,
    pub dir_path: String,
    pub artist: ArtistView,
    pub tags: Vec<TagView>,
    pub updated_at: String,
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
