use std::sync::Arc;

use crate::app::usecase::artist::ArtistUseCase;
use crate::app::usecase::artist_view::ArtistViewUseCase;
use crate::app::usecase::file::FileUseCase;
use crate::app::usecase::tag::TagUseCase;
use crate::app::usecase::work_tag_map::WorkTagMapUseCase;
use crate::app::usecase::work_view::WorkViewUseCase;
use crate::{
    adapter::{
        modules::{RepositoriesModule, RepositoriesModuleExt},
        persistence::sqlite::Db,
    },
    app::usecase::work::WorkUseCase,
};

pub struct Modules {
    artist_use_case: ArtistUseCase<RepositoriesModule>,
    work_use_case: WorkUseCase<RepositoriesModule>,
    tag_use_case: TagUseCase<RepositoriesModule>,
    work_tag_map_use_case: WorkTagMapUseCase<RepositoriesModule>,
    work_view_use_case: WorkViewUseCase<RepositoriesModule>,
    artist_view_use_case: ArtistViewUseCase<RepositoriesModule>,
    file_use_case: FileUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn artist_use_case(&self) -> &ArtistUseCase<Self::RepositoriesModule>;
    fn work_use_case(&self) -> &WorkUseCase<Self::RepositoriesModule>;
    fn tag_use_case(&self) -> &TagUseCase<Self::RepositoriesModule>;
    fn work_tag_map_use_case(&self) -> &WorkTagMapUseCase<Self::RepositoriesModule>;
    fn work_view_use_case(&self) -> &WorkViewUseCase<Self::RepositoriesModule>;
    fn artist_view_use_case(&self) -> &ArtistViewUseCase<Self::RepositoriesModule>;
    fn file_use_case(&self) -> &FileUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn artist_use_case(&self) -> &ArtistUseCase<Self::RepositoriesModule> {
        &self.artist_use_case
    }
    fn work_use_case(&self) -> &WorkUseCase<Self::RepositoriesModule> {
        &self.work_use_case
    }
    fn tag_use_case(&self) -> &TagUseCase<Self::RepositoriesModule> {
        &self.tag_use_case
    }
    fn work_tag_map_use_case(&self) -> &WorkTagMapUseCase<Self::RepositoriesModule> {
        &self.work_tag_map_use_case
    }
    fn work_view_use_case(&self) -> &WorkViewUseCase<Self::RepositoriesModule> {
        &self.work_view_use_case
    }
    fn artist_view_use_case(&self) -> &ArtistViewUseCase<Self::RepositoriesModule> {
        &self.artist_view_use_case
    }
    fn file_use_case(&self) -> &FileUseCase<Self::RepositoriesModule> {
        &self.file_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let artist_use_case = ArtistUseCase::new(repositories_module.clone());
        let work_use_case = WorkUseCase::new(repositories_module.clone());
        let tag_use_case = TagUseCase::new(repositories_module.clone());
        let work_tag_map_use_case = WorkTagMapUseCase::new(repositories_module.clone());
        let work_view_use_case = WorkViewUseCase::new(repositories_module.clone());
        let artist_view_use_case = ArtistViewUseCase::new(repositories_module.clone());
        let file_use_case = FileUseCase::new(repositories_module.clone());

        Self {
            artist_use_case,
            work_use_case,
            tag_use_case,
            work_tag_map_use_case,
            work_view_use_case,
            artist_view_use_case,
            file_use_case,
        }
    }
}
