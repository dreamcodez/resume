use yew::prelude::*;

use crate::components::common::markdown::parse_markdown_to_html;

#[test]
fn test_markdown_renders_basic_content() {
    let content = "# Hello World\n\nThis is a paragraph.";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("Hello World"));
    assert!(rendered.contains("This is a paragraph"));
    assert!(rendered.contains("<h1")); // h1 structure
}

#[test]
fn test_markdown_renders_headings() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<h1")); // h1
    assert!(rendered.contains("<h2")); // h2
    assert!(rendered.contains("<h3")); // h3
    assert!(rendered.contains("<h4")); // h4
    assert!(rendered.contains("<h5")); // h5
    assert!(rendered.contains("<h6")); // h6
}

#[test]
fn test_markdown_renders_bold_and_italic() {
    let content = "This is **bold** text and *italic* text.";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<strong")); // bold structure
    assert!(rendered.contains("<em")); // italic structure
}

#[test]
fn test_markdown_renders_lists() {
    let content = r#"- Item 1
- Item 2
- Item 3

1. Numbered item 1
2. Numbered item 2"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<ul"));
    assert!(rendered.contains("<li"));
    assert!(rendered.contains("Item 1"));
    assert!(rendered.contains("Item 2"));
    assert!(rendered.contains("Item 3"));
    assert!(rendered.contains("Numbered item 1"));
    assert!(rendered.contains("Numbered item 2"));
}

#[test]
fn test_markdown_renders_links() {
    let content = "[Link Text](https://example.com)";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("href=\"https://example.com\""));
    assert!(rendered.contains("<a"));
    assert!(rendered.contains("Link Text"));
}

#[test]
fn test_markdown_renders_code() {
    let content = r#"Inline `code` and code blocks:

```rust
fn main() {
    println!(\"Hello, World!\");
}
```"#;
    let rendered = parse_markdown_to_html(content);
    // Inline code
    assert!(rendered.contains("<code"));
    // Code blocks
    assert!(rendered.contains("<pre"));
    assert!(rendered.contains("fn main()"));
    assert!(rendered.contains("println!"));
}

#[test]
fn test_markdown_renders_blockquotes() {
    let content = "> This is a blockquote\n> with multiple lines";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<blockquote"));
    assert!(rendered.contains("This is a blockquote"));
    assert!(rendered.contains("with multiple lines"));
}

#[test]
fn test_markdown_renders_tables() {
    let content = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<table"));
    assert!(rendered.contains("<thead"));
    assert!(rendered.contains("<tbody"));
    assert!(rendered.contains("<th"));
    assert!(rendered.contains("<td"));
    assert!(rendered.contains("Header 1"));
    assert!(rendered.contains("Header 2"));
    assert!(rendered.contains("Cell 1"));
    assert!(rendered.contains("Cell 2"));
}

#[test]
fn test_markdown_renders_strikethrough() {
    let content = "~~Strikethrough text~~";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("<del>"));
    assert!(rendered.contains("</del>"));
    assert!(rendered.contains("Strikethrough text"));
}

#[test]
fn test_markdown_renders_task_lists() {
    let content = r#"- [x] Completed task
- [ ] Pending task
- [ ] Another pending task"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("Completed task"));
    assert!(rendered.contains("Pending task"));
    assert!(rendered.contains("Another pending task"));
    assert!(rendered.contains("<input"));
    assert!(rendered.contains("type=\"checkbox\""));
}

#[test]
fn test_markdown_renders_footnotes() {
    let content = r#"Here is a sentence with a footnote[^1].

[^1]: This is the footnote content."#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("footnote"));
    assert!(rendered.contains("This is the footnote content"));
}

#[test]
fn test_markdown_renders_empty_content() {
    let content = "";
    let rendered = parse_markdown_to_html(content);
    // Empty content should return empty string or minimal structure
    assert!(rendered.is_empty() || rendered.contains("<div"));
}

#[test]
fn test_markdown_renders_whitespace_only() {
    let content = "   \n\t  \n";
    let rendered = parse_markdown_to_html(content);
    // Whitespace-only content should return empty string or minimal structure
    assert!(rendered.is_empty() || rendered.contains("<div"));
}

#[test]
fn test_markdown_renders_with_custom_classes() {
    let content = "# Test Title\n\nTest content.";
    // Custom classes are applied at the component level, not in the HTML output of parse_markdown_to_html.
    // This test can be omitted or moved to a component-level test if needed.
    // For now, just check the HTML output for content.
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("Test Title"));
    assert!(rendered.contains("Test content"));
}

#[test]
fn test_markdown_renders_nested_elements() {
    let content = r#"# Title

- **Bold Item**
- *Italic Item*
- [Link Item](https://example.com)
- `Code Item`
- > Blockquote Item
"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("Title"));
    assert!(rendered.contains("Bold Item"));
    assert!(rendered.contains("Italic Item"));
    assert!(rendered.contains("Link Item"));
    assert!(rendered.contains("Code Item"));
    assert!(rendered.contains("Blockquote Item"));
}

#[test]
fn test_markdown_renders_special_characters() {
    let content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```"#;
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("quotes"));
    assert!(rendered.contains("apostrophes"));
    assert!(rendered.contains("&amp;"));
    assert!(rendered.contains("&lt;"));
    assert!(rendered.contains("&gt;"));
    assert!(rendered.contains("🚀"));
    assert!(rendered.contains("🎉"));
    // HTML in code blocks is also escaped
    assert!(rendered.contains("&lt;div class=\"test\"&gt;Content&lt;/div&gt;"));
}

#[test]
fn test_markdown_renders_unicode_content() {
    let content = "Привет мир! 🌍 你好世界! こんにちは世界!";
    let rendered = parse_markdown_to_html(content);
    assert!(rendered.contains("Привет мир!"));
    assert!(rendered.contains("你好世界!"));
    assert!(rendered.contains("こんにちは世界!"));
    assert!(rendered.contains("🌍"));
}
