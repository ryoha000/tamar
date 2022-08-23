use serde::{Deserialize, Serialize};

use crate::app::model::work_sep_artist::{WorkSepArtistView, WorkSepArtistWorkView};

use super::artist_view::JsonArtistView;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonWorkSepArtistView {
    pub artist_id: String,
    pub artist: JsonArtistView,
    pub works: Vec<JsonWorkSepArtistWorkView>,
}

impl From<WorkSepArtistView> for JsonWorkSepArtistView {
    fn from(s: WorkSepArtistView) -> Self {
        JsonWorkSepArtistView {
            artist_id: s.artist_id,
            artist: JsonArtistView::from(s.artist),
            works: s
                .works
                .into_iter()
                .map(|v| JsonWorkSepArtistWorkView::from(v))
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonWorkSepArtistWorkView {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub sample_path: String,
    pub updated_at: String,
}

impl From<WorkSepArtistWorkView> for JsonWorkSepArtistWorkView {
    fn from(s: WorkSepArtistWorkView) -> Self {
        JsonWorkSepArtistWorkView {
            id: s.id,
            title: s.title,
            sample_path: s.sample_path,
            artist_id: s.artist_id,
            updated_at: s.updated_at,
        }
    }
}
