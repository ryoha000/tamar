use crate::kernel::model::{
    search_history::{NewSearchHistory, SearchHistoryTypeEnum},
    Id,
};
use derive_new::new;

#[derive(new)]
pub struct CreateSearchHistory {
    pub value_id: String,
    pub value_type: u32,
}

impl TryFrom<CreateSearchHistory> for NewSearchHistory {
    type Error = anyhow::Error;

    fn try_from(c: CreateSearchHistory) -> anyhow::Result<Self> {
        let search_history_id = Id::gen();
        let t;
        match c.value_type {
            0 => t = SearchHistoryTypeEnum::artist,
            1 => t = SearchHistoryTypeEnum::tag,
            _ => {
                return Err(anyhow::anyhow!("invalid value_type: {}", c.value_type));
            }
        };
        Ok(NewSearchHistory::new(search_history_id, c.value_id, t))
    }
}
