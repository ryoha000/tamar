use serde::{Deserialize, Serialize};

use crate::app::model::work_view::WorkView;

use super::{artist_view::JsonArtistView, tag_view::JsonTagView};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonWorkView {
    pub id: String,
    pub title: String,
    pub paths: Vec<String>, // TODO: 最初の一つでよさそう(一覧のとき別構造体？)
    pub artist: JsonArtistView,
    pub tags: Vec<JsonTagView>,
    pub updated_at: String,
}

impl From<WorkView> for JsonWorkView {
    fn from(s: WorkView) -> Self {
        JsonWorkView {
            id: s.id,
            title: s.title,
            paths: s.paths,
            artist: JsonArtistView::from(s.artist),
            tags: s
                .tags
                .into_iter()
                .map(|tag| JsonTagView::from(tag))
                .collect(),
            updated_at: s.updated_at,
        }
    }
}
