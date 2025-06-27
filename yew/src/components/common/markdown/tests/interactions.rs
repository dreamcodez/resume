use yew::prelude::*;

use crate::components::common::markdown::{parse_markdown_to_html, MarkdownProps};

#[test]
fn test_markdown_props_updates_content() {
    // Initial content
    let initial_props = MarkdownProps {
        content: "# Initial Title\n\nInitial content.".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.contains("Initial Title"));
    assert!(initial_html.contains("Initial content"));

    // Update content
    let updated_props = MarkdownProps {
        content: "# Updated Title\n\nUpdated content.".to_string(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("Updated Title"));
    assert!(updated_html.contains("Updated content"));
    assert!(!updated_html.contains("Initial Title"));
    assert!(!updated_html.contains("Initial content"));
}

#[test]
fn test_markdown_props_updates_classes() {
    // Initial classes
    let initial_props = MarkdownProps {
        content: "# Test Title\n\nTest content.".to_string(),
        class: classes!("initial-class"),
    };

    assert!(initial_props.class.contains("initial-class"));

    // Update classes
    let updated_props = MarkdownProps {
        content: "# Test Title\n\nTest content.".to_string(),
        class: classes!("updated-class", "another-class"),
    };

    assert!(updated_props.class.contains("updated-class"));
    assert!(updated_props.class.contains("another-class"));
    assert!(!updated_props.class.contains("initial-class"));
}

#[test]
fn test_markdown_props_handles_empty_to_content_transition() {
    // Initial empty content
    let initial_props = MarkdownProps {
        content: String::new(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.is_empty() || initial_html.trim().is_empty());

    // Update to content
    let updated_props = MarkdownProps {
        content: "# New Title\n\nNew content.".to_string(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("New Title"));
    assert!(updated_html.contains("New content"));
}

#[test]
fn test_markdown_props_handles_content_to_empty_transition() {
    // Initial content
    let initial_props = MarkdownProps {
        content: "# Initial Title\n\nInitial content.".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.contains("Initial Title"));
    assert!(initial_html.contains("Initial content"));

    // Update to empty content
    let updated_props = MarkdownProps {
        content: String::new(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(!updated_html.contains("Initial Title"));
    assert!(!updated_html.contains("Initial content"));
}

#[test]
fn test_markdown_props_handles_whitespace_transitions() {
    // Initial whitespace content
    let initial_props = MarkdownProps {
        content: "   \n\t  \n".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    // Whitespace-only content should result in empty or minimal HTML
    assert!(initial_html.trim().is_empty() || initial_html.contains("<p>"));

    // Update to content
    let updated_props = MarkdownProps {
        content: "# Title\n\nContent.".to_string(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("Title"));
    assert!(updated_html.contains("Content"));
}

#[test]
fn test_markdown_props_handles_complex_content_updates() {
    // Initial simple content
    let initial_props = MarkdownProps {
        content: "# Simple Title\n\nSimple content.".to_string(),
        class: classes!("simple-class"),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.contains("Simple Title"));
    assert!(initial_html.contains("Simple content"));
    assert!(initial_props.class.contains("simple-class"));

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

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("Complex Title"));
    assert!(updated_html.contains("Subtitle"));
    assert!(updated_html.contains("<strong")); // bold
    assert!(updated_html.contains("<em")); // italic
    assert!(updated_html.contains("<code")); // code
    assert!(updated_html.contains("List item 1"));
    assert!(updated_html.contains("Nested item"));
    assert!(updated_html.contains("fn main()"));
    assert!(updated_html.contains("<blockquote")); // blockquote
    assert!(updated_html.contains("<a")); // links
    assert!(updated_html.contains("<table")); // table
    assert!(updated_props.class.contains("complex-class"));
    assert!(updated_props.class.contains("another-class"));
    assert!(!updated_props.class.contains("simple-class"));
}

#[test]
fn test_markdown_props_handles_unicode_content_updates() {
    // Initial ASCII content
    let initial_props = MarkdownProps {
        content: "# ASCII Title\n\nASCII content.".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.contains("ASCII Title"));
    assert!(initial_html.contains("ASCII content"));

    // Update to unicode content
    let unicode_content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let updated_props = MarkdownProps {
        content: unicode_content.to_string(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("Привет мир!"));
    assert!(updated_html.contains("你好世界!"));
    assert!(updated_html.contains("こんにちは世界!"));
    assert!(updated_html.contains("🌍"));
}

#[test]
fn test_markdown_props_handles_special_character_updates() {
    // Initial normal content
    let initial_props = MarkdownProps {
        content: "# Normal Title\n\nNormal content.".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
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

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("quotes"));
    assert!(updated_html.contains("apostrophes"));
    assert!(updated_html.contains("&amp;"));
    assert!(updated_html.contains("&lt;"));
    assert!(updated_html.contains("&gt;"));
    assert!(updated_html.contains("🚀"));
    assert!(updated_html.contains("🎉"));
    // HTML in code blocks is also escaped
    assert!(updated_html.contains("&lt;div class=\"test\"&gt;Content&lt;/div&gt;"));
    assert!(!updated_html.contains("Normal Title"));
    assert!(!updated_html.contains("Normal content"));
}

#[test]
fn test_markdown_props_handles_multiple_rapid_updates() {
    // Perform multiple rapid updates
    for i in 1..=5 {
        let props = MarkdownProps {
            content: format!("# Update {}\n\nContent for update {}.", i, i),
            class: classes!(format!("update-class-{}", i)),
        };

        let html = parse_markdown_to_html(&props.content);

        // Should show the current update
        assert!(html.contains(&format!("Update {}", i)));
        assert!(html.contains(&format!("Content for update {}", i)));
        assert!(props.class.contains(&format!("update-class-{}", i)));
    }
}

#[test]
fn test_markdown_props_handles_large_content_updates() {
    // Initial small content
    let initial_props = MarkdownProps {
        content: "# Small Title\n\nSmall content.".to_string(),
        class: Classes::new(),
    };

    let initial_html = parse_markdown_to_html(&initial_props.content);
    assert!(initial_html.contains("Small Title"));
    assert!(initial_html.contains("Small content"));

    // Update to large content
    let large_content = "# ".to_string() + &"A".repeat(1000) + "\n\n" + &"B".repeat(1000);

    let updated_props = MarkdownProps {
        content: large_content.clone(),
        class: Classes::new(),
    };

    let updated_html = parse_markdown_to_html(&updated_props.content);
    assert!(updated_html.contains("A"));
    assert!(updated_html.contains("B"));
    assert!(!updated_html.contains("Small Title"));
    assert!(!updated_html.contains("Small content"));
}
