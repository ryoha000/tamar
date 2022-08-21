use serde::{Deserialize, Serialize};

use crate::app::model::artist_view::ArtistView;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonArtistView {
    id: String,
    name: String,
    updated_at: String,
}

impl From<ArtistView> for JsonArtistView {
    fn from(s: ArtistView) -> Self {
        JsonArtistView {
            id: s.id,
            name: s.name,
            updated_at: s.updated_at,
        }
    }
}
