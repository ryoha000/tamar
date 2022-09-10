use crate::{
    driver::context::errors::CommandError,
    migration::{drop_all_table, migration},
};

#[tauri::command]
pub async fn delete_all_data() -> anyhow::Result<(), CommandError> {
    drop_all_table().await?;
    migration().await;

    Ok(())
}
