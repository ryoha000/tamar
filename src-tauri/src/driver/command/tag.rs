use std::sync::Arc;
use tauri::State;

use crate::{
    app::model::{
        tag::{CreateTag, GetByNameTag},
        work_tag_map::{CreateWorkTagMap, DetachWorkTagMap},
    },
    driver::{
        context::errors::CommandError,
        module::{Modules, ModulesExt},
    },
};

#[tauri::command]
pub async fn attach_tag(
    modules: State<'_, Arc<Modules>>,
    work_id: String,
    tag_id: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .work_tag_map_use_case()
        .register_work_tag_map(CreateWorkTagMap::from_raw(work_id, tag_id)?)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn detach_tag(
    modules: State<'_, Arc<Modules>>,
    work_id: String,
    tag_id: String,
) -> anyhow::Result<(), CommandError> {
    modules
        .work_tag_map_use_case()
        .delete_work_tag_map(DetachWorkTagMap::from_raw(work_id, tag_id)?)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn attach_tag_by_name(
    modules: State<'_, Arc<Modules>>,
    work_id: String,
    name: String,
) -> anyhow::Result<(), CommandError> {
    let tag = modules
        .tag_use_case()
        .find_tag_by_name(GetByNameTag::new(name.clone()))
        .await?;

    let tag_id;
    if tag.is_none() {
        modules
            .tag_use_case()
            .register_tag(CreateTag::new(name.clone()))
            .await?;
        let tag = modules
            .tag_use_case()
            .find_tag_by_name(GetByNameTag::new(name))
            .await?
            .ok_or(anyhow::anyhow!("tag is not found(internal error)"))?;
        tag_id = tag.id
    } else {
        tag_id = tag.unwrap().id;
    }

    modules
        .work_tag_map_use_case()
        .register_work_tag_map(CreateWorkTagMap::from_raw(
            work_id,
            tag_id.value.to_string(),
        )?)
        .await?;

    Ok(())
}
