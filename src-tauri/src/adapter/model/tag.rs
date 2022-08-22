use crate::kernel::model::tag::{NewTag, Tag};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct TagTable {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<TagTable> for Tag {
    type Error = anyhow::Error;
    fn try_from(st: TagTable) -> Result<Self, Self::Error> {
        Ok(Tag::new(
            st.id.try_into()?,
            st.name,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewTag> for TagTable {
    type Error = anyhow::Error;
    fn try_from(s: NewTag) -> Result<Self, Self::Error> {
        Ok(TagTable {
            id: s.id.value.to_string(),
            name: s.name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
