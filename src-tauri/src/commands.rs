use super::forms::import_directory::*;

#[tauri::command]
pub async fn import_directory(
    dir_path_infos: Vec<DirPathInfo>,
    usages: Vec<Usages>,
) -> Result<String, String> {
    println!(
        "dir_path_info: {:#?}, usages: {:#?}",
        dir_path_infos, usages
    );
    Ok("hello my command".into())
}
