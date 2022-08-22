use crate::kernel::model::{
    artist::Artist,
    work::{NewWork, SearchAroundTitleWork, SearchAroundUpdatedAtWork},
    Id,
};
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(new)]
pub struct CreateWork {
    pub title: String,
    pub artist_id: Id<Artist>,
}

impl TryFrom<CreateWork> for NewWork {
    type Error = anyhow::Error;

    fn try_from(c: CreateWork) -> anyhow::Result<Self> {
        let work_id = Id::gen();
        Ok(NewWork::new(work_id, c.title, c.artist_id))
    }
}

#[derive(new)]
pub struct SearchEqualWork {
    pub title: String,
    pub artist_id: Id<Artist>,
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

#[derive(new)]
pub struct SearchAroundUpdatedAtWorkView {
    pub limit: u8,
    pub is_before: bool,
    pub updated_at: String,
}

impl TryFrom<SearchAroundUpdatedAtWorkView> for SearchAroundUpdatedAtWork {
    type Error = anyhow::Error;

    fn try_from(c: SearchAroundUpdatedAtWorkView) -> anyhow::Result<Self> {
        Ok(SearchAroundUpdatedAtWork::new(
            c.limit,
            c.is_before,
            NaiveDateTime::parse_from_str("2018/12/07 19:31:28.55", "%Y/%m/%d %H:%M:%S%.f")?,
        ))
    }
}
