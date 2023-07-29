use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tinyid::TinyId;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteDTO {
    pub id: TinyId,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub color: String,
    pub starred: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Default for NoteDTO {
    fn default() -> Self {
        NoteDTO {
            id: TinyId::random(),
            title: "New Note".to_string(),
            excerpt: "Excerpt".to_string(),
            content: "".to_string(),
            color: "green".to_string(),
            starred: false,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize)]
pub struct PaginatedResult<T> {
    pub records: Vec<T>,
    pub total: u32,
    pub current_page: u32,
    pub per_page: u32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NoteFrontmatter {
    pub id: Option<TinyId>,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub color: Option<String>,
    pub starred: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NoteFrontmatter {
    pub fn new() -> NoteFrontmatter {
        NoteFrontmatter {
            id: None,
            title: None,
            excerpt: None,
            color: None,
            starred: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<NoteDTO> for NoteFrontmatter {
    fn from(note: NoteDTO) -> Self {
        NoteFrontmatter {
            id: Some(note.id),
            title: Some(note.title),
            excerpt: Some(note.excerpt),
            color: Some(note.color),
            starred: Some(note.starred),
            created_at: Some(note.created_at),
            updated_at: Some(note.updated_at),
        }
    }
}
