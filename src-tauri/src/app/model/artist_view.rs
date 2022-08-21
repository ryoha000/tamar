use crate::kernel::model::artist::Artist;

pub struct ArtistView {
    pub id: String,
    pub name: String,
    pub updated_at: String,
}

impl ArtistView {
    pub fn new(artist: Artist) -> Self {
        Self {
            id: artist.id.value.to_string(),
            name: artist.name,
            updated_at: artist.updated_at.to_string(),
        }
    }
}
