use yew::prelude::*;

use crate::components::common::markdown::MarkdownProps;

#[test]
fn test_markdown_props_default_values() {
    let props = MarkdownProps::default();

    assert_eq!(props.content, "");
    assert!(props.class.is_empty());
}

#[test]
fn test_markdown_props_custom_values() {
    let props = MarkdownProps {
        content: "# Hello World\n\nThis is **bold** text.".to_string(),
        class: classes!("custom-class", "another-class"),
    };

    assert_eq!(props.content, "# Hello World\n\nThis is **bold** text.");
    assert!(!props.class.is_empty());
    assert!(props.class.contains("custom-class"));
    assert!(props.class.contains("another-class"));
}

#[test]
fn test_markdown_props_partial_eq() {
    let props1 = MarkdownProps {
        content: "# Test Content".to_string(),
        class: classes!("test-class"),
    };

    let props2 = MarkdownProps {
        content: "# Test Content".to_string(),
        class: classes!("test-class"),
    };

    let props3 = MarkdownProps {
        content: "# Different Content".to_string(),
        class: classes!("different-class"),
    };

    assert_eq!(props1.content, props2.content);
    assert_eq!(props1.class, props2.class);
    assert_eq!(props1, props2);

    assert_ne!(props1.content, props3.content);
    assert_ne!(props1.class, props3.class);
    assert_ne!(props1, props3);
}

#[test]
fn test_markdown_props_empty_content() {
    let props = MarkdownProps {
        content: String::new(),
        class: classes!("some-class"),
    };

    assert_eq!(props.content, "");
    assert!(!props.class.is_empty());
}

#[test]
fn test_markdown_props_whitespace_content() {
    let props = MarkdownProps {
        content: "   \n\t  \n".to_string(),
        class: Classes::new(),
    };

    assert_eq!(props.content, "   \n\t  \n");
    assert!(props.class.is_empty());
}

#[test]
fn test_markdown_props_complex_content() {
    let complex_content = r#"# Main Title

## Subtitle

This is a **bold** paragraph with *italic* text and `inline code`.

### Code Block
```rust
fn main() {
    println!("Hello, World!");
}
```

- List item 1
- List item 2
  - Nested item

1. Numbered item
2. Another item

> This is a blockquote

[Link text](https://example.com)

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |

~~Strikethrough text~~

- [x] Completed task
- [ ] Pending task"#;

    let props = MarkdownProps {
        content: complex_content.to_string(),
        class: classes!("prose-lg"),
    };

    assert_eq!(props.content, complex_content);
    assert!(props.class.contains("prose-lg"));
}

#[test]
fn test_markdown_props_multiple_classes() {
    let props = MarkdownProps {
        content: "Simple content".to_string(),
        class: classes!("class1", "class2", "class3"),
    };

    assert!(props.class.contains("class1"));
    assert!(props.class.contains("class2"));
    assert!(props.class.contains("class3"));
}

#[test]
fn test_markdown_props_special_characters() {
    let special_content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```

> Quote with "nested" quotes"#;

    let props = MarkdownProps {
        content: special_content.to_string(),
        class: classes!("special-content"),
    };

    assert_eq!(props.content, special_content);
    assert!(props.class.contains("special-content"));
}

#[test]
fn test_markdown_props_unicode_content() {
    let unicode_content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let props = MarkdownProps {
        content: unicode_content.to_string(),
        class: Classes::new(),
    };

    assert_eq!(props.content, unicode_content);
}

#[test]
fn test_markdown_props_very_long_content() {
    let long_content = "# ".to_string() + &"A".repeat(10000) + "\n\n" + &"B".repeat(10000);

    let props = MarkdownProps {
        content: long_content.clone(),
        class: classes!("long-content"),
    };

    assert_eq!(props.content, long_content);
    assert!(props.class.contains("long-content"));
}

#[test]
fn test_markdown_props_clone() {
    let original = MarkdownProps {
        content: "Original content".to_string(),
        class: classes!("original-class"),
    };

    let cloned = original.clone();

    assert_eq!(original.content, cloned.content);
    assert_eq!(original.class, cloned.class);
    assert_eq!(original, cloned);
}

#[test]
fn test_markdown_props_debug() {
    let props = MarkdownProps {
        content: "Debug content".to_string(),
        class: classes!("debug-class"),
    };

    let debug_output = format!("{:?}", props);

    assert!(debug_output.contains("Debug content"));
    assert!(debug_output.contains("debug-class"));
}

#[test]
fn test_markdown_props_with_html_content() {
    let html_content = r#"# HTML in Markdown

<div class="custom-html">
    <p>This is HTML content</p>
    <script>alert('test');</script>
</div>

More markdown content."#;

    let props = MarkdownProps {
        content: html_content.to_string(),
        class: classes!("html-content"),
    };

    assert_eq!(props.content, html_content);
    assert!(props.class.contains("html-content"));
}

#[test]
fn test_markdown_props_with_malformed_markdown() {
    let malformed_content = r#"# Unclosed heading
**Unclosed bold
*Unclosed italic
[Unclosed link
`Unclosed code

- Unclosed list
1. Unclosed numbered list
> Unclosed blockquote"#;

    let props = MarkdownProps {
        content: malformed_content.to_string(),
        class: classes!("malformed"),
    };

    assert_eq!(props.content, malformed_content);
    assert!(props.class.contains("malformed"));
}
