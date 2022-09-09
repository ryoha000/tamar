use std::{collections::HashMap, sync::Arc};
use tauri::{State, Window};

use crate::{
    app::model::{
        artist::{CreateArtist, GetByNameArtist},
        file::{SaveOriginalFiles, SaveThumbnails},
        tag::{CreateTag, GetByNameTag},
        work::{GetByTitleWork, ImportWork},
        work_tag_map::CreateWorkTagMap,
    },
    driver::context::errors::CommandError,
    driver::module::Modules,
    driver::{model::import_directory::*, module::ModulesExt},
    kernel::model::{work::Work, Id},
    migration::UNKNOWN_ARTIST_NAME,
};

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    count: i32,
}

#[tauri::command]
pub async fn import_directory(
    window: Window,
    modules: State<'_, Arc<Modules>>,
    dir_path_infos: Vec<DirPathInfo>,
    usages: HashMap<u16, HashMap<u16, String>>,
) -> anyhow::Result<(), CommandError> {
    let mut artist_usage_map = HashMap::new();
    let mut title_usage_map = HashMap::new();
    let mut tag_usage_map = HashMap::new(); // tag は複数追加可能だから型が違う

    // usages の validate
    for each_path_usage in usages.into_iter() {
        let mut is_title_exist = false;
        for each_deps_usage in each_path_usage.1.into_iter() {
            match &*(each_deps_usage.1) {
                "タグ" => {
                    if !tag_usage_map.contains_key(&each_path_usage.0) {
                        tag_usage_map.insert(each_path_usage.0, vec![]);
                    }
                    tag_usage_map
                        .get_mut(&each_path_usage.0)
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

    let artist_usage_map = Arc::new(artist_usage_map);
    let title_usage_map = Arc::new(title_usage_map);
    let tag_usage_map = Arc::new(tag_usage_map); // tag は複数追加可能だから型が違う
    let window_arc = Arc::new(window);

    let mut features = vec![];
    for dir_path_info in dir_path_infos.into_iter() {
        let m = modules.inner().clone();
        let w = window_arc.clone();

        let max_deps = &(dir_path_info.dir_deps.len() as u16);
        let get_name = |deps: &u16| dir_path_info.dir_deps[(*deps - 1) as usize].name.clone();

        // -------- artist に関係する処理 ここから ---------
        // artist を insert
        let artist_name;
        match artist_usage_map.get(max_deps) {
            Some(deps) => {
                artist_name = get_name(deps);

                m.artist_use_case()
                    .register_artist(CreateArtist::new(artist_name.clone()))
                    .await?;
            }
            None => artist_name = UNKNOWN_ARTIST_NAME.to_string(), // UNKNOWN_ARTIST は最初に INSERT 済
        }

        // work の insert に使うため insert したはずの artist を取得
        let artist = m
            .artist_use_case()
            .find_artist_by_name(GetByNameArtist::new(artist_name.clone()))
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
        m.work_use_case()
            .import_work(ImportWork::new(
                work_title.clone(),
                artist.id.clone(),
                dir_path_info.path.clone(),
            ))
            .await?;

        // tag の insert に使うため insert したはずの work を取得
        let work = m
            .work_use_case()
            .get_by_title_work(GetByTitleWork::new(work_title.clone(), artist.id.clone()))
            .await?;

        if work.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "work is not found(internal error)"
            )));
        }
        let work = work.unwrap();
        // -------- work に関係する処理 ここまで ---------

        // -------- tag に関係する処理 ここから ---------
        match tag_usage_map.get(max_deps) {
            // tag をつけるとき
            Some(deps_vec) => {
                for deps in deps_vec.iter() {
                    let tag_name = get_name(deps);

                    // tag を insert
                    m.tag_use_case()
                        .register_tag(CreateTag::new(tag_name.clone()))
                        .await?;

                    // work_tag_map を作る必要があるため insert したはずの tag を取得
                    let tag = m
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
                    m.work_tag_map_use_case()
                        .register_work_tag_map(CreateWorkTagMap::new(work.id.clone(), tag.id))
                        .await?
                }
            }
            None => {} // tag をつけなくていいため何もしない
        }

        let handle = tauri::async_runtime::spawn(async move {
            import_individual(m, w, &dir_path_info, work.id.clone())
        });
        features.push(handle);
    }
    println!("end insert");

    for f in features {
        let join_res = f.await;
        if let Err(e) = join_res {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "{:#?} (internal error)",
                e
            )));
        } else {
            join_res.unwrap()?;
        }
    }

    Ok(())
}

fn import_individual(
    m: Arc<Modules>,
    window: Arc<Window>,
    dir_path_info: &DirPathInfo,
    id: Id<Work>,
) -> anyhow::Result<(), CommandError> {
    // ファイルコピー
    m.file_use_case()
        .save_original_files(SaveOriginalFiles::new(
            id.clone(),
            dir_path_info.path.clone(),
        ))?;
    if let Err(e) = window.emit("import_dir_progress", ProgressPayload { count: 1 }) {
        return Err(CommandError::Anyhow(anyhow::anyhow!(e)));
    }

    m.file_use_case().save_thumbnail(SaveThumbnails {
        src_path: dir_path_info.path.clone(),
        id: id.clone(),
    })?;

    Ok(())
}
