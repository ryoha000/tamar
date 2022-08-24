use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work::{SearchAroundTitleWorkView, SearchAroundUpdatedAtWorkView, UpdateTitleWork},
    driver::{
        context::errors::CommandError,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn search_around_title_work(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    is_before: bool,
    title: String,
) -> anyhow::Result<Vec<String>, CommandError> {
    let works = modules
        .work_use_case()
        .search_around_title(SearchAroundTitleWorkView::new(limit, is_before, title))
        .await?
        .into_iter()
        .map(|v| v.value.to_string())
        .collect();

    Ok(works)
}

#[tauri::command]
pub async fn search_around_updated_at_work(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    is_before: bool,
    updated_at: String,
) -> anyhow::Result<Vec<String>, CommandError> {
    let works = modules
        .work_use_case()
        .search_around_updated_at(SearchAroundUpdatedAtWorkView::new(
            limit, is_before, updated_at,
        ))
        .await?
        .into_iter()
        .map(|v| v.value.to_string())
        .collect();

    Ok(works)
}

#[tauri::command]
pub async fn update_work_title(
    modules: State<'_, Arc<Modules>>,
    id: String,
    title: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .work_use_case()
        .update_work_title(UpdateTitleWork::new(id, title))
        .await?;
    Ok(())
}
