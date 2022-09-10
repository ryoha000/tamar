use derive_new::new;
use ulid::Ulid;

use crate::kernel::model::{
    tag::Tag,
    work::Work,
    work_tag_map::{DeleteWorkTagMap, NewWorkTagMap},
    Id,
};

#[derive(new)]
pub struct CreateWorkTagMap {
    pub work_id: Id<Work>,
    pub tag_id: Id<Tag>,
}

impl CreateWorkTagMap {
    pub fn from_raw(work_id: String, tag_id: String) -> anyhow::Result<Self> {
        let w = Id::<Work>::new(ulid::Ulid::from_string(&work_id)?);
        let t = Id::<Tag>::new(ulid::Ulid::from_string(&tag_id)?);
        Ok(Self {
            work_id: w,
            tag_id: t,
        })
    }
}

impl TryFrom<CreateWorkTagMap> for NewWorkTagMap {
    type Error = anyhow::Error;

    fn try_from(c: CreateWorkTagMap) -> anyhow::Result<Self> {
        let work_tag_map_id = Id::gen();
        Ok(NewWorkTagMap::new(work_tag_map_id, c.work_id, c.tag_id))
    }
}

pub type DetachWorkTagMap = CreateWorkTagMap;

impl TryFrom<DetachWorkTagMap> for DeleteWorkTagMap {
    type Error = anyhow::Error;

    fn try_from(c: DetachWorkTagMap) -> anyhow::Result<Self> {
        Ok(DeleteWorkTagMap::new(c.work_id, c.tag_id))
    }
}

pub struct GetWorkAttachedTags {
    pub tag_ids: Vec<Id<Tag>>,
}

impl GetWorkAttachedTags {
    pub fn new(ids: Vec<String>) -> anyhow::Result<Self> {
        let mut tag_ids = vec![];
        for id in ids {
            tag_ids.push(Id::new(Ulid::from_string(&id)?))
        }
        Ok(GetWorkAttachedTags { tag_ids })
    }
}
