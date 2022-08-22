use super::{tag::Tag, work::Work, Id};
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(new, Debug)]
pub struct WorkTagMap {
    pub id: Id<WorkTagMap>,
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewWorkTagMap {
    pub id: Id<WorkTagMap>,
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
}
