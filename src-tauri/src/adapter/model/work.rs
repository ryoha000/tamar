use crate::kernel::model::work::{NewImportWork, NewWork, Work};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct WorkTable {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}

impl TryFrom<NewImportWork> for WorkTable {
    type Error = anyhow::Error;
    fn try_from(s: NewImportWork) -> Result<Self, Self::Error> {
        Ok(WorkTable {
            id: s.id.value.to_string(),
            title: s.title,
            artist_id: s.artist_id.value.to_string(),
            created_at: s.created_at,
            updated_at: s.updated_at,
        })
    }
}
