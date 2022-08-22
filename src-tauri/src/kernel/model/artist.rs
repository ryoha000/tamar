use super::Id;
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(new, Debug, Clone)]
pub struct Artist {
    pub id: Id<Artist>,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewArtist {
    pub id: Id<Artist>,
    pub name: String,
}
