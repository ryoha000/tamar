use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{
        artist_view::{GetArtistView, SearchByNameArtistView},
        tag::GetTag,
        tag_view::SearchByNameTagView,
    },
    driver::{
        context::errors::CommandError,
        model::{suggest::JsonSuggest, tag_view::JsonTagView},
        module::{Modules, ModulesExt},
    },
    kernel::model::search_history::SearchHistoryTypeEnum,
};

#[tauri::command]
pub async fn get_initial_suggest(
    modules: State<'_, Arc<Modules>>,
    limit: u32,
) -> anyhow::Result<JsonSuggest, CommandError> {
    let recent = modules
        .search_history_use_case()
        .get_recent_search_history(limit)
        .await?;

    let mut artists = vec![];
    let mut tags = vec![];
    for v in recent {
        match v.value_type {
            SearchHistoryTypeEnum::artist => {
                let artist = modules
                    .artist_view_use_case()
                    .get(GetArtistView::new(v.value_id))
                    .await?;
                artists.push(artist);
            }
            SearchHistoryTypeEnum::tag => {
                let tag = modules
                    .tag_use_case()
                    .find_tag(GetTag::new(v.value_id)?)
                    .await?;
                tags.push(tag);
            }
        }
    }

    Ok(JsonSuggest::new(artists, tags))
}

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

#[tauri::command]
pub async fn get_tag_suggest(
    modules: State<'_, Arc<Modules>>,
    text: String,
) -> anyhow::Result<Vec<JsonTagView>, CommandError> {
    let tags = modules
        .tag_use_case()
        .search_by_name(SearchByNameTagView::new(&text))
        .await?
        .into_iter()
        .map(|tag| JsonTagView::from(tag))
        .collect();

    Ok(tags)
}
