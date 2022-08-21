use serde::{Deserialize, Serialize};

use crate::app::model::tag_view::TagView;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonTagView {
    id: String,
    name: String,
    updated_at: String,
}

impl From<TagView> for JsonTagView {
    fn from(s: TagView) -> Self {
        JsonTagView {
            id: s.id,
            name: s.name,
            updated_at: s.updated_at,
        }
    }
}
