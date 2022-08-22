use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work_view::{
        GetWorkView, SearchAroundTitleWorkView, SearchAroundUpdatedAtWorkView, SearchWorkView,
    },
    driver::{
        context::errors::CommandError,
        model::work_view::JsonWorkView,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn search_work(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    offset: u8,
    search: String,
    tags: Vec<String>,
    sort_col: String,
    sort_desc: bool,
) -> anyhow::Result<Vec<JsonWorkView>, CommandError> {
    // TODO: タグを用いた検索
    // TODO: search を artist にも適用(search が空文字の時はスキップ)
    // TODO: search を tag にも適用(search が空文字の時はスキップ)

    let works = modules
        .work_view_use_case()
        .search(SearchWorkView::new(
            limit,
            offset,
            sort_col,
            sort_desc,
            search.clone(),
        ))
        .await?
        .into_iter()
        .map(|v| JsonWorkView::from(v))
        .collect();

    Ok(works)
}

#[tauri::command]
pub async fn search_around_title_work(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    is_before: bool,
    title: String,
) -> anyhow::Result<Vec<JsonWorkView>, CommandError> {
    let works = modules
        .work_view_use_case()
        .search_around_title(SearchAroundTitleWorkView::new(limit, is_before, title))
        .await?
        .into_iter()
        .map(|v| JsonWorkView::from(v))
        .collect();

    Ok(works)
}

#[tauri::command]
pub async fn search_around_updated_at_work(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    is_before: bool,
    updated_at: String,
) -> anyhow::Result<Vec<JsonWorkView>, CommandError> {
    let works = modules
        .work_view_use_case()
        .search_around_updated_at(SearchAroundUpdatedAtWorkView::new(
            limit, is_before, updated_at,
        ))
        .await?
        .into_iter()
        .map(|v| JsonWorkView::from(v))
        .collect();

    Ok(works)
}

#[tauri::command]
pub async fn get_work(
    modules: State<'_, Arc<Modules>>,
    id: String,
) -> anyhow::Result<JsonWorkView, CommandError> {
    let work = modules
        .work_view_use_case()
        .get_work(GetWorkView::new(id)?)
        .await?;

    Ok(JsonWorkView::from(work))
}
