use super::Id;
use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug)]
pub struct Artist {
    pub id: Id<Artist>,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewArtist {
    pub id: Id<Artist>,
    pub name: String,
}
