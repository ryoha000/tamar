use super::{artist::Artist, Id};
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(new, Debug, Clone)]
pub struct Work {
    pub id: Id<Work>,
    pub title: String,
    pub artist_id: Id<Artist>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewWork {
    pub id: Id<Work>,
    pub title: String,
    pub artist_id: Id<Artist>,
}

#[derive(new, Debug)]
pub struct NewImportWork {
    pub id: Id<Work>,
    pub title: String,
    pub artist_id: Id<Artist>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewerTitleWork {
    pub id: Id<Work>,
    pub title: String,
}

#[derive(new, Debug)]
pub struct NewerArtistIdWork {
    pub id: Id<Work>,
    pub artist_id: Id<Artist>,
}

#[derive(new, Debug)]
pub struct SearchWork {
    pub limit: u16,
    pub offset: u16,
    pub sort_col: String,
    pub sort_desc: bool,
    pub text: String,
}

#[derive(new, Debug)]
pub struct SearchAroundTitleWork {
    pub limit: u16,
    pub is_before: bool,
    pub title: String,
}

#[derive(new, Debug)]
pub struct SearchAroundUpdatedAtWork {
    pub limit: u16,
    pub is_before: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct SearchAroundViewTimeWork {
    pub limit: u16,
    pub is_before: bool,
    pub view_time: NaiveDateTime,
}
