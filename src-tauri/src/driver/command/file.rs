use std::sync::Arc;
use tauri::State;

use crate::driver::{
    context::errors::CommandError,
    module::{Modules, ModulesExt},
};

#[tauri::command]
pub async fn rotate_work_file(
    modules: State<'_, Arc<Modules>>,
    file: String,
) -> anyhow::Result<(), CommandError> {
    modules.file_use_case().rotate_image_file(file)?;
    Ok(())
}

#[tauri::command]
pub async fn delete_work_file(
    modules: State<'_, Arc<Modules>>,
    file: String,
) -> anyhow::Result<(), CommandError> {
    modules.file_use_case().delete_work_file(file)?;
    Ok(())
}
