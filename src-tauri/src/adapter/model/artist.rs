use crate::kernel::model::artist::{Artist, NewArtist};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct ArtistTable {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl TryFrom<ArtistTable> for Artist {
    type Error = anyhow::Error;
    fn try_from(st: ArtistTable) -> Result<Self, Self::Error> {
        Ok(Artist::new(
            st.id.try_into()?,
            st.name,
            st.created_at,
            st.updated_at,
        ))
    }
}

impl TryFrom<NewArtist> for ArtistTable {
    type Error = anyhow::Error;
    fn try_from(s: NewArtist) -> Result<Self, Self::Error> {
        Ok(ArtistTable {
            id: s.id.value.to_string(),
            name: s.name,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc()
        })
    }
}
