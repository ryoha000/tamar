use crate::kernel::model::{
    artist::Artist,
    work::{NewWork, NewerWork, SearchAroundTitleWork, SearchAroundUpdatedAtWork},
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
pub struct UpdateTitleWork {
    pub id: String,
    pub title: String,
}

impl TryFrom<UpdateTitleWork> for NewerWork {
    type Error = anyhow::Error;

    fn try_from(c: UpdateTitleWork) -> anyhow::Result<Self> {
        let work_id = ulid::Ulid::from_string(&c.id)?;
        Ok(NewerWork::new(Id::new(work_id), c.title))
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
        let t = NaiveDateTime::parse_from_str(&c.updated_at, "%Y-%m-%d %H:%M:%S%.f")?;
        Ok(SearchAroundUpdatedAtWork::new(c.limit, c.is_before, t))
    }
}
