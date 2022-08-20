use super::Id;
use chrono::{DateTime, Local};
use derive_new::new;

#[derive(new, Debug)]
pub struct Tag {
    pub id: Id<Tag>,
    pub name: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(new, Debug)]
pub struct NewTag {
    pub id: Id<Tag>,
    pub name: String,
}
