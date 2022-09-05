use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{
        artist::{CreateArtist, GetByNameArtist},
        file::SaveOriginalFiles,
        work::{CreateWork, GetByTitleWork},
    },
    driver::context::errors::CommandError,
    driver::module::Modules,
    driver::module::ModulesExt,
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
            .register_work(CreateWork::new(work_title.clone(), artist.id.clone()))
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
        // -------- work に関係する処理 ここまで ---------

        // zip ファイルを解凍
        let zip_dir_path = format!("../tmp/{}", work_title);
        modules
            .file_use_case()
            .extract_zip_file(&file_path_str, &zip_dir_path)?;

        // ファイルコピー
        modules
            .file_use_case()
            .save_original_files(SaveOriginalFiles::new(
                work.id.clone(),
                zip_dir_path.clone(),
            ))?;

        // 解凍したディレクトリを消す
        modules.file_use_case().delete_dir(zip_dir_path)?;
    }

    Ok(())
}
