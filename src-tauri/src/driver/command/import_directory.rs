use std::{collections::HashMap, fs, path, sync::Arc};
use tauri::State;

use crate::{
    app::model::{
        artist::{CreateArtist, SearchEqualArtist},
        tag::{CreateTag, GetByNameTag},
        work::{CreateWork, SearchEqualWork},
        work_tag_map::CreateWorkTagMap,
    },
    driver::context::errors::CommandError,
    driver::module::Modules,
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
    let mut tag_usage_map = HashMap::new(); // tag は複数追加可能だから型が違う

    // usages の validate
    for each_path_usage in usages.iter() {
        let mut is_title_exist = false;
        for each_deps_usage in each_path_usage.1.iter() {
            match &**(each_deps_usage.1) {
                "タグ" => {
                    if !tag_usage_map.contains_key(each_path_usage.0) {
                        tag_usage_map.insert(each_path_usage.0, vec![]);
                    }
                    tag_usage_map
                        .get_mut(each_path_usage.0)
                        .unwrap()
                        .push(each_deps_usage.0);
                }
                "作者名" => {
                    let is_existed = artist_usage_map
                        .insert(each_path_usage.0, each_deps_usage.0)
                        .is_some();
                    if is_existed {
                        return Err(CommandError::Anyhow(anyhow::anyhow!(
                            "artist has duplicated difinition"
                        )));
                    }
                }
                "作品名" => {
                    let is_existed = title_usage_map
                        .insert(each_path_usage.0, each_deps_usage.0)
                        .is_some();
                    if is_existed {
                        return Err(CommandError::Anyhow(anyhow::anyhow!(
                            "title has duplicated difinition"
                        )));
                    }
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

    for dir_path_info in dir_path_infos.iter() {
        let max_deps = &(dir_path_info.dir_deps.len() as u8);
        let get_name = |deps: &&u8| dir_path_info.dir_deps[(**deps - 1) as usize].name.clone();

        // -------- artist に関係する処理 ここから ---------
        // artist を insert
        let artist_name;
        match artist_usage_map.get(max_deps) {
            Some(deps) => {
                artist_name = get_name(deps);

                modules
                    .artist_use_case()
                    .register_artist(CreateArtist::new(artist_name.clone()))
                    .await?;
            }
            None => artist_name = UNKNOWN_ARTIST_NAME.to_string(), // UNKNOWN_ARTIST は最初に INSERT 済
        }

        // work の insert に使うため insert したはずの artist を取得
        let artist = modules
            .artist_use_case()
            .search_equal_artist(SearchEqualArtist::new(artist_name.clone()))
            .await?;
        if artist.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "artist is not found(internal error)"
            )));
        }
        let artist = artist.unwrap();
        // -------- artist に関係する処理 ここまで ---------

        // -------- work に関係する処理 ここから ---------
        let work_title;
        match title_usage_map.get(max_deps) {
            Some(deps) => {
                work_title = get_name(deps);
            }
            None => {
                return Err(CommandError::Anyhow(anyhow::anyhow!(
                    "title is not defined by usage(internal error)"
                )));
            }
        }
        // work の insert
        modules
            .work_use_case()
            .register_work(CreateWork::new(work_title.clone(), artist.id.clone()))
            .await?;

        // tag の insert に使うため insert したはずの work を取得
        let work = modules
            .work_use_case()
            .search_equal_work(SearchEqualWork::new(work_title.clone(), artist.id))
            .await?;

        if work.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "work is not found(internal error)"
            )));
        }
        let work = work.unwrap();
        // -------- work に関係する処理 ここまで ---------

        // ファイルコピー
        copy_work_files(&work_title, &artist_name, &dir_path_info.path)?; // TODO: 全然並列じゃない

        // -------- tag に関係する処理 ここから ---------
        match tag_usage_map.get(max_deps) {
            // tag をつけるとき
            Some(deps_vec) => {
                for deps in deps_vec.iter() {
                    let tag_name = get_name(deps);

                    // tag を insert
                    modules
                        .tag_use_case()
                        .register_tag(CreateTag::new(tag_name.clone()))
                        .await?;

                    // work_tag_map を作る必要があるため insert したはずの tag を取得
                    let tag = modules
                        .tag_use_case()
                        .find_tag_by_name(GetByNameTag::new(tag_name))
                        .await?;
                    if tag.is_none() {
                        return Err(CommandError::Anyhow(anyhow::anyhow!(
                            "tag is not found(internal error)"
                        )));
                    }
                    let tag = tag.unwrap();

                    // work_tag_map を insert
                    modules
                        .work_tag_map_use_case()
                        .register_work_tag_map(CreateWorkTagMap::new(work.id.clone(), tag.id))
                        .await?
                }
            }
            None => {} // tag をつけなくていいため何もしない
        }
        // -------- tag に関係する処理 ここまで ---------
    }

    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok(())
}

fn copy_work_files(work_title: &str, artist_name: &str, work_path: &str) -> anyhow::Result<()> {
    let copy_root_dir = "../tamar_content";

    let artist_dir_path_buf = path::Path::new(copy_root_dir).join(path::Path::new(artist_name));
    let dst_work_dir_path_buf = artist_dir_path_buf.join(path::Path::new(work_title));
    let dst_work_dir_path = dst_work_dir_path_buf.as_path();
    // コピー先のディレクトリをつくる
    fs::create_dir_all(dst_work_dir_path)?;

    let src_work_dir_path = path::Path::new(work_path);

    copy_files(dst_work_dir_path, src_work_dir_path, vec![])?;

    Ok(())
}

fn copy_files(
    dst_work_dir_path: &path::Path,
    dir_path: &path::Path,
    dirs: Vec<String>,
) -> anyhow::Result<()> {
    let children = fs::read_dir(dir_path)?;
    for child in children {
        let child = child?;

        let is_dir_child = child.file_type()?.is_dir();

        let child_path_buf = child.path();
        let child_path = child_path_buf.as_path();

        let child_name = child_path
            .file_name()
            .ok_or(anyhow::anyhow!("failed to get file_name"))?
            .to_str()
            .ok_or(anyhow::anyhow!("failed to get &str"))?
            .to_string();

        if is_dir_child {
            let mut new_dirs = dirs.to_vec();
            new_dirs.push(child_name);
            copy_files(dst_work_dir_path, child_path, new_dirs)?;
        } else {
            // child が ファイルの時は callback
            let dst_filename;
            match dirs.len() {
                0 => dst_filename = child_name,
                _ => dst_filename = format!("{}-{}", dirs.join("-"), child_name),
            }

            let dst_path_buf = dst_work_dir_path.join(path::Path::new(&dst_filename));
            let dst_path = dst_path_buf.as_path();
            fs::copy(child_path, dst_path)?;
        }
    }
    Ok(())
}
