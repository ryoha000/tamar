#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod adapter;
mod app;
mod commands;
mod forms;
mod kernel;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::import_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
