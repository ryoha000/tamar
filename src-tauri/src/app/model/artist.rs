use crate::kernel::model::{
    artist::{Artist, NewArtist, UpdateNameArtist},
    Id,
};
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

pub struct UpdateArtistName {
    pub id: Id<Artist>,
    pub name: String,
}

impl UpdateArtistName {
    pub fn new(id: String, name: String) -> anyhow::Result<Self> {
        let id = Id::<Artist>::new(ulid::Ulid::from_string(&id)?);
        Ok(UpdateArtistName { id, name })
    }
}

impl TryFrom<UpdateArtistName> for UpdateNameArtist {
    type Error = anyhow::Error;

    fn try_from(c: UpdateArtistName) -> anyhow::Result<Self> {
        Ok(UpdateNameArtist::new(c.id, c.name))
    }
}

#[derive(new)]
pub struct GetByNameArtist {
    pub name: String,
}
