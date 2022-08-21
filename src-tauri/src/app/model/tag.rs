use crate::kernel::model::{tag::NewTag, Id};
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

#[derive(new)]
pub struct SearchTag {
    pub name: String,
}
