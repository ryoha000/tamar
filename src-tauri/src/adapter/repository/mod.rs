pub mod artist;
pub mod tag;
pub mod work;
pub mod work_tag_map;

use super::persistence::sqlite::Db;
use derive_new::new;
use std::marker::PhantomData;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}
