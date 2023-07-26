// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // Pass in commands
            commands::md_parser::parse_md_to_html,
            commands::note_controller::fetch_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
