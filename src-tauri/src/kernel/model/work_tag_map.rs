use super::{tag::Tag, work::Work, Id};
use chrono::{DateTime, Utc};
use derive_new::new;

#[derive(new, Debug)]
pub struct WorkTagMap {
    pub id: Id<WorkTagMap>,
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(new, Debug)]
pub struct NewWorkTagMap {
    pub id: Id<WorkTagMap>,
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
}
