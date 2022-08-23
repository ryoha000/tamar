use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work_view::{GetWorkView, SearchWorkView},
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
    //! まぎらわしい仕様についてのメモ /
    //! tags の指定は AND 検索になる ( tags を指定するとその tag を持たない work は表示されない) /

    // TODO: タグを用いた検索(SearchWorkView に `work`.`id` IN (?) 用のプロパティを生やす)

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
