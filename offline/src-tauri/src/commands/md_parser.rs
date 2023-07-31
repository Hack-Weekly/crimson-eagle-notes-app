use comrak::nodes::{AstNode, NodeValue};
use comrak::{markdown_to_html, parse_document, Arena, ComrakOptions};
use serde::{Deserialize, Serialize};
use tinyid::TinyId;

use crate::models::*;

#[derive(Debug, Serialize, Deserialize)]
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

    if let NodeValue::FrontMatter(block) = &mut node.data.borrow_mut().value {
        // block is a String
        stack.push(block.clone());
    }

    for c in node.children() {
        stack.append(&mut iter_nodes(c));
    }

    stack
}

fn get_first_paragraph<'a>(node: &'a AstNode<'a>) -> Option<String> {
    let mut paragraph = None;

    if let NodeValue::Text(ref block) = node.data.borrow().value {
        if let NodeValue::Paragraph = node.parent().unwrap().data.borrow().value {
            return Some(block.clone());
        }
    }

    for c in node.children() {
        paragraph = get_first_paragraph(c);
        if paragraph.is_some() {
            return paragraph;
        }
    }

    paragraph
}

pub fn convert_to_note(content: String, default_note: NoteDTO) -> NoteDTO {
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
    let mut frontmatter: NoteFrontmatter = match serde_yaml::from_str(&frontmatter) {
        Ok(f) => f,
        Err(err) => {
            println!("Error: {}", err);
            NoteFrontmatter::new()
        }
    };
    println!("Frontmatter: {:?}", frontmatter);
    if frontmatter.excerpt.is_none() {
        frontmatter.excerpt = get_first_paragraph(root);
    }

    let mut note = default_note;
    note.content = safe_mu_string;

    // add optional data from frontmatter
    if let Some(id) = frontmatter.id {
        note.id = TinyId::from_u64_unchecked(id);
    }
    if let Some(title) = frontmatter.title {
        note.title = title;
    }
    if let Some(excerpt) = frontmatter.excerpt {
        note.excerpt = excerpt;
    }
    if let Some(color) = frontmatter.color {
        note.color = color;
    }
    if let Some(starred) = frontmatter.starred {
        note.starred = starred;
    }
    if let Some(created_at) = frontmatter.created_at {
        note.created_at = created_at;
    }
    if let Some(updated_at) = frontmatter.updated_at {
        note.updated_at = updated_at;
    }

    note
}
