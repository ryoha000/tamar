use crate::kernel::model::{artist::NewArtist, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateArtist {
    pub name: String,
}

impl TryFrom<CreateArtist> for NewArtist {
    type Error = anyhow::Error;

    fn try_from(c: CreateArtist) -> anyhow::Result<Self> {
        let artist_id = Id::gen();
        Ok(NewArtist::new(artist_id, c.name))
    }
}

#[derive(new)]
pub struct GetByNameArtist {
    pub name: String,
}
