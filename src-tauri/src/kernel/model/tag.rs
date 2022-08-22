use super::Id;
use chrono::{DateTime, Utc};
use derive_new::new;

#[derive(new, Debug, Clone)]
pub struct Tag {
    pub id: Id<Tag>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(new, Debug)]
pub struct NewTag {
    pub id: Id<Tag>,
    pub name: String,
}
