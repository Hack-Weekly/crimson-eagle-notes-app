// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler!  [ // Pass in commands
    commands::md_parser::parse_md_to_html
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
