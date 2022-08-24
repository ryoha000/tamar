use crate::kernel::model::{
    artist::Artist,
    work::{
        NewWork, NewerArtistIdWork, NewerTitleWork, SearchAroundTitleWork,
        SearchAroundUpdatedAtWork,
    },
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

impl TryFrom<UpdateTitleWork> for NewerTitleWork {
    type Error = anyhow::Error;

    fn try_from(c: UpdateTitleWork) -> anyhow::Result<Self> {
        let work_id = ulid::Ulid::from_string(&c.id)?;
        Ok(NewerTitleWork::new(Id::new(work_id), c.title))
    }
}

#[derive(new)]
pub struct UpdateArtistIdWork {
    pub id: String,
    pub artist_id: Id<Artist>,
}

impl TryFrom<UpdateArtistIdWork> for NewerArtistIdWork {
    type Error = anyhow::Error;

    fn try_from(c: UpdateArtistIdWork) -> anyhow::Result<Self> {
        let work_id = ulid::Ulid::from_string(&c.id)?;
        Ok(NewerArtistIdWork::new(Id::new(work_id), c.artist_id))
    }
}

#[derive(new)]
pub struct GetByTitleWork {
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
