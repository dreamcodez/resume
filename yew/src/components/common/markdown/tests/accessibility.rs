use yew::prelude::*;

use crate::components::common::markdown::parse_markdown_to_html;

#[test]
fn test_markdown_has_semantic_structure() {
    let content = "# Main Title\n\n## Subtitle\n\nThis is a paragraph.";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("prose"));
    assert!(html.contains("prose-sm"));
    assert!(html.contains("max-w-none"));
}

#[test]
fn test_markdown_headings_have_proper_structure() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<h1"));
    assert!(html.contains("<h2"));
    assert!(html.contains("<h3"));
    assert!(html.contains("<h4"));
    assert!(html.contains("<h5"));
    assert!(html.contains("<h6"));
    assert!(html.contains("H1 Title"));
    assert!(html.contains("H2 Title"));
    assert!(html.contains("H3 Title"));
    assert!(html.contains("H4 Title"));
    assert!(html.contains("H5 Title"));
    assert!(html.contains("H6 Title"));
}

#[test]
fn test_markdown_lists_have_proper_structure() {
    let content = r#"- Item 1
- Item 2
- Item 3

1. Numbered item 1
2. Numbered item 2"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<ul"));
    assert!(html.contains("<ol"));
    assert!(html.contains("<li"));
    assert!(html.contains("Item 1"));
    assert!(html.contains("Item 2"));
    assert!(html.contains("Item 3"));
    assert!(html.contains("Numbered item 1"));
    assert!(html.contains("Numbered item 2"));
}

#[test]
fn test_markdown_links_have_proper_attributes() {
    let content = "[Link Text](https://example.com)";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("href=\"https://example.com\""));
    assert!(html.contains("Link Text"));
    assert!(html.contains("text-blue-600"));
    assert!(html.contains("hover:text-blue-800"));
    assert!(html.contains("underline"));
}

#[test]
fn test_markdown_code_has_proper_structure() {
    let content = r#"Inline `code` and code blocks:

```rust
fn main() {
    println!(\"Hello, World!\");
}
```"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<code"));
    assert!(html.contains("<pre"));
    assert!(html.contains("fn main()"));
    assert!(html.contains("println!"));
    assert!(html.contains("bg-gray-100"));
    assert!(html.contains("font-mono"));
}

#[test]
fn test_markdown_tables_have_proper_structure() {
    let content = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<table"));
    assert!(html.contains("<thead"));
    assert!(html.contains("<tbody"));
    assert!(html.contains("<th"));
    assert!(html.contains("<td"));
    assert!(html.contains("Header 1"));
    assert!(html.contains("Header 2"));
    assert!(html.contains("Cell 1"));
    assert!(html.contains("Cell 2"));
    assert!(html.contains("Cell 3"));
    assert!(html.contains("Cell 4"));
}

#[test]
fn test_markdown_blockquotes_have_proper_structure() {
    let content = "> This is a blockquote\n> with multiple lines";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<blockquote"));
    assert!(html.contains("This is a blockquote"));
    assert!(html.contains("with multiple lines"));
    assert!(html.contains("border-l-4"));
    assert!(html.contains("border-gray-300"));
    assert!(html.contains("italic"));
}

#[test]
fn test_markdown_task_lists_have_proper_structure() {
    let content = r#"- [x] Completed task
- [ ] Pending task
- [ ] Another pending task"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<input"));
    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("Completed task"));
    assert!(html.contains("Pending task"));
    assert!(html.contains("Another pending task"));
}

#[test]
fn test_markdown_emphasis_has_proper_structure() {
    let content = "This is **bold** text and *italic* text.";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<strong") || html.contains("font-semibold"));
    assert!(html.contains("<em") || html.contains("italic"));
    assert!(html.contains("bold"));
    assert!(html.contains("italic"));
}

#[test]
fn test_markdown_strikethrough_has_proper_structure() {
    let content = "~~Strikethrough text~~";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("<del") || html.contains("<s") || html.contains("<strike"));
    assert!(html.contains("Strikethrough text"));
}

#[test]
fn test_markdown_footnotes_have_proper_structure() {
    let content = r#"Here is a sentence with a footnote[^1].

[^1]: This is the footnote content."#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("footnote"));
    assert!(html.contains("This is the footnote content"));
}

#[test]
fn test_markdown_has_proper_text_contrast() {
    let content = "# Title\n\nThis is regular text.";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("text-gray-900"));
    assert!(html.contains("text-gray-700"));
}

#[test]
fn test_markdown_has_proper_spacing() {
    let content = "# Title\n\nParagraph 1.\n\nParagraph 2.";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("mb-2"));
    assert!(html.contains("mb-4"));
    assert!(html.contains("leading-relaxed"));
}

#[test]
fn test_markdown_handles_empty_content_accessibly() {
    let content = "";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("prose"));
    assert!(html.contains("prose-sm"));
    assert!(html.contains("max-w-none"));
}

#[test]
fn test_markdown_has_proper_whitespace_handling() {
    let content = "   \n\t  \n";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("whitespace-pre-line"));
}

#[test]
fn test_markdown_has_proper_overflow_handling() {
    let content = r#"```rust
fn very_long_function_name_with_many_parameters(
    param1: String,
    param2: String,
    param3: String,
    param4: String,
    param5: String,
    param6: String,
    param7: String,
    param8: String,
    param9: String,
    param10: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Function implementation
    Ok(())
}
```"#;
    let html = parse_markdown_to_html(content);
    assert!(html.contains("overflow-x-auto"));
}

#[test]
fn test_markdown_has_proper_focus_states() {
    let content = "[Link Text](https://example.com)";
    let html = parse_markdown_to_html(content);
    assert!(html.contains("text-blue-600"));
    assert!(html.contains("hover:text-blue-800"));
}
