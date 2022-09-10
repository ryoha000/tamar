use crate::kernel::model::{work_history::NewWorkHistory, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateWorkHistory {
    pub work_id: String,
}

impl TryFrom<CreateWorkHistory> for NewWorkHistory {
    type Error = anyhow::Error;

    fn try_from(c: CreateWorkHistory) -> anyhow::Result<Self> {
        let search_history_id = Id::gen();
        let work_id = Id::new(ulid::Ulid::from_string(&c.work_id)?);
        Ok(NewWorkHistory::new(search_history_id, work_id))
    }
}
