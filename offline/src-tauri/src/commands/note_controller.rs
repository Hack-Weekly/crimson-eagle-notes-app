use chrono::Utc;
use std::{fs, path::PathBuf};
use tauri::api::path::document_dir;

use crate::models::*;
use crate::commands::*;

fn get_files_in_folder(path: PathBuf) -> Result<Vec<PathBuf>, NotesError> {
    let entries = path.read_dir()?;
    let all = entries.filter_map(|entry| Some(entry.ok()?.path())).collect::<Vec<PathBuf>>();
    Ok(all)
}

fn read_directory() -> Result<Vec<PathBuf>, NotesError> {
    let vault_path: PathBuf = match document_dir() {
        Some(mut dir) => {
            dir.push("Nota");
            dir
        },
        None => return Err(NotesError::Internal("Document dir not set.".to_string())),
    };

    // check if it exists and create if not
    match vault_path.try_exists() {
        Ok(b) => if !b || !vault_path.is_dir() {
            fs::create_dir(&vault_path)?;
        },
        Err(_) => return Err(NotesError::Internal("File system error".to_string())),
    }

    get_files_in_folder(vault_path)
}

#[tauri::command]
pub async fn fetch_notes() -> Result<PaginatedResult<NoteDTO>, NotesError> {

    let vault_contents: Vec<PathBuf> = match read_directory() {
        Ok(files) => files,
        Err(err) => return Err(err),
    };

    let notes = vault_contents.into_iter().map(convert_to_note).collect::<Vec<NoteDTO>>();
    
    Ok(PaginatedResult {
        records: notes,
        total: 3,
        current_page: 1,
        per_page: 12,
    })
}






    
