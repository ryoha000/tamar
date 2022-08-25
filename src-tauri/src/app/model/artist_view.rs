use derive_new::new;

use crate::kernel::model::artist::{Artist, SearchAlsoUsingWorkArtist};

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

#[derive(new)]
pub struct SearchArtistView {
    pub limit: u16,
    pub offset: u16,
    pub sort_col: String,
    pub sort_desc: bool,
    pub text: String,
}

impl TryFrom<SearchArtistView> for SearchAlsoUsingWorkArtist {
    type Error = anyhow::Error;

    fn try_from(c: SearchArtistView) -> anyhow::Result<Self> {
        Ok(SearchAlsoUsingWorkArtist::new(
            c.limit,
            c.offset,
            c.sort_col,
            c.sort_desc,
            c.text,
        ))
    }
}

#[derive(new)]
pub struct GetArtistView {
    pub id: String,
}

#[derive(new)]
pub struct SearchByNameArtistView<'a> {
    pub name: &'a str,
}
