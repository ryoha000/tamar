use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::artist::CreateArtist,
    driver::context::errors::CommandError,
    driver::model::import_directory::*,
    driver::module::{Modules, ModulesExt},
};

#[tauri::command]
pub async fn import_directory(
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: Vec<Usages>,
) -> anyhow::Result<(), CommandError> {
    modules
        .artist_use_case()
        .register_artist(CreateArtist::new("ryoha000".into()))
        .await?;

    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}
