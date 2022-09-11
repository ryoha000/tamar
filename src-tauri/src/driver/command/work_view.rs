use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{
        work_tag_map::GetWorkAttachedTags,
        work_view::{GetWorkView, SearchWorkView, SelectByArtistView},
    },
    driver::{
        context::errors::CommandError,
        model::work_view::{JsonWorkView, JsonWorkViewSummary},
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn search_work(
    modules: State<'_, Arc<Modules>>,
    limit: u16,
    offset: u16,
    search: String,
    tags: Vec<String>,
    sort_col: String,
    sort_desc: bool,
) -> anyhow::Result<Vec<JsonWorkViewSummary>, CommandError> {
    //! まぎらわしい仕様についてのメモ /
    //! tags の指定は AND 検索になる ( tags を指定するとその tag を持たない work は表示されない) /

    let target_work_ids = modules
        .work_tag_map_use_case()
        .get_work_attached_tags(GetWorkAttachedTags::new(tags)?)
        .await?
        .into_iter()
        .map(|v| v.work_id)
        .collect();

    let works = modules
        .work_view_use_case()
        .search(SearchWorkView::new(
            limit,
            offset,
            sort_col,
            sort_desc,
            search.clone(),
            target_work_ids,
        ))
        .await?
        .into_iter()
        .map(|v| JsonWorkViewSummary::from(v))
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

#[tauri::command]
pub async fn select_work_by_artist(
    modules: State<'_, Arc<Modules>>,
    artist_id: String,
) -> anyhow::Result<Vec<JsonWorkViewSummary>, CommandError> {
    // TODO: limit が必要か考える
    let works = modules
        .work_view_use_case()
        .select_by_artist(SelectByArtistView::new(artist_id))
        .await?
        .into_iter()
        .map(|v| JsonWorkViewSummary::from(v))
        .collect();

    Ok(works)
}
