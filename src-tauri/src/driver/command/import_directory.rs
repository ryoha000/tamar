use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tauri::State;

use crate::{
    app::model::{
        artist::{CreateArtist, SearchEqualArtist},
        tag::{CreateTag, SearchEqualTag},
        work::{CreateWork, SearchEqualWork},
        work_tag_map::CreateWorkTagMap,
    },
    driver::context::errors::CommandError,
    driver::module::{self, Modules},
    driver::{model::import_directory::*, module::ModulesExt},
    migration::UNKNOWN_ARTIST_NAME,
};

#[tauri::command]
pub async fn import_directory(
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: HashMap<u8, HashMap<u8, String>>,
) -> anyhow::Result<(), CommandError> {
    let mut artist_usage_map = HashMap::new();
    let mut title_usage_map = HashMap::new();
    let mut tag_usage_map = HashMap::new();
    // usages の validate
    for each_path_usage in usages.iter() {
        let mut is_title_exist = false;
        for each_deps_usage in each_path_usage.1.iter() {
            match &**(each_deps_usage.1) {
                "タグ" => {
                    tag_usage_map.insert(each_path_usage.0, each_deps_usage.0);
                }
                "作者名" => {
                    artist_usage_map.insert(each_path_usage.0, each_deps_usage.0);
                }
                "作品名" => {
                    title_usage_map.insert(each_path_usage.0, each_deps_usage.0);
                    is_title_exist = true
                }
                "無視する" => {}
                _ => {
                    return Err(CommandError::Anyhow(anyhow::anyhow!(
                        "usage not match (タグ | 作者名 | 作品名 | 無視する)"
                    )));
                }
            }
        }
        if !is_title_exist {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "title is not defined by usage"
            )));
        }
    }

    // 対象の artist の Set をつくる
    let mut artist_set = HashSet::new();
    for dir_path_info in dir_path_infos.iter() {
        match artist_usage_map.get(&(dir_path_info.dir_deps.len() as u8)) {
            Some(deps) => {
                artist_set.insert(&dir_path_info.dir_deps[(**deps as usize) - 1].name);
            }
            None => {}
        }
    }

    // artist がないなら insert
    for new_artist_name in artist_set.into_iter() {
        modules
            .artist_use_case()
            .register_artist(CreateArtist::new((*new_artist_name).clone()))
            .await?;
    }

    // 対象の work を insert
    for dir_path_info in dir_path_infos.iter() {
        let max_deps = &(dir_path_info.dir_deps.len() as u8);
        match title_usage_map.get(max_deps) {
            Some(deps) => {
                // work には artist が要るから取得
                let artist_name;
                match artist_usage_map.get(max_deps) {
                    Some(deps) => {
                        artist_name = dir_path_info.dir_deps[(**deps - 1) as usize].name.clone()
                    }
                    None => artist_name = UNKNOWN_ARTIST_NAME.to_string(),
                }
                let artist = modules
                    .artist_use_case()
                    .search_equal_artist(SearchEqualArtist::new(artist_name))
                    .await?;

                match artist {
                    Some(artist) => {
                        modules
                            .work_use_case()
                            .register_work(CreateWork::new(
                                dir_path_info.dir_deps[(**deps - 1) as usize].name.clone(),
                                artist.id,
                            ))
                            .await?;
                    }
                    None => {
                        return Err(CommandError::Anyhow(anyhow::anyhow!(
                            "artist is not found(internal error)"
                        )));
                    }
                }
            }
            None => {}
        }
    }

    // 対象のタグを insert
    // 対象の tag の Set をつくる
    let mut tag_set = HashSet::new();
    for dir_path_info in dir_path_infos.iter() {
        match tag_usage_map.get(&(dir_path_info.dir_deps.len() as u8)) {
            Some(deps) => {
                tag_set.insert(&dir_path_info.dir_deps[(**deps as usize) - 1].name);
            }
            None => {}
        }
    }

    for new_tag_name in tag_set.into_iter() {
        // tag がないなら insert
        modules
            .tag_use_case()
            .register_tag(CreateTag::new((*new_tag_name).clone()))
            .await?;
    }

    // 対象のタグマップを insert
    for dir_path_info in dir_path_infos.iter() {
        let max_deps = &(dir_path_info.dir_deps.len() as u8);

        // insert したはずの tag を取得
        let tag_name;
        match artist_usage_map.get(max_deps) {
            Some(deps) => tag_name = dir_path_info.dir_deps[(**deps - 1) as usize].name.clone(),
            None => continue, // TODO
        }
        let tag = modules
            .tag_use_case()
            .search_equal_tag(SearchEqualTag::new(tag_name))
            .await?;
        if tag.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "tag is not found(internal error)"
            )));
        }
        let tag = tag.unwrap();

        // insert したはずの artist を取得
        let artist_name;
        match artist_usage_map.get(max_deps) {
            Some(deps) => artist_name = dir_path_info.dir_deps[(**deps - 1) as usize].name.clone(),
            None => artist_name = UNKNOWN_ARTIST_NAME.to_string(),
        }
        let artist = modules
            .artist_use_case()
            .search_equal_artist(SearchEqualArtist::new(artist_name))
            .await?;
        if artist.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "artist is not found(internal error)"
            )));
        }
        let artist = artist.unwrap();

        let title;
        match title_usage_map.get(max_deps) {
            Some(deps) => title = dir_path_info.dir_deps[(**deps - 1) as usize].name.clone(),
            None => {
                return Err(CommandError::Anyhow(anyhow::anyhow!(
                    "title is not defined by usage(internal error)"
                )));
            }
        }
        let work = modules
            .work_use_case()
            .search_equal_work(SearchEqualWork::new(title, artist.id))
            .await?;
        if work.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "work is not found(internal error)"
            )));
        }
        let work = work.unwrap();

        // tag_map を insert
        modules
            .work_tag_map_use_case()
            .register_work_tag_map(CreateWorkTagMap::new(work.id, tag.id))
            .await?;
    }

    // ファイルコピー

    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}
