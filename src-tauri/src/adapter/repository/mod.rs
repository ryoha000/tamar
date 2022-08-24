pub mod artist;
pub mod tag;
pub mod work;
pub mod work_tag_map;
pub mod file;

use super::persistence::sqlite::Db;
use derive_new::new;
use std::marker::PhantomData;

#[derive(new)]
pub struct DatabaseRepositoryImpl<T> {
    pool: Db,
    _marker: PhantomData<T>,
}

#[derive(new)]
pub struct RepositoryImpl<T> {
    _marker: PhantomData<T>,
}
