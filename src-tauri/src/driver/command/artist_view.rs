use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::artist_view::{GetArtistView, SearchArtistView},
    driver::{
        context::errors::CommandError,
        model::artist_view::JsonArtistView,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn search_artist(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
    offset: u8,
    search: String,
    sort_col: String,
    sort_desc: bool,
) -> anyhow::Result<Vec<JsonArtistView>, CommandError> {
    let artists = modules
        .artist_view_use_case()
        .search(SearchArtistView::new(
            limit,
            offset,
            sort_col,
            sort_desc,
            search.clone(),
        ))
        .await?
        .into_iter()
        .map(|v| JsonArtistView::from(v))
        .collect();

    Ok(artists)
}

#[tauri::command]
pub async fn get_artist(
    modules: State<'_, Arc<Modules>>,
    id: String,
) -> anyhow::Result<JsonArtistView, CommandError> {
    let artist = modules
        .artist_view_use_case()
        .get(GetArtistView::new(id))
        .await?;

    Ok(JsonArtistView::from(artist))
}
