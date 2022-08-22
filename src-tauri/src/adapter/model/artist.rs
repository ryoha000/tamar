use crate::kernel::model::artist::{Artist, NewArtist};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow)]
pub struct ArtistTable {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
}
