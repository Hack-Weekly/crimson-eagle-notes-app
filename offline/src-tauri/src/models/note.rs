use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteDTO {
    pub id: u32,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub color: String,
    pub starred: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub id: Option<u32>,
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
            color: Some(String::from("green")),
            starred: Some(false),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }
    }
}
