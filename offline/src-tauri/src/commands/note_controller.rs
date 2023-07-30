use chrono::Utc;
use std::{fs, path::PathBuf, time::SystemTime};
use tauri::api::path::document_dir;

use crate::commands::*;
use crate::models::*;

fn get_files_in_folder(path: PathBuf) -> Result<Vec<PathBuf>, NotesError> {
    let entries = path.read_dir()?;
    let all = entries
        .filter_map(|entry| Some(entry.ok()?.path()))
        .collect::<Vec<PathBuf>>();
    Ok(all)
}

fn read_directory() -> Result<Vec<PathBuf>, NotesError> {
    let vault_path: PathBuf = match document_dir() {
        Some(mut dir) => {
            dir.push("Nota");
            dir
        }
        None => return Err(NotesError::Internal("Document dir not set.".to_string())),
    };

    // check if it exists and create if not
    match vault_path.try_exists() {
        Ok(b) => {
            if !b || !vault_path.is_dir() {
                fs::create_dir(&vault_path)?;
            }
        }
        Err(_) => return Err(NotesError::Internal("File system error".to_string())),
    }

    get_files_in_folder(vault_path)
}

fn convert_file_to_note(file: PathBuf) -> NoteDTO {
    let mut created_at = Utc::now();
    let mut updated_at = Utc::now();
    if let Ok(metadata) = file.metadata() {
        created_at = metadata.created().unwrap_or(SystemTime::now()).into();
        updated_at = metadata.modified().unwrap_or(SystemTime::now()).into();
    }

    let default_note = NoteDTO {
        title: file.file_name().unwrap().to_str().unwrap().to_string(),
        created_at: created_at.naive_utc(),
        updated_at: updated_at.naive_utc(),
        ..Default::default()
    };

    let content = fs::read_to_string(&file).unwrap();

    convert_to_note(content, default_note)
}

#[tauri::command]
pub async fn save_note(note_input: String) -> Result<NoteDTO, NotesError> {
    let default_note: NoteDTO = Default::default();

    // validation
    let note = convert_to_note(note_input, default_note);

    // TODO convert back to markdown

    // TODO do the saving

    Ok(note)
}

#[tauri::command]
pub async fn fetch_notes() -> Result<PaginatedResult<NoteDTO>, NotesError> {
    let vault_contents: Vec<PathBuf> = match read_directory() {
        Ok(files) => files,
        Err(err) => return Err(err),
    };

    let notes = vault_contents
        .into_iter()
        .map(convert_file_to_note)
        .collect::<Vec<NoteDTO>>();

    Ok(PaginatedResult {
        records: notes,
        total: 3,
        current_page: 1,
        per_page: 12,
    })
}
