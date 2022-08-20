use crate::kernel::model::work::Work;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::{model::artist::Artist, repository::artist::ArtistRepository};

use crate::adapter::persistence::sqlite::Db;

use super::repository::DatabaseRepositoryImpl;

pub struct RepositoriesModule {
    artist_repository: DatabaseRepositoryImpl<Artist>,
    work_repository: DatabaseRepositoryImpl<Work>,
}

pub trait RepositoriesModuleExt {
    type ArtistRepo: ArtistRepository;
    type WorkRepo: WorkRepository;

    fn artist_repository(&self) -> &Self::ArtistRepo;
    fn work_repository(&self) -> &Self::WorkRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type ArtistRepo = DatabaseRepositoryImpl<Artist>;
    type WorkRepo = DatabaseRepositoryImpl<Work>;

    fn artist_repository(&self) -> &Self::ArtistRepo {
        &self.artist_repository
    }
    fn work_repository(&self) -> &Self::WorkRepo {
        &self.work_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let artist_repository = DatabaseRepositoryImpl::new(db.clone());
        let work_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            artist_repository,
            work_repository,
        }
    }
}
