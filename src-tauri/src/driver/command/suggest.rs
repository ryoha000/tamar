use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{artist_view::SearchByNameArtistView, tag_view::SearchByNameTagView},
    driver::{
        context::errors::CommandError,
        model::suggest::JsonSuggest,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn get_suggest(
    modules: State<'_, Arc<Modules>>,
    text: String,
) -> anyhow::Result<JsonSuggest, CommandError> {
    let artists = modules
        .artist_view_use_case()
        .search_by_name(SearchByNameArtistView::new(&text))
        .await?;

    let tags = modules
        .tag_use_case()
        .search_by_name(SearchByNameTagView::new(&text))
        .await?;

    Ok(JsonSuggest::new(artists, tags))
}