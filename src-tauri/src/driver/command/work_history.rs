use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work_history::CreateWorkHistory,
    driver::{
        context::errors::CommandError,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn view_work(
    modules: State<'_, Arc<Modules>>,
    work_id: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .work_history_use_case()
        .register_work_history(CreateWorkHistory::new(work_id))
        .await?;

    Ok(())
}
