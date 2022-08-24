use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work_tag_map::CreateWorkTagMap,
    driver::{
        context::errors::CommandError,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn attach_tag(
    modules: State<'_, Arc<Modules>>,
    work_id: String,
    tag_id: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .work_tag_map_use_case()
        .register_work_tag_map(CreateWorkTagMap::from_raw(work_id, tag_id)?)
        .await?;

    Ok(())
}
