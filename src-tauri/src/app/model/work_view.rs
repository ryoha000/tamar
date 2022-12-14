use derive_new::new;

use crate::kernel::model::{
    work::{SearchWork, Work},
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

pub struct WorkViewSummary {
    pub id: String,
    pub title: String,
    pub work_list_thumbnail: String,
    pub artist_list_thumbnail: String,
    pub artist: ArtistView,
}

#[derive(new)]
pub struct SearchWorkView {
    pub limit: u16,
    pub offset: u16,
    pub sort_col: String,
    pub sort_desc: bool,
    pub text: String,
    pub work_ids: Vec<Id<Work>>,
}

impl TryFrom<SearchWorkView> for SearchWork {
    type Error = anyhow::Error;

    fn try_from(c: SearchWorkView) -> anyhow::Result<Self> {
        Ok(SearchWork::new(
            c.limit,
            c.offset,
            c.sort_col,
            c.sort_desc,
            c.text,
            c.work_ids,
        ))
    }
}

pub struct GetWorkView {
    pub id: Id<Work>,
}

#[derive(new)]
pub struct SelectByArtistView {
    pub id: String,
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

impl WorkViewSummary {
    pub fn new(
        work: Work,
        work_list_thumbnail: String,
        artist_list_thumbnail: String,
        artist: ArtistView,
    ) -> Self {
        Self {
            id: work.id.value.to_string(),
            title: work.title,
            work_list_thumbnail,
            artist_list_thumbnail,
            artist,
        }
    }
}
