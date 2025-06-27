use gloo_utils::document;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use yew::prelude::*;

use crate::components::common::markdown::{Markdown, MarkdownProps};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_markdown_updates_content() {
    let div = document().create_element("div").unwrap();

    // Initial content
    let initial_props = MarkdownProps {
        content: "# Initial Title\n\nInitial content.".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("Initial Title"));
    assert!(initial_html.contains("Initial content"));

    // Update content
    let updated_props = MarkdownProps {
        content: "# Updated Title\n\nUpdated content.".to_string(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("Updated Title"));
    assert!(updated_html.contains("Updated content"));
    assert!(!updated_html.contains("Initial Title"));
    assert!(!updated_html.contains("Initial content"));
}

#[wasm_bindgen_test]
fn test_markdown_updates_classes() {
    let div = document().create_element("div").unwrap();

    // Initial classes
    let initial_props = MarkdownProps {
        content: "# Test Title\n\nTest content.".to_string(),
        class: classes!("initial-class"),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("initial-class"));

    // Update classes
    let updated_props = MarkdownProps {
        content: "# Test Title\n\nTest content.".to_string(),
        class: classes!("updated-class", "another-class"),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("updated-class"));
    assert!(updated_html.contains("another-class"));
    assert!(!updated_html.contains("initial-class"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_empty_to_content_transition() {
    let div = document().create_element("div").unwrap();

    // Initial empty content
    let initial_props = MarkdownProps {
        content: String::new(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("prose prose-sm max-w-none"));
    assert!(initial_html.contains("whitespace-pre-line text-gray-700 leading-relaxed"));

    // Update to content
    let updated_props = MarkdownProps {
        content: "# New Title\n\nNew content.".to_string(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("New Title"));
    assert!(updated_html.contains("New content"));
    assert!(updated_html.contains("prose prose-sm max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_content_to_empty_transition() {
    let div = document().create_element("div").unwrap();

    // Initial content
    let initial_props = MarkdownProps {
        content: "# Initial Title\n\nInitial content.".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("Initial Title"));
    assert!(initial_html.contains("Initial content"));

    // Update to empty content
    let updated_props = MarkdownProps {
        content: String::new(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(!updated_html.contains("Initial Title"));
    assert!(!updated_html.contains("Initial content"));
    assert!(updated_html.contains("prose prose-sm max-w-none"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_whitespace_transitions() {
    let div = document().create_element("div").unwrap();

    // Initial whitespace content
    let initial_props = MarkdownProps {
        content: "   \n\t  \n".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("prose prose-sm max-w-none"));

    // Update to content
    let updated_props = MarkdownProps {
        content: "# Title\n\nContent.".to_string(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("Title"));
    assert!(updated_html.contains("Content"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_complex_content_updates() {
    let div = document().create_element("div").unwrap();

    // Initial simple content
    let initial_props = MarkdownProps {
        content: "# Simple Title\n\nSimple content.".to_string(),
        class: classes!("simple-class"),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("Simple Title"));
    assert!(initial_html.contains("Simple content"));
    assert!(initial_html.contains("simple-class"));

    // Update to complex content
    let complex_content = r#"# Complex Title

## Subtitle

This is **bold** and *italic* text with `code`.

- List item 1
- List item 2
  - Nested item

```rust
fn main() {
    println!("Hello, World!");
}
```

> Blockquote content

[Link text](https://example.com)

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |"#;

    let updated_props = MarkdownProps {
        content: complex_content.to_string(),
        class: classes!("complex-class", "another-class"),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("Complex Title"));
    assert!(updated_html.contains("Subtitle"));
    assert!(updated_html.contains("font-semibold text-gray-900")); // bold
    assert!(updated_html.contains("italic text-gray-700")); // italic
    assert!(updated_html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // code
    assert!(updated_html.contains("List item 1"));
    assert!(updated_html.contains("Nested item"));
    assert!(updated_html.contains("fn main()"));
    assert!(updated_html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // blockquote
    assert!(updated_html.contains("text-blue-600 hover:text-blue-800 underline")); // links
    assert!(updated_html.contains("border-collapse border border-gray-300 mb-2")); // table
    assert!(updated_html.contains("complex-class"));
    assert!(updated_html.contains("another-class"));
    assert!(!updated_html.contains("simple-class"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_unicode_content_updates() {
    let div = document().create_element("div").unwrap();

    // Initial ASCII content
    let initial_props = MarkdownProps {
        content: "# ASCII Title\n\nASCII content.".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("ASCII Title"));
    assert!(initial_html.contains("ASCII content"));

    // Update to unicode content
    let unicode_content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let updated_props = MarkdownProps {
        content: unicode_content.to_string(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("Привет мир!"));
    assert!(updated_html.contains("你好世界!"));
    assert!(updated_html.contains("こんにちは世界!"));
    assert!(updated_html.contains("🌍"));
    assert!(!updated_html.contains("ASCII Title"));
    assert!(!updated_html.contains("ASCII content"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_special_character_updates() {
    let div = document().create_element("div").unwrap();

    // Initial normal content
    let initial_props = MarkdownProps {
        content: "# Normal Title\n\nNormal content.".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("Normal Title"));
    assert!(initial_html.contains("Normal content"));

    // Update to special character content
    let special_content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```"#;

    let updated_props = MarkdownProps {
        content: special_content.to_string(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("quotes"));
    assert!(updated_html.contains("apostrophes"));
    assert!(updated_html.contains("&"));
    assert!(updated_html.contains("<"));
    assert!(updated_html.contains(">"));
    assert!(updated_html.contains("🚀"));
    assert!(updated_html.contains("🎉"));
    assert!(updated_html.contains("<div class=\"test\">Content</div>"));
    assert!(!updated_html.contains("Normal Title"));
    assert!(!updated_html.contains("Normal content"));
}

#[wasm_bindgen_test]
fn test_markdown_handles_multiple_rapid_updates() {
    let div = document().create_element("div").unwrap();

    // Perform multiple rapid updates
    for i in 1..=5 {
        let props = MarkdownProps {
            content: format!("# Update {}\n\nContent for update {}.", i, i),
            class: classes!(format!("update-class-{}", i)),
        };

        let renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), props);
        renderer.render();

        // Small delay between updates
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    // Wait for final rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let final_html = div.inner_html();

    // Should show the last update
    assert!(final_html.contains("Update 5"));
    assert!(final_html.contains("Content for update 5"));
    assert!(final_html.contains("update-class-5"));

    // Should not show previous updates
    for i in 1..5 {
        assert!(!final_html.contains(&format!("Update {}", i)));
        assert!(!final_html.contains(&format!("Content for update {}", i)));
        assert!(!final_html.contains(&format!("update-class-{}", i)));
    }
}

#[wasm_bindgen_test]
fn test_markdown_handles_large_content_updates() {
    let div = document().create_element("div").unwrap();

    // Initial small content
    let initial_props = MarkdownProps {
        content: "# Small Title\n\nSmall content.".to_string(),
        class: Classes::new(),
    };

    let mut renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), initial_props);
    renderer.render();

    // Wait for initial rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let initial_html = div.inner_html();
    assert!(initial_html.contains("Small Title"));
    assert!(initial_html.contains("Small content"));

    // Update to large content
    let large_content = "# ".to_string() + &"A".repeat(1000) + "\n\n" + &"B".repeat(1000);

    let updated_props = MarkdownProps {
        content: large_content.clone(),
        class: Classes::new(),
    };

    renderer = yew::Renderer::<Markdown>::with_root_and_props(div.clone(), updated_props);
    renderer.render();

    // Wait for update rendering
    std::thread::sleep(std::time::Duration::from_millis(100));

    let updated_html = div.inner_html();
    assert!(updated_html.contains("A"));
    assert!(updated_html.contains("B"));
    assert!(!updated_html.contains("Small Title"));
    assert!(!updated_html.contains("Small content"));
}
