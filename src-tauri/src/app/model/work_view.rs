use derive_new::new;

use crate::kernel::model::{
    work::{SearchAroundTitleWork, SearchWork, Work},
    Id,
};

use super::{artist_view::ArtistView, tag_view::TagView};

pub struct WorkView {
    pub id: String,
    pub title: String,
    pub paths: Vec<String>,
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

#[derive(new)]
pub struct SearchAroundTitleWorkView {
    pub limit: u8,
    pub is_before: bool,
    pub title: String,
}

impl TryFrom<SearchAroundTitleWorkView> for SearchAroundTitleWork {
    type Error = anyhow::Error;

    fn try_from(c: SearchAroundTitleWorkView) -> anyhow::Result<Self> {
        Ok(SearchAroundTitleWork::new(c.limit, c.is_before, c.title))
    }
}

pub struct GetWorkView {
    pub id: Id<Work>,
}

impl GetWorkView {
    pub fn new(id: String) -> anyhow::Result<Self> {
        Ok(Self {
            id: Id::<Work>::new(ulid::Ulid::from_string(&id)?),
        })
    }
}

impl WorkView {
    pub fn new(work: Work, paths: Vec<String>, artist: ArtistView, tags: Vec<TagView>) -> Self {
        Self {
            id: work.id.value.to_string(),
            title: work.title,
            paths,
            artist,
            tags,
            updated_at: work.updated_at.to_string(),
        }
    }
}
