use super::{artist::Artist, Id};
use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug)]
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
