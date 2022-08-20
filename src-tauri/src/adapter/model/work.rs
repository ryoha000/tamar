use crate::kernel::model::work::{NewWork, Work};
use chrono::{DateTime, Local};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct WorkTable {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

impl TryFrom<WorkTable> for Work {
    type Error = anyhow::Error;
    fn try_from(st: WorkTable) -> Result<Self, Self::Error> {
        Ok(Work::new(
            st.id.try_into()?,
            st.title,
            st.artist_id.try_into()?,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewWork> for WorkTable {
    type Error = anyhow::Error;
    fn try_from(s: NewWork) -> Result<Self, Self::Error> {
        Ok(WorkTable {
            id: s.id.value.to_string(),
            title: s.title,
            artist_id: s.artist_id.value.to_string(),
            created_at: Local::now(),
            updated_at: Local::now(),
        })
    }
}
