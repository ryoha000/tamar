use crate::kernel::model::work_tag_map::{NewWorkTagMap, WorkTagMap};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct WorkTagMapTable {
    pub id: String,
    pub work_id: String,
    pub tag_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<WorkTagMapTable> for WorkTagMap {
    type Error = anyhow::Error;
    fn try_from(st: WorkTagMapTable) -> Result<Self, Self::Error> {
        Ok(WorkTagMap::new(
            st.id.try_into()?,
            st.work_id.try_into()?,
            st.tag_id.try_into()?,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewWorkTagMap> for WorkTagMapTable {
    type Error = anyhow::Error;
    fn try_from(s: NewWorkTagMap) -> Result<Self, Self::Error> {
        Ok(WorkTagMapTable {
            id: s.id.value.to_string(),
            work_id: s.work_id.value.to_string(),
            tag_id: s.tag_id.value.to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}
