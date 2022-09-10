use super::Id;
use derive_new::new;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub enum SearchHistoryTypeEnum {
    artist = 0,
    tag = 1,
}

#[derive(new, Debug, Clone)]
pub struct SearchHistory {
    pub id: Id<SearchHistory>,
    pub value_id: String,
    pub value_type: SearchHistoryTypeEnum,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(new, Debug)]
pub struct NewSearchHistory {
    pub id: Id<SearchHistory>,
    pub value_id: String,
    pub value_type: SearchHistoryTypeEnum,
}
