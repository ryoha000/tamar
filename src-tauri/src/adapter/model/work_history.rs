use crate::kernel::model::work_history::{NewWorkHistory, WorkHistory};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct WorkHistoryTable {
    pub id: String,
    pub work_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<WorkHistoryTable> for WorkHistory {
    type Error = anyhow::Error;
    fn try_from(st: WorkHistoryTable) -> Result<Self, Self::Error> {
        Ok(WorkHistory::new(
            st.id.try_into()?,
            st.work_id.try_into()?,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewWorkHistory> for WorkHistoryTable {
    type Error = anyhow::Error;
    fn try_from(s: NewWorkHistory) -> Result<Self, Self::Error> {
        Ok(WorkHistoryTable {
            id: s.id.value.to_string(),
            work_id: s.work_id.value.to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}
