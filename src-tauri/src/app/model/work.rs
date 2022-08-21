use crate::kernel::model::{artist::Artist, work::NewWork, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateWork {
    pub title: String,
    pub artist_id: Id<Artist>,
}

impl TryFrom<CreateWork> for NewWork {
    type Error = anyhow::Error;

    fn try_from(c: CreateWork) -> anyhow::Result<Self> {
        let work_id = Id::gen();
        Ok(NewWork::new(work_id, c.title, c.artist_id))
    }
}

#[derive(new)]
pub struct SearchWork {
    pub title: String,
    pub artist_id: Id<Artist>,
}
