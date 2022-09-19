use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{
        artist::{CreateArtist, GetByNameArtist},
        file::{SaveOriginalFiles, SaveThumbnails},
        work::{GetByTitleWork, ImportWork},
        work_history::CreateWorkHistory,
    },
    driver::context::errors::CommandError,
    driver::module::Modules,
    driver::module::ModulesExt,
    kernel::model::{work::Work, Id},
};

#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    count: i32,
}

#[tauri::command]
pub async fn import_file(
    modules: State<'_, Arc<Modules>>,
    artist_name: String,
    file_paths: Vec<String>,
) -> anyhow::Result<(), CommandError> {
    let mut features = vec![];
    for file_path_str in file_paths.into_iter() {
        // -------- artist に関係する処理 ここから ---------
        // artist を insert
        modules
            .artist_use_case()
            .register_artist(CreateArtist::new(artist_name.clone()))
            .await?;

        // work の insert に使うため insert したはずの artist を取得
        let artist = modules
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
        let work_title = modules.file_use_case().get_file_name(&file_path_str)?;
        // work の insert
        modules
            .work_use_case()
            .import_work(ImportWork::new(
                work_title.clone(),
                artist.id.clone(),
                file_path_str.clone(),
            ))
            .await?;

        // 保存先に使うため insert したはずの work を取得
        let work = modules
            .work_use_case()
            .get_by_title_work(GetByTitleWork::new(work_title.clone(), artist.id.clone()))
            .await?;

        if work.is_none() {
            return Err(CommandError::Anyhow(anyhow::anyhow!(
                "work is not found(internal error)"
            )));
        }
        let work = work.unwrap();

        // 閲覧マーカーをつけたい
        modules
            .work_history_use_case()
            .register_work_history(CreateWorkHistory::new(work.id.value.to_string()))
            .await?;
        // -------- work に関係する処理 ここまで ---------

        let m = modules.inner().clone();
        let handle = tauri::async_runtime::spawn(async move {
            import_individual(m, work.id, file_path_str, work_title)
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
    id: Id<Work>,
    file_path_str: String,
    work_title: String,
) -> anyhow::Result<(), CommandError> {
    let is_zip_file = file_path_str.ends_with("zip");
    let dir_path_str;
    if is_zip_file {
        // zip ファイルを解凍
        let zip_dir_path = m.file_use_case().get_zip_exclude_dir_path(&work_title);
        m.file_use_case()
            .extract_zip_file(&file_path_str, &zip_dir_path)?;

        dir_path_str = zip_dir_path;
    } else {
        dir_path_str = file_path_str;
    }

    m.file_use_case().save_thumbnail(SaveThumbnails {
        src_path: dir_path_str.clone(),
        id: id.clone(),
    })?;

    // ファイルコピー
    m.file_use_case()
        .save_original_files(SaveOriginalFiles::new(id.clone(), dir_path_str.clone()))?;

    if is_zip_file {
        // 解凍したディレクトリを消す
        m.file_use_case().delete_dir(dir_path_str)?;
    }

    Ok(())
}
