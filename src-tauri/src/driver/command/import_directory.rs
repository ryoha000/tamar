use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tauri::State;

use crate::{
    driver::context::errors::CommandError, driver::model::import_directory::*,
    driver::module::Modules,
};

#[tauri::command]
pub async fn import_directory(
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: HashMap<u8, HashMap<u8, String>>,
) -> anyhow::Result<(), CommandError> {
    // 対象の artist の Set をつくる
    // let artist_set = HashSet::new();
    // artist がないなら insert
    // 対象の work を insert
    // 対象のタグを insert
    // 対象のタグマップを insert

    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}
