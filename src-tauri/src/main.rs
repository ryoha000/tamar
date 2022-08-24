#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use driver::module::Modules;
use tauri::async_runtime::block_on;
use tauri::Manager;

mod adapter;
mod app;
mod driver;
mod kernel;
mod migration;

#[cfg(test)]
mod test_util;

fn main() {
    block_on(migration::migration());

    let modules = Arc::new(block_on(Modules::new()));

    tauri::Builder::default()
        .setup(|app| {
            app.manage(modules);

            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            driver::command::import_directory::import_directory,
            driver::command::work_view::search_work,
            driver::command::work_view::get_work,
            driver::command::work_view::select_work_by_artist,
            driver::command::work::search_around_title_work,
            driver::command::work::search_around_updated_at_work,
            driver::command::work::update_work_title,
            driver::command::artist_view::search_artist,
            driver::command::artist_view::get_artist,
            driver::command::tag_view::select_tag,
            driver::command::tag::attach_tag,
            driver::command::tag::detach_tag,
            driver::command::tag::attach_tag_by_name,
            driver::command::suggest::get_suggest,
            driver::command::suggest::get_tag_suggest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
