use crate::kernel::model::search_history::{
    NewSearchHistory, SearchHistory, SearchHistoryTypeEnum,
};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct SearchHistoryTable {
    pub id: String,
    pub value_id: String,
    pub value_type: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<SearchHistoryTable> for SearchHistory {
    type Error = anyhow::Error;
    fn try_from(st: SearchHistoryTable) -> Result<Self, Self::Error> {
        let t;
        match st.value_type {
            0 => t = SearchHistoryTypeEnum::artist,
            1 => t = SearchHistoryTypeEnum::tag,
            _ => {
                return Err(anyhow::anyhow!("invalid value_type: {}", st.value_type));
            }
        };
        Ok(SearchHistory::new(
            st.id.try_into()?,
            st.value_id,
            t,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewSearchHistory> for SearchHistoryTable {
    type Error = anyhow::Error;
    fn try_from(s: NewSearchHistory) -> Result<Self, Self::Error> {
        Ok(SearchHistoryTable {
            id: s.id.value.to_string(),
            value_id: s.value_id,
            value_type: s.value_type as u32,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        })
    }
}
