#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod commands;
pub mod forms;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::import_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
