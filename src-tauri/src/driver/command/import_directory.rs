use std::sync::Arc;
use tauri::State;

use crate::{
    driver::context::errors::CommandError, driver::model::import_directory::*,
    driver::module::Modules,
};

#[tauri::command]
pub async fn import_directory(
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: Vec<Usages>,
) -> anyhow::Result<(), CommandError> {
    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}
