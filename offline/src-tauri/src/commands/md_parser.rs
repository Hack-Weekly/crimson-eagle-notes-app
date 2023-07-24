use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};


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






    
