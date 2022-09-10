use super::{work::Work, Id};
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(new, Debug, Clone)]
pub struct WorkHistory {
    pub id: Id<WorkHistory>,
    pub work_id: Id<Work>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewWorkHistory {
    pub id: Id<WorkHistory>,
    pub work_id: Id<Work>,
}
