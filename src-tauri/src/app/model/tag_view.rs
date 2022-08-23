use derive_new::new;

use crate::kernel::model::tag::Tag;

pub struct TagView {
    pub id: String,
    pub name: String,
    pub updated_at: String,
}

impl TagView {
    pub fn new(tag: Tag) -> Self {
        Self {
            id: tag.id.value.to_string(),
            name: tag.name,
            updated_at: tag.updated_at.to_string(),
        }
    }
}

#[derive(new)]
pub struct SearchByNameTagView<'a> {
    pub name: &'a str,
}
