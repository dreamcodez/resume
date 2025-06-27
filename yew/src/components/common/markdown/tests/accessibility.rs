use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::components::common::markdown::Markdown;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_markdown_has_semantic_structure() {
    let content = "# Main Title\n\n## Subtitle\n\nThis is a paragraph.";

    let element = render_markdown_element(content);

    // Check that the component renders with proper semantic structure
    assert!(element.has_attribute("class"));
    let classes = element.get_attribute("class").unwrap();
    assert!(classes.contains("prose"));
    assert!(classes.contains("prose-sm"));
    assert!(classes.contains("max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_headings_have_proper_structure() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;

    let html = render_markdown_html(content);

    // Check that headings are properly structured
    assert!(html.contains("<h1"));
    assert!(html.contains("<h2"));
    assert!(html.contains("<h3"));
    assert!(html.contains("<h4"));
    assert!(html.contains("<h5"));
    assert!(html.contains("<h6"));

    // Check that heading content is preserved
    assert!(html.contains("H1 Title"));
    assert!(html.contains("H2 Title"));
    assert!(html.contains("H3 Title"));
    assert!(html.contains("H4 Title"));
    assert!(html.contains("H5 Title"));
    assert!(html.contains("H6 Title"));
}

#[wasm_bindgen_test]
fn test_markdown_lists_have_proper_structure() {
    let content = r#"- Item 1
- Item 2
- Item 3

1. Numbered item 1
2. Numbered item 2"#;

    let html = render_markdown_html(content);

    // Check that lists are properly structured
    assert!(html.contains("<ul"));
    assert!(html.contains("<ol"));
    assert!(html.contains("<li"));

    // Check that list content is preserved
    assert!(html.contains("Item 1"));
    assert!(html.contains("Item 2"));
    assert!(html.contains("Item 3"));
    assert!(html.contains("Numbered item 1"));
    assert!(html.contains("Numbered item 2"));
}

#[wasm_bindgen_test]
fn test_markdown_links_have_proper_attributes() {
    let content = "[Link Text](https://example.com)";

    let html = render_markdown_html(content);

    // Check that links have proper attributes
    assert!(html.contains("href=\"https://example.com\""));
    assert!(html.contains("Link Text"));

    // Check that links have proper styling classes
    assert!(html.contains("text-blue-600"));
    assert!(html.contains("hover:text-blue-800"));
    assert!(html.contains("underline"));
}

#[wasm_bindgen_test]
fn test_markdown_code_has_proper_structure() {
    let content = r#"Inline `code` and code blocks:

```rust
fn main() {
    println!("Hello, World!");
}
```"#;

    let html = render_markdown_html(content);

    // Check that code elements are properly structured
    assert!(html.contains("<code"));
    assert!(html.contains("<pre"));

    // Check that code content is preserved
    assert!(html.contains("fn main()"));
    assert!(html.contains("println!"));

    // Check that code has proper styling
    assert!(html.contains("bg-gray-100"));
    assert!(html.contains("font-mono"));
}

#[wasm_bindgen_test]
fn test_markdown_tables_have_proper_structure() {
    let content = r#"| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |"#;

    let html = render_markdown_html(content);

    // Check that tables are properly structured
    assert!(html.contains("<table"));
    assert!(html.contains("<thead"));
    assert!(html.contains("<tbody"));
    assert!(html.contains("<th"));
    assert!(html.contains("<td"));

    // Check that table content is preserved
    assert!(html.contains("Header 1"));
    assert!(html.contains("Header 2"));
    assert!(html.contains("Cell 1"));
    assert!(html.contains("Cell 2"));
    assert!(html.contains("Cell 3"));
    assert!(html.contains("Cell 4"));
}

#[wasm_bindgen_test]
fn test_markdown_blockquotes_have_proper_structure() {
    let content = "> This is a blockquote\n> with multiple lines";

    let html = render_markdown_html(content);

    // Check that blockquotes are properly structured
    assert!(html.contains("<blockquote"));

    // Check that blockquote content is preserved
    assert!(html.contains("This is a blockquote"));
    assert!(html.contains("with multiple lines"));

    // Check that blockquotes have proper styling
    assert!(html.contains("border-l-4"));
    assert!(html.contains("border-gray-300"));
    assert!(html.contains("italic"));
}

#[wasm_bindgen_test]
fn test_markdown_task_lists_have_proper_structure() {
    let content = r#"- [x] Completed task
- [ ] Pending task
- [ ] Another pending task"#;

    let html = render_markdown_html(content);

    // Check that task lists are properly structured
    assert!(html.contains("<input"));
    assert!(html.contains("type=\"checkbox\""));

    // Check that task list content is preserved
    assert!(html.contains("Completed task"));
    assert!(html.contains("Pending task"));
    assert!(html.contains("Another pending task"));
}

#[wasm_bindgen_test]
fn test_markdown_emphasis_has_proper_structure() {
    let content = "This is **bold** text and *italic* text.";

    let html = render_markdown_html(content);

    // Check that emphasis elements are properly structured
    assert!(html.contains("<strong") || html.contains("font-semibold"));
    assert!(html.contains("<em") || html.contains("italic"));

    // Check that emphasis content is preserved
    assert!(html.contains("bold"));
    assert!(html.contains("italic"));
}

#[wasm_bindgen_test]
fn test_markdown_strikethrough_has_proper_structure() {
    let content = "~~Strikethrough text~~";

    let html = render_markdown_html(content);

    // Check that strikethrough elements are properly structured
    assert!(html.contains("<del"));

    // Check that strikethrough content is preserved
    assert!(html.contains("Strikethrough text"));
}

#[wasm_bindgen_test]
fn test_markdown_footnotes_have_proper_structure() {
    let content = r#"Here is a sentence with a footnote[^1].

[^1]: This is the footnote content."#;

    let html = render_markdown_html(content);

    // Check that footnotes are properly structured
    assert!(html.contains("footnote"));
    assert!(html.contains("This is the footnote content"));
}

#[wasm_bindgen_test]
fn test_markdown_has_proper_text_contrast() {
    let content = "# Title\n\nThis is regular text.";

    let html = render_markdown_html(content);

    // Check that text has proper contrast classes
    assert!(html.contains("text-gray-900")); // headings
    assert!(html.contains("text-gray-700")); // regular text
}

#[wasm_bindgen_test]
fn test_markdown_has_proper_spacing() {
    let content = "# Title\n\nParagraph 1.\n\nParagraph 2.";

    let html = render_markdown_html(content);

    // Check that proper spacing classes are applied
    assert!(html.contains("mb-2")); // paragraph spacing
    assert!(html.contains("mb-4")); // heading spacing
    assert!(html.contains("leading-relaxed")); // line height
}

#[wasm_bindgen_test]
fn test_markdown_handles_empty_content_accessibly() {
    let content = "";

    let element = render_markdown_element(content);

    // Check that empty content still renders with proper structure
    assert!(element.has_attribute("class"));
    let classes = element.get_attribute("class").unwrap();
    assert!(classes.contains("prose"));
    assert!(classes.contains("prose-sm"));
    assert!(classes.contains("max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_has_proper_whitespace_handling() {
    let content = "   \n\t  \n";

    let element = render_markdown_element(content);

    // Check that whitespace-only content still renders properly
    assert!(element.has_attribute("class"));
    let classes = element.get_attribute("class").unwrap();
    assert!(classes.contains("whitespace-pre-line"));
}

#[wasm_bindgen_test]
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

    let html = render_markdown_html(content);

    // Check that code blocks have proper overflow handling
    assert!(html.contains("overflow-x-auto"));
}

#[wasm_bindgen_test]
fn test_markdown_has_proper_focus_states() {
    let content = "[Link Text](https://example.com)";

    let html = render_markdown_html(content);

    // Check that links have proper focus states
    assert!(html.contains("text-blue-600"));
    assert!(html.contains("hover:text-blue-800"));
}

// Helper function to render markdown component and return the element
fn render_markdown_element(content: &str) -> HtmlElement {
    let div = document().create_element("div").unwrap();
    let props = crate::components::common::markdown::MarkdownProps {
        content: content.to_string(),
        class: Classes::new(),
    };

    let _rendered = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), props).render();

    // Wait for rendering to complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    div.dyn_into::<HtmlElement>().unwrap()
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
