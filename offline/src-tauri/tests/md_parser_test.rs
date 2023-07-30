use app::commands::md_parser::*;
use app::models::*;
use chrono::NaiveDateTime;
use tinyid::TinyId;

fn assert_eq_note(note: NoteDTO, expected_note: NoteDTO) {
    assert_eq!(note.id, expected_note.id);
    assert_eq!(note.title, expected_note.title);
    assert_eq!(note.excerpt, expected_note.excerpt);
    assert_eq!(note.content, expected_note.content);
    assert_eq!(note.color, expected_note.color);
    assert_eq!(note.starred, expected_note.starred);
    assert_eq!(note.created_at, expected_note.created_at);
    assert_eq!(note.updated_at, expected_note.updated_at);
}

#[test]
fn test_convert_to_note_with_frontmatter_and_content() {
    let content = String::from("---
id: 1
title: test note
---
This is a test note.
");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content.clone(), default_note);
    assert_eq!(note.id, TinyId::from_u64_unchecked(1));
    assert_eq!(note.title, "test note");
    assert_eq!(note.excerpt, "This is a test note.");
}

#[test]
fn test_convert_to_note_without_frontmatter() {
    let content = String::from("This is a test note without frontmatter.");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content, default_note.clone());

    assert_eq!(note.title, default_note.title);
    assert_eq!(note.excerpt, "This is a test note without frontmatter.");
}

#[test]
fn test_convert_to_note_without_content() {
    let content = String::from("---
id: 1
title: test note
---
");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content, default_note.clone());

    assert_eq!(note.title, "test note");
    assert_eq!(note.excerpt, default_note.excerpt);
}

#[test]
fn test_convert_to_note_all_frontmatter_fields() {
    let content = String::from("---
id: 2
title: Complete Note
excerpt: Complete Excerpt
color: red
starred: true
created_at: 2023-07-30T14:30:00Z
updated_at: 2023-07-30T14:30:00Z
---
This is a complete note.
");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content, default_note);

    assert_eq!(note.id, TinyId::from_u64_unchecked(2));
    assert_eq!(note.title, "Complete Note");
    assert_eq!(note.excerpt, "Complete Excerpt");
    assert_eq!(note.color, "red");
    assert_eq!(note.starred, true);
    assert_eq!(note.created_at, NaiveDateTime::parse_from_str("2023-07-30T14:30:00", "%Y-%m-%dT%H:%M:%S").unwrap());
    assert_eq!(note.updated_at, NaiveDateTime::parse_from_str("2023-07-30T14:30:00", "%Y-%m-%dT%H:%M:%S").unwrap());
}

#[test]
fn test_convert_to_note_no_frontmatter_fields() {
    let content = String::from("This is a note with no frontmatter.");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content.clone(), default_note.clone());

    assert_eq!(note.excerpt, content); // first paragraph is used as excerpt
}

#[test]
fn test_convert_to_note_invalid_frontmatter() {
    let content = String::from("---
invalid_yaml
---
This is a note with invalid frontmatter.
");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content.clone(), default_note.clone());

    assert_eq!(note.excerpt, "This is a note with invalid frontmatter."); // Assuming first paragraph is used as excerpt
}

#[test]
fn test_convert_to_note_frontmatter_overrides_defaults() {
    let content = String::from("---
title: Overridden title
---
This is a note with overridden title.
");
    let default_note: NoteDTO = Default::default();

    let note = convert_to_note(content, default_note.clone());

    assert_ne!(note.title, default_note.title);
    assert_eq!(note.title, "Overridden title");
}

#[test]
fn test_convert_to_note_with_large_content() {
    let content = String::from("---
id: 69
title: test note
---
# Heading1
This is the first paragraph of the note.

## Heading2
Here is the second paragraph.

### Heading3
And this is the third paragraph.
");

    let default_note: NoteDTO = Default::default();
    let note = convert_to_note(content, default_note);
    
    assert_eq!(note.id, TinyId::from_u64_unchecked(69));
    assert_eq!(note.title, "test note");
    assert_eq!(note.excerpt,"This is the first paragraph of the note.");
}

#[test]
fn test_convert_to_note_with_lists() {
    let content = String::from("---
id: 3
title: Note with Lists
---
# Shopping List
- Apples
- Oranges
- Bananas

# Todo List
1. Take out the trash
2. Buy groceries
3. Cook dinner
    ");
    let default_note: NoteDTO = Default::default();
    let note = convert_to_note(content, default_note);

    // Check if the list elements are preserved in the content
    assert!(note.content.contains("<li>Apples</li>"));
    assert!(note.content.contains("<li>Oranges</li>"));
    assert!(note.content.contains("<li>Bananas</li>"));
    assert!(note.content.contains("<li>Take out the trash</li>"));
    assert!(note.content.contains("<li>Buy groceries</li>"));
    assert!(note.content.contains("<li>Cook dinner</li>"));
}

#[test]
fn test_convert_to_note_with_code_blocks() {
    let content = String::from("---
id: 4
title: Note with Code Blocks
---
Here is a function in Python:

```python
def hello_world():
    print(\"Hello, world!\")
```
");
    let default_note: NoteDTO = Default::default();
    let note = convert_to_note(content, default_note);
    // Check if the code block is preserved in the content
    assert!(note.content.contains("<code class=\"language-python\">def hello_world():"));
    assert!(note.content.contains("print(\"Hello, world!\")\n</code>"));
}

#[test]
fn test_convert_to_note_with_bold_and_italic_text() {
let content = String::from("---
id: 6
title: Note with Bold and Italic Text
---
This is **bold** text.

This is *italic* text.
");
    let default_note: NoteDTO = Default::default();
    let note = convert_to_note(content, default_note);
    // Check if the bold and italic text are preserved in the content
    println!("{}", note.content);
    assert!(note.content.contains("<strong>bold</strong>"));
    assert!(note.content.contains("<em>italic</em>"));
}

#[test]
fn test_convert_to_note_with_links() {
let content = String::from("---
id: 5
title: Note with Links
---
Check out this [Link](https://www.example.com) for more information.
");
    let default_note: NoteDTO = Default::default();
    let note = convert_to_note(content, default_note);
    // Check if the link is preserved in the content
    println!("{}", note.content);
    assert!(note.content.contains("<a href=\"https://www.example.com\" rel=\"noopener noreferrer\">Link</a>"));
}