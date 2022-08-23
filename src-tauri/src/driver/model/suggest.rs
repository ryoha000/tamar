use serde::{Deserialize, Serialize};

use crate::app::model::{artist_view::ArtistView, tag_view::TagView};

use super::{artist_view::JsonArtistView, tag_view::JsonTagView};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonSuggest {
    artists: Vec<JsonArtistView>,
    tags: Vec<JsonTagView>,
}

impl JsonSuggest {
    pub fn new(artists: Vec<ArtistView>, tags: Vec<TagView>) -> Self {
        Self {
            artists: artists
                .into_iter()
                .map(|v| JsonArtistView::from(v))
                .collect(),
            tags: tags.into_iter().map(|v| JsonTagView::from(v)).collect(),
        }
    }
}
