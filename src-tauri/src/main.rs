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

fn main() {
    // block_on(migration::migration());

    let modules = Arc::new(block_on(Modules::new()));

    tauri::Builder::default()
        .setup(|app| {
            app.manage(modules);

            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![driver::commands::import_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
