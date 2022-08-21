use super::{artist::Artist, Id};
use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct Work {
    pub id: Id<Work>,
    pub title: String,
    pub artist_id: Id<Artist>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewWork {
    pub id: Id<Work>,
    pub title: String,
    pub artist_id: Id<Artist>,
}

#[derive(new, Debug)]
pub struct SearchWork {
    pub limit: u8,
    pub offset: u8,
    pub sort_col: String,
    pub sort_desc: bool,
    pub title: String,
}
