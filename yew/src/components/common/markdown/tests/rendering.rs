use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::markdown::Markdown;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_markdown_renders_basic_content() {
    let content = "# Hello World\n\nThis is a paragraph.";

    let rendered = render_markdown(content);

    assert!(rendered.contains("Hello World"));
    assert!(rendered.contains("This is a paragraph"));
    assert!(rendered.contains("text-2xl font-bold")); // h1 styling
}

#[wasm_bindgen_test]
fn test_markdown_renders_headings() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("text-2xl font-bold")); // h1
    assert!(rendered.contains("text-xl font-semibold")); // h2
    assert!(rendered.contains("text-lg font-medium")); // h3
    assert!(rendered.contains("text-base font-medium")); // h4
    assert!(rendered.contains("text-sm font-medium")); // h5
    assert!(rendered.contains("text-xs font-medium")); // h6
}

#[wasm_bindgen_test]
fn test_markdown_renders_bold_and_italic() {
    let content = "This is **bold** text and *italic* text.";

    let rendered = render_markdown(content);

    assert!(rendered.contains("font-semibold text-gray-900")); // bold styling
    assert!(rendered.contains("italic text-gray-700")); // italic styling
}

#[wasm_bindgen_test]
fn test_markdown_renders_lists() {
    let content = r#"- Item 1
- Item 2
- Item 3

1. Numbered item 1
2. Numbered item 2"#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("<ul class=\"ml-4 mb-2\">"));
    assert!(rendered.contains("<li class=\"mb-1\">"));
    assert!(rendered.contains("Item 1"));
    assert!(rendered.contains("Item 2"));
    assert!(rendered.contains("Item 3"));
    assert!(rendered.contains("Numbered item 1"));
    assert!(rendered.contains("Numbered item 2"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_links() {
    let content = "[Link Text](https://example.com)";

    let rendered = render_markdown(content);

    assert!(rendered.contains("href=\"https://example.com\""));
    assert!(rendered.contains("text-blue-600 hover:text-blue-800 underline"));
    assert!(rendered.contains("Link Text"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_code() {
    let content = r#"Inline `code` and code blocks:

```rust
fn main() {
    println!("Hello, World!");
}
```"#;

    let rendered = render_markdown(content);

    // Inline code
    assert!(rendered.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono"));

    // Code blocks
    assert!(rendered.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2"));
    assert!(rendered.contains("fn main()"));
    assert!(rendered.contains("println!"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_blockquotes() {
    let content = "> This is a blockquote\n> with multiple lines";

    let rendered = render_markdown(content);

    assert!(rendered.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2"));
    assert!(rendered.contains("This is a blockquote"));
    assert!(rendered.contains("with multiple lines"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_tables() {
    let content = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |"#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("border-collapse border border-gray-300 mb-2"));
    assert!(rendered.contains("border border-gray-300 px-3 py-2 bg-gray-100 font-semibold"));
    assert!(rendered.contains("border border-gray-300 px-3 py-2"));
    assert!(rendered.contains("Header 1"));
    assert!(rendered.contains("Header 2"));
    assert!(rendered.contains("Cell 1"));
    assert!(rendered.contains("Cell 2"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_strikethrough() {
    let content = "~~Strikethrough text~~";

    let rendered = render_markdown(content);

    assert!(rendered.contains("<del>"));
    assert!(rendered.contains("</del>"));
    assert!(rendered.contains("Strikethrough text"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_task_lists() {
    let content = r#"- [x] Completed task
- [ ] Pending task
- [ ] Another pending task"#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("Completed task"));
    assert!(rendered.contains("Pending task"));
    assert!(rendered.contains("Another pending task"));
    assert!(rendered.contains("<input"));
    assert!(rendered.contains("type=\"checkbox\""));
}

#[wasm_bindgen_test]
fn test_markdown_renders_footnotes() {
    let content = r#"Here is a sentence with a footnote[^1].

[^1]: This is the footnote content."#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("footnote"));
    assert!(rendered.contains("This is the footnote content"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_empty_content() {
    let content = "";

    let rendered = render_markdown(content);

    // Should render empty div with proper classes
    assert!(rendered.contains("prose prose-sm max-w-none"));
    assert!(rendered.contains("whitespace-pre-line text-gray-700 leading-relaxed"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_whitespace_only() {
    let content = "   \n\t  \n";

    let rendered = render_markdown(content);

    // Should still render the container
    assert!(rendered.contains("prose prose-sm max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_with_custom_classes() {
    let content = "# Test Title\n\nTest content.";

    let div = document().create_element("div").unwrap();
    let props = crate::components::common::markdown::MarkdownProps {
        content: content.to_string(),
        class: classes!("custom-class", "another-class"),
    };

    let _rendered = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    let html = div.inner_html();

    assert!(html.contains("custom-class"));
    assert!(html.contains("another-class"));
    assert!(html.contains("prose prose-sm max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_nested_elements() {
    let content = r#"# Main Title

## Subtitle

This paragraph contains **bold** and *italic* text with `inline code`.

- List item with **bold** text
- List item with *italic* text
  - Nested item with `code`

> Blockquote with **bold** and *italic* text"#;

    let rendered = render_markdown(content);

    // Check that nested styling is applied correctly
    assert!(rendered.contains("font-semibold text-gray-900")); // bold in list
    assert!(rendered.contains("italic text-gray-700")); // italic in list
    assert!(rendered.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // code in list
    assert!(rendered.contains("font-semibold text-gray-900")); // bold in blockquote
    assert!(rendered.contains("italic text-gray-700")); // italic in blockquote
}

#[wasm_bindgen_test]
fn test_markdown_renders_special_characters() {
    let content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```"#;

    let rendered = render_markdown(content);

    assert!(rendered.contains("quotes"));
    assert!(rendered.contains("apostrophes"));
    assert!(rendered.contains("&"));
    assert!(rendered.contains("<"));
    assert!(rendered.contains(">"));
    assert!(rendered.contains("🚀"));
    assert!(rendered.contains("🎉"));
    assert!(rendered.contains("<div class=\"test\">Content</div>"));
}

#[wasm_bindgen_test]
fn test_markdown_renders_unicode_content() {
    let content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let rendered = render_markdown(content);

    assert!(rendered.contains("Привет мир!"));
    assert!(rendered.contains("你好世界!"));
    assert!(rendered.contains("こんにちは世界!"));
    assert!(rendered.contains("🌍"));
}

// Helper function to render markdown component and return the HTML string
fn render_markdown(content: &str) -> String {
    let div = document().create_element("div").unwrap();
    let props = crate::components::common::markdown::MarkdownProps {
        content: content.to_string(),
        class: Classes::new(),
    };

    let _rendered = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    div.inner_html()
}
