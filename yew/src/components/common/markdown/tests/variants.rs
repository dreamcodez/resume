use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::markdown::Markdown;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_markdown_variant_basic_text() {
    let content = "This is basic text content.";

    let html = render_markdown_html(content);

    // Should render basic text with proper styling
    assert!(html.contains("This is basic text content"));
    assert!(html.contains("text-gray-700"));
    assert!(html.contains("leading-relaxed"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_headings_only() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;

    let html = render_markdown_html(content);

    // Should render all heading levels with proper styling
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1
    assert!(html.contains("text-xl font-semibold mb-3 text-gray-900")); // h2
    assert!(html.contains("text-lg font-medium mb-2 text-gray-900")); // h3
    assert!(html.contains("text-base font-medium mb-2 text-gray-900")); // h4
    assert!(html.contains("text-sm font-medium mb-1 text-gray-900")); // h5
    assert!(html.contains("text-xs font-medium mb-1 text-gray-900")); // h6
}

#[wasm_bindgen_test]
fn test_markdown_variant_lists_only() {
    let content = r#"- Unordered item 1
- Unordered item 2
- Unordered item 3

1. Ordered item 1
2. Ordered item 2
3. Ordered item 3"#;

    let html = render_markdown_html(content);

    // Should render lists with proper styling
    assert!(html.contains("<ul class=\"ml-4 mb-2\">"));
    assert!(html.contains("<li class=\"mb-1\">"));
    assert!(html.contains("<ol>"));
    assert!(html.contains("Unordered item 1"));
    assert!(html.contains("Unordered item 2"));
    assert!(html.contains("Unordered item 3"));
    assert!(html.contains("Ordered item 1"));
    assert!(html.contains("Ordered item 2"));
    assert!(html.contains("Ordered item 3"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_code_only() {
    let content = r#"Inline `code` text.

```rust
fn main() {
    println!("Hello, World!");
}
```

More inline `code`."#;

    let html = render_markdown_html(content);

    // Should render code with proper styling
    assert!(html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // inline code
    assert!(html.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2")); // code block
    assert!(html.contains("fn main()"));
    assert!(html.contains("println!"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_links_only() {
    let content = r#"[External Link](https://example.com)
[Internal Link](/about)
[Link with Title](https://example.com "Link Title")"#;

    let html = render_markdown_html(content);

    // Should render links with proper styling
    assert!(html.contains("text-blue-600 hover:text-blue-800 underline"));
    assert!(html.contains("href=\"https://example.com\""));
    assert!(html.contains("href=\"/about\""));
    assert!(html.contains("External Link"));
    assert!(html.contains("Internal Link"));
    assert!(html.contains("Link with Title"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_tables_only() {
    let content = r#"| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |"#;

    let html = render_markdown_html(content);

    // Should render tables with proper styling
    assert!(html.contains("border-collapse border border-gray-300 mb-2"));
    assert!(html.contains("border border-gray-300 px-3 py-2 bg-gray-100 font-semibold")); // headers
    assert!(html.contains("border border-gray-300 px-3 py-2")); // cells
    assert!(html.contains("Header 1"));
    assert!(html.contains("Header 2"));
    assert!(html.contains("Header 3"));
    assert!(html.contains("Cell 1"));
    assert!(html.contains("Cell 2"));
    assert!(html.contains("Cell 3"));
    assert!(html.contains("Cell 4"));
    assert!(html.contains("Cell 5"));
    assert!(html.contains("Cell 6"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_blockquotes_only() {
    let content = r#"> This is a single line blockquote.

> This is a multi-line
> blockquote with
> multiple lines.

> Blockquote with **bold** and *italic* text."#;

    let html = render_markdown_html(content);

    // Should render blockquotes with proper styling
    assert!(html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2"));
    assert!(html.contains("This is a single line blockquote"));
    assert!(html.contains("This is a multi-line"));
    assert!(html.contains("blockquote with"));
    assert!(html.contains("multiple lines"));
    assert!(html.contains("font-semibold text-gray-900")); // bold in blockquote
    assert!(html.contains("italic text-gray-700")); // italic in blockquote
}

#[wasm_bindgen_test]
fn test_markdown_variant_emphasis_only() {
    let content = r#"This is **bold** text.
This is *italic* text.
This is ***bold and italic*** text.
This is ~~strikethrough~~ text."#;

    let html = render_markdown_html(content);

    // Should render emphasis with proper styling
    assert!(html.contains("font-semibold text-gray-900")); // bold
    assert!(html.contains("italic text-gray-700")); // italic
    assert!(html.contains("<del>")); // strikethrough
    assert!(html.contains("bold"));
    assert!(html.contains("italic"));
    assert!(html.contains("strikethrough"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_task_lists_only() {
    let content = r#"- [x] Completed task 1
- [ ] Pending task 1
- [x] Completed task 2
- [ ] Pending task 2"#;

    let html = render_markdown_html(content);

    // Should render task lists with proper structure
    assert!(html.contains("<input"));
    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("Completed task 1"));
    assert!(html.contains("Pending task 1"));
    assert!(html.contains("Completed task 2"));
    assert!(html.contains("Pending task 2"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_footnotes_only() {
    let content = r#"Here is a sentence with a footnote[^1].

Another sentence with a footnote[^2].

[^1]: This is the first footnote content.
[^2]: This is the second footnote content."#;

    let html = render_markdown_html(content);

    // Should render footnotes with proper structure
    assert!(html.contains("footnote"));
    assert!(html.contains("This is the first footnote content"));
    assert!(html.contains("This is the second footnote content"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_mixed_content() {
    let content = r#"# Mixed Content Document

## Text Formatting
This paragraph contains **bold**, *italic*, and `code` text.

## Lists
- Item 1
- Item 2
  - Nested item
- Item 3

## Code
```rust
fn example() {
    println!("Hello, World!");
}
```

## Links
[Visit our website](https://example.com)

## Tables
| Feature | Status |
|---------|--------|
| Working | ✅     |
| Pending | ⏳     |

## Blockquotes
> This is an important note.

## Task List
- [x] Feature 1
- [ ] Feature 2

## Footnotes
Here is a reference[^1].

[^1]: Reference details."#;

    let html = render_markdown_html(content);

    // Should render all content types together
    assert!(html.contains("Mixed Content Document"));
    assert!(html.contains("font-semibold text-gray-900")); // bold
    assert!(html.contains("italic text-gray-700")); // italic
    assert!(html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // inline code
    assert!(html.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2")); // code block
    assert!(html.contains("Item 1"));
    assert!(html.contains("Nested item"));
    assert!(html.contains("text-blue-600 hover:text-blue-800 underline")); // links
    assert!(html.contains("border-collapse border border-gray-300 mb-2")); // table
    assert!(html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // blockquote
    assert!(html.contains("<input")); // task lists
    assert!(html.contains("footnote")); // footnotes
}

#[wasm_bindgen_test]
fn test_markdown_variant_with_custom_classes() {
    let content = "# Custom Styled Content\n\nThis content has custom classes.";

    let div = document().create_element("div").unwrap();
    let props = crate::components::common::markdown::MarkdownProps {
        content: content.to_string(),
        class: classes!("custom-prose", "prose-lg", "dark:prose-invert"),
    };

    let _rendered = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    let html = div.inner_html();

    // Should apply custom classes
    assert!(html.contains("custom-prose"));
    assert!(html.contains("prose-lg"));
    assert!(html.contains("dark:prose-invert"));
    assert!(html.contains("prose prose-sm max-w-none")); // default classes should still be present
}

#[wasm_bindgen_test]
fn test_markdown_variant_empty_content() {
    let content = "";

    let html = render_markdown_html(content);

    // Should render empty container with proper classes
    assert!(html.contains("prose prose-sm max-w-none"));
    assert!(html.contains("whitespace-pre-line text-gray-700 leading-relaxed"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_whitespace_only() {
    let content = "   \n\t  \n";

    let html = render_markdown_html(content);

    // Should render container even with only whitespace
    assert!(html.contains("prose prose-sm max-w-none"));
    assert!(html.contains("whitespace-pre-line"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_unicode_content() {
    let content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let html = render_markdown_html(content);

    // Should render unicode content properly
    assert!(html.contains("Привет мир!"));
    assert!(html.contains("你好世界!"));
    assert!(html.contains("こんにちは世界!"));
    assert!(html.contains("🌍"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_special_characters() {
    let content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```

> Quote with "nested" quotes"#;

    let html = render_markdown_html(content);

    // Should render special characters properly
    assert!(html.contains("quotes"));
    assert!(html.contains("apostrophes"));
    assert!(html.contains("&"));
    assert!(html.contains("<"));
    assert!(html.contains(">"));
    assert!(html.contains("🚀"));
    assert!(html.contains("🎉"));
    assert!(html.contains("<div class=\"test\">Content</div>"));
}

#[wasm_bindgen_test]
fn test_markdown_variant_nested_structures() {
    let content = r#"# Main Title

## Subtitle

### Sub-subtitle

This paragraph contains **bold** and *italic* text.

- List item with **bold** text
- List item with *italic* text
  - Nested item with `code`
    - Deep nested item

1. Numbered item with **bold**
2. Numbered item with *italic*
   1. Nested numbered item

> Blockquote with **bold** and *italic* text

| Header with **bold** | Header with *italic* |
|----------------------|----------------------|
| Cell with `code`     | Cell with **bold**   |"#;

    let html = render_markdown_html(content);

    // Should render nested structures properly
    assert!(html.contains("Main Title"));
    assert!(html.contains("Subtitle"));
    assert!(html.contains("Sub-subtitle"));
    assert!(html.contains("font-semibold text-gray-900")); // bold in various contexts
    assert!(html.contains("italic text-gray-700")); // italic in various contexts
    assert!(html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // code in various contexts
    assert!(html.contains("List item with"));
    assert!(html.contains("Nested item with"));
    assert!(html.contains("Deep nested item"));
    assert!(html.contains("Numbered item with"));
    assert!(html.contains("Nested numbered item"));
    assert!(html.contains("Blockquote with"));
    assert!(html.contains("Header with"));
    assert!(html.contains("Cell with"));
}

// Helper function to render markdown component and return the HTML string
fn render_markdown_html(content: &str) -> String {
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
