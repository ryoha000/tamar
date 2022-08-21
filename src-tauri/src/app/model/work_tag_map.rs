use crate::kernel::model::{tag::Tag, work::Work, work_tag_map::NewWorkTagMap, Id};
use derive_new::new;

#[derive(new)]
pub struct CreateWorkTagMap {
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
}

impl TryFrom<CreateWorkTagMap> for NewWorkTagMap {
    type Error = anyhow::Error;

    fn try_from(c: CreateWorkTagMap) -> anyhow::Result<Self> {
        let work_tag_map_id = Id::gen();
        Ok(NewWorkTagMap::new(work_tag_map_id, c.work_id, c.tag_id))
    }
}
