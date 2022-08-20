use crate::kernel::model::tag::Tag;
use crate::kernel::model::work::Work;
use crate::kernel::model::work_tag_map::WorkTagMap;
use crate::kernel::repository::tag::TagRepository;
use crate::kernel::repository::work::WorkRepository;
use crate::kernel::repository::work_tag_map::WorkTagMapRepository;
use crate::kernel::{model::artist::Artist, repository::artist::ArtistRepository};

use crate::adapter::persistence::sqlite::Db;

use super::repository::DatabaseRepositoryImpl;

pub struct RepositoriesModule {
    artist_repository: DatabaseRepositoryImpl<Artist>,
    work_repository: DatabaseRepositoryImpl<Work>,
    tag_repository: DatabaseRepositoryImpl<Tag>,
    work_tag_map_repository: DatabaseRepositoryImpl<WorkTagMap>,
}

pub trait RepositoriesModuleExt {
    type ArtistRepo: ArtistRepository;
    type WorkRepo: WorkRepository;
    type TagRepo: TagRepository;
    type WorkTagMapRepo: WorkTagMapRepository;

    fn artist_repository(&self) -> &Self::ArtistRepo;
    fn work_repository(&self) -> &Self::WorkRepo;
    fn tag_repository(&self) -> &Self::TagRepo;
    fn work_tag_map_repository(&self) -> &Self::WorkTagMapRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type ArtistRepo = DatabaseRepositoryImpl<Artist>;
    type WorkRepo = DatabaseRepositoryImpl<Work>;
    type TagRepo = DatabaseRepositoryImpl<Tag>;
    type WorkTagMapRepo = DatabaseRepositoryImpl<WorkTagMap>;

    fn artist_repository(&self) -> &Self::ArtistRepo {
        &self.artist_repository
    }
    fn work_repository(&self) -> &Self::WorkRepo {
        &self.work_repository
    }
    fn tag_repository(&self) -> &Self::TagRepo {
        &self.tag_repository
    }
    fn work_tag_map_repository(&self) -> &Self::WorkTagMapRepo {
        &self.work_tag_map_repository
    }
}

impl RepositoriesModule {
    pub fn new(db: Db) -> Self {
        let artist_repository = DatabaseRepositoryImpl::new(db.clone());
        let work_repository = DatabaseRepositoryImpl::new(db.clone());
        let tag_repository = DatabaseRepositoryImpl::new(db.clone());
        let work_tag_map_repository = DatabaseRepositoryImpl::new(db.clone());
        Self {
            artist_repository,
            work_repository,
            tag_repository,
            work_tag_map_repository,
        }
    }
}
