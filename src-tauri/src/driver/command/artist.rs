use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::artist::UpdateArtistName,
    driver::{
        context::errors::CommandError,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn update_artist_name(
    modules: State<'_, Arc<Modules>>,
    id: String,
    name: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .artist_use_case()
        .update_artist_name(UpdateArtistName::new(id, name)?)
        .await?;
    Ok(())
}
