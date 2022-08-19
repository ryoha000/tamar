use crate::kernel::{model::artist::Artist, repository::artist::ArtistRepository};

use crate::adapter::persistence::sqlite::Db;

use super::repository::DatabaseRepositoryImpl;

pub struct RepositoriesModule {
    artist_repository: DatabaseRepositoryImpl<Artist>,
}

pub trait RepositoriesModuleExt {
    type ArtistRepo: ArtistRepository;
    fn artist_repository(&self) -> &Self::ArtistRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type ArtistRepo = DatabaseRepositoryImpl<Artist>;

    fn artist_repository(&self) -> &Self::ArtistRepo {
        &self.artist_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let artist_repository = DatabaseRepositoryImpl::new(db.clone());
        Self { artist_repository }
    }
}
