use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::work_view::SearchWorkView,
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
