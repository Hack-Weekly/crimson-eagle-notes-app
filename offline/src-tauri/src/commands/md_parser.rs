use chrono::Utc;
use comrak::nodes::{AstNode, NodeValue};
use comrak::{markdown_to_html, parse_document, Arena, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::models::*;

#[derive(Debug,Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MdResponse {
    markup: String,
}

/// Parse Markdown string into HTML Markup string
#[tauri::command]
pub async fn parse_md_to_html(md_string: String) -> MdResponse {
    let mut comrak_options = ComrakOptions::default(); //Comrak options below
    comrak_options.extension.autolink = true; // Auto detect links
    comrak_options.extension.table = true; // Auto detect tables
    comrak_options.extension.tasklist = true; // Auto detect task lists
    comrak_options.render.unsafe_ = true; // Allow raw HTML
    comrak_options.render.hardbreaks = true; // Treat newlines as hard line breaks
    let unsafe_mu_string = markdown_to_html(&md_string, &comrak_options);
    let safe_mu_string = ammonia::Builder::new() // Sanitize HTML for unsafe strings
        .add_tag_attributes("code", &["class"]) // Allow class attribute on code tags (req for syntax highlighting)
        .clean(&unsafe_mu_string)
        .to_string();
    MdResponse {
        markup: safe_mu_string,
    }
}

fn iter_nodes<'a>(node: &'a AstNode<'a>) -> Vec<String> {
    let mut stack = Vec::<String>::new();

    if let NodeValue::FrontMatter(block) = &mut node.data.borrow_mut().value { // block is a String
        stack.push(block.clone());
    }
    
    for c in node.children() {
        stack.append(&mut iter_nodes(c));
    }

    stack
}

pub fn convert_to_note(file: PathBuf) -> NoteDTO {
    let content = fs::read_to_string(&file).unwrap();
    
    let mut comrak_options = ComrakOptions::default(); //Comrak options below
    comrak_options.extension.autolink = true; // Auto detect links
    comrak_options.extension.table = true; // Auto detect tables
    comrak_options.extension.tasklist = true; // Auto detect task lists
    comrak_options.render.unsafe_ = true; // Allow raw HTML
    comrak_options.render.hardbreaks = true; // Treat newlines as hard line breaks
    comrak_options.extension.front_matter_delimiter = Some("---".to_owned());
    let unsafe_mu_string = markdown_to_html(&content, &comrak_options);
    let safe_mu_string = ammonia::Builder::new() // Sanitize HTML for unsafe strings
        .add_tag_attributes("code", &["class"]) // Allow class attribute on code tags (req for syntax highlighting)
        .clean(&unsafe_mu_string)
        .to_string();

    let arena = Arena::new();
    let root = parse_document(&arena, &content, &comrak_options);
    
    let frontmatter = iter_nodes(root).join("").replace("---", "");
    let frontmatter: NoteFrontmatter = match serde_yaml::from_str(&frontmatter) {
        Ok(f) => f,
        Err(_) => NoteFrontmatter::new(),
    };

    NoteDTO {
        id: frontmatter.id.unwrap_or(1),
        title: frontmatter.title.unwrap_or(file.file_name().unwrap().to_str().unwrap().to_string()),
        excerpt: frontmatter.excerpt.unwrap_or("excerpt".to_string()),
        content: safe_mu_string,
        color: frontmatter.color.unwrap_or("orange".to_string()),
        starred: frontmatter.starred.unwrap_or(false),
        created_at: frontmatter.created_at.unwrap_or(Utc::now().naive_utc()),
        updated_at: frontmatter.created_at.unwrap_or(Utc::now().naive_utc()),
    }
}






    
