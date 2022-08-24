use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::tag_view::SelectTagView,
    driver::{
        context::errors::CommandError,
        model::tag_view::JsonTagView,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn select_tag(
    modules: State<'_, Arc<Modules>>,
    limit: u8,
) -> anyhow::Result<Vec<JsonTagView>, CommandError> {
    let tags = modules
        .tag_use_case()
        .select(SelectTagView::new(limit))
        .await?
        .into_iter()
        .map(|tag| JsonTagView::from(tag))
        .collect();

    Ok(tags)
}
