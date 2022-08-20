use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tauri::State;

use crate::{
    app::model::artist::CreateArtist,
    driver::context::errors::CommandError,
    driver::module::Modules,
    driver::{model::import_directory::*, module::ModulesExt},
};

#[tauri::command]
pub async fn import_directory(
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: HashMap<u8, HashMap<u8, String>>,
) -> anyhow::Result<(), CommandError> {
    let mut artist_usage_map = HashMap::new();
    // usages の validate
    for each_path_usage in usages.iter() {
        for each_deps_usage in each_path_usage.1.iter() {
            match &**(each_deps_usage.1) {
                "タグ" => {}
                "作者名" => {
                    artist_usage_map.insert(each_path_usage.0, each_deps_usage.0);
                }
                "作品名" => {}
                "無視する" => {}
                _ => {
                    return Err(CommandError::Anyhow(anyhow::anyhow!(
                        "usage not match (タグ | 作者名 | 作品名 | 無視する)"
                    )));
                }
            }
        }
    }

    // 対象の artist の Set をつくる
    let mut artist_set = HashSet::new();
    for dir_path_info in dir_path_infos.iter() {
        match artist_usage_map.get(&(dir_path_info.dir_deps.len() as u8)) {
            Some(deps) => {
                artist_set.insert(&dir_path_info.dir_deps[**deps as usize].name);
            }
            None => {}
        }
        artist_usage_map.get(&(dir_path_info.dir_deps.len() as u8));
    }

    // artist がないなら insert
    for new_artist_name in artist_set.into_iter() {
        modules
            .artist_use_case()
            .register_artist(CreateArtist::new((*new_artist_name).clone()))
            .await?;
    }

    // 対象の work を insert
    // 対象のタグを insert
    // 対象のタグマップを insert

    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}
