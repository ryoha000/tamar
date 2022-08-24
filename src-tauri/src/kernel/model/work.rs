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
pub struct NewerWork {
    pub id: Id<Work>,
    pub title: String,
}

#[derive(new, Debug)]
pub struct SearchWork {
    pub limit: u8,
    pub offset: u8,
    pub sort_col: String,
    pub sort_desc: bool,
    pub text: String,
}

#[derive(new, Debug)]
pub struct SearchAroundTitleWork {
    pub limit: u8,
    pub is_before: bool,
    pub title: String,
}

#[derive(new, Debug)]
pub struct SearchAroundUpdatedAtWork {
    pub limit: u8,
    pub is_before: bool,
    pub updated_at: NaiveDateTime,
}
