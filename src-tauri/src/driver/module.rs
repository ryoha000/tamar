use std::sync::Arc;

use crate::adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::sqlite::Db,
};
use crate::app::usecase::artist::ArtistUseCase;

pub struct Modules {
    artist_use_case: ArtistUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn artist_use_case(&self) -> &ArtistUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn artist_use_case(&self) -> &ArtistUseCase<Self::RepositoriesModule> {
        &self.artist_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let db = Db::new().await;

        let repositories_module = Arc::new(RepositoriesModule::new(db.clone()));

        let artist_use_case = ArtistUseCase::new(repositories_module.clone());

        Self { artist_use_case }
    }
}
