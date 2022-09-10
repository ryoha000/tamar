use crate::kernel::model::{
    tag::{NewTag, Tag},
    Id,
};
use derive_new::new;

#[derive(new)]
pub struct CreateTag {
    pub name: String,
}

impl TryFrom<CreateTag> for NewTag {
    type Error = anyhow::Error;

    fn try_from(c: CreateTag) -> anyhow::Result<Self> {
        let tag_id = Id::gen();
        Ok(NewTag::new(tag_id, c.name))
    }
}

pub struct GetTag {
    pub id: Id<Tag>,
}

impl GetTag {
    pub fn new(id: String) -> anyhow::Result<Self> {
        let tag_id = ulid::Ulid::from_string(&id)?;
        Ok(GetTag {
            id: Id::new(tag_id),
        })
    }
}

#[derive(new)]
pub struct GetByNameTag {
    pub name: String,
}
