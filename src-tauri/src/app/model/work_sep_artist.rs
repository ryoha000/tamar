use derive_new::new;

use crate::kernel::model::{artist::Artist, Id};

use super::artist_view::ArtistView;

pub struct WorkSepArtistView {
    pub artist_id: String,
    pub artist: ArtistView,
    pub works: Vec<WorkSepArtistWorkView>,
}

#[derive(new)]
pub struct WorkSepArtistWorkView {
    pub id: String,
    pub title: String,
    pub artist_id: String,
    pub sample_path: String,
    pub updated_at: String,
}

impl WorkSepArtistView {
    pub fn new(
        artist_id: Id<Artist>,
        artist: ArtistView,
        works: Vec<WorkSepArtistWorkView>,
    ) -> Self {
        Self {
            artist_id: artist_id.value.to_string(),
            artist,
            works,
        }
    }
}
