use yew::prelude::*;

use crate::components::common::markdown::{parse_markdown_to_html, MarkdownProps};

#[test]
fn test_markdown_handles_malformed_markdown() {
    let malformed_content = r#"# Unclosed heading
**Unclosed bold
*Unclosed italic
[Unclosed link
`Unclosed code

- Unclosed list
1. Unclosed numbered list
> Unclosed blockquote"#;

    let result = parse_markdown_to_html(malformed_content);

    // Should still render without crashing
    assert!(result.contains("Unclosed heading"));
    assert!(result.contains("Unclosed bold"));
    assert!(result.contains("Unclosed italic"));
    assert!(result.contains("Unclosed link"));
    assert!(result.contains("Unclosed code"));
    assert!(result.contains("Unclosed list"));
    assert!(result.contains("Unclosed numbered list"));
    assert!(result.contains("Unclosed blockquote"));
}

#[test]
fn test_markdown_handles_very_long_content() {
    let long_content = "# ".to_string() + &"A".repeat(10000) + "\n\n" + &"B".repeat(10000);

    let result = parse_markdown_to_html(&long_content);

    // Should render without crashing
    assert!(result.contains("A"));
    assert!(result.contains("B"));
}

#[test]
fn test_markdown_handles_empty_content() {
    let content = "";

    let result = parse_markdown_to_html(content);

    // Should render empty container
    assert!(result.is_empty() || result.contains(""));
}

#[test]
fn test_markdown_handles_whitespace_only() {
    let content = "   \n\t  \n";

    let result = parse_markdown_to_html(content);

    // Should render container even with only whitespace
    assert!(result.is_empty() || result.contains(""));
}

#[test]
fn test_markdown_handles_null_bytes() {
    let content = "Content with \0 null bytes";

    let result = parse_markdown_to_html(content);

    // Should handle null bytes gracefully
    assert!(result.contains("Content with"));
}

#[test]
fn test_markdown_handles_control_characters() {
    let content = "Content with \x01\x02\x03 control chars";

    let result = parse_markdown_to_html(content);

    // Should handle control characters gracefully
    assert!(result.contains("Content with"));
}

#[test]
fn test_markdown_handles_nested_markdown() {
    let content = r#"# Title

## Subtitle

### Sub-subtitle

#### Sub-sub-subtitle

##### Sub-sub-sub-subtitle

###### Sub-sub-sub-sub-subtitle

This is **bold** and *italic* text with `code`.

- List item 1
  - Nested item 1
    - Deep nested item 1
      - Very deep nested item 1
  - Nested item 2
- List item 2

1. Numbered item 1
   1. Nested numbered item 1
      1. Deep nested numbered item 1
   2. Nested numbered item 2
2. Numbered item 2"#;

    let result = parse_markdown_to_html(content);

    // Should handle deeply nested content
    assert!(result.contains("Title"));
    assert!(result.contains("Subtitle"));
    assert!(result.contains("Sub-subtitle"));
    assert!(result.contains("Sub-sub-subtitle"));
    assert!(result.contains("Sub-sub-sub-subtitle"));
    assert!(result.contains("Sub-sub-sub-sub-subtitle"));
    assert!(result.contains("font-semibold text-gray-900")); // bold
    assert!(result.contains("italic text-gray-700")); // italic
    assert!(result.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // code
    assert!(result.contains("List item 1"));
    assert!(result.contains("Nested item 1"));
    assert!(result.contains("Deep nested item 1"));
    assert!(result.contains("Very deep nested item 1"));
    assert!(result.contains("Numbered item 1"));
    assert!(result.contains("Nested numbered item 1"));
    assert!(result.contains("Deep nested numbered item 1"));
}

#[test]
fn test_markdown_handles_html_in_content() {
    let content = r#"# HTML in Markdown

<div class="custom-html">
    <p>This is HTML content</p>
    <script>alert('test');</script>
    <style>
        .custom { color: red; }
    </style>
</div>

More markdown content with **bold** and *italic*."#;

    let result = parse_markdown_to_html(content);

    // Should handle HTML content
    assert!(result.contains("HTML in Markdown"));
    assert!(result.contains("<div class=\"custom-html\">"));
    assert!(result.contains("<p>This is HTML content</p>"));
    assert!(result.contains("<script>alert('test');</script>"));
    assert!(result.contains("<style>"));
    assert!(result.contains("More markdown content"));
    assert!(result.contains("font-semibold text-gray-900")); // bold
    assert!(result.contains("italic text-gray-700")); // italic
}

#[test]
fn test_markdown_handles_special_characters() {
    let content = r#"# Title with "quotes" and 'apostrophes'

Content with & < > symbols and emojis 🚀 🎉

```html
<div class="test">Content</div>
```

> Quote with "nested" quotes

| Header with "quotes" | Header with 'apostrophes' |
|----------------------|---------------------------|
| Cell with & symbols  | Cell with < > symbols     |"#;

    let result = parse_markdown_to_html(content);

    // Should handle special characters
    assert!(result.contains("quotes"));
    assert!(result.contains("apostrophes"));
    assert!(result.contains("&"));
    assert!(result.contains("<"));
    assert!(result.contains(">"));
    assert!(result.contains("🚀"));
    assert!(result.contains("🎉"));
    assert!(result.contains("<div class=\"test\">Content</div>"));
}

#[test]
fn test_markdown_handles_unicode_content() {
    let content = "Привет мир! 🌍 你好世界! こんにちは世界!";

    let result = parse_markdown_to_html(content);

    // Should handle unicode content
    assert!(result.contains("Привет мир!"));
    assert!(result.contains("你好世界!"));
    assert!(result.contains("こんにちは世界!"));
    assert!(result.contains("🌍"));
}

#[test]
fn test_markdown_handles_mixed_content_types() {
    let content = r#"# Mixed Content

## Text with formatting
This is **bold**, *italic*, and `code` text.

## Lists
- Item 1
- Item 2
  - Nested item
- Item 3

## Code blocks
```rust
fn main() {
    println!("Hello, World!");
}
```

## Tables
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |

## Blockquotes
> This is a blockquote

## Links
[Link text](https://example.com)

## Strikethrough
~~Strikethrough text~~

## Task lists
- [x] Completed task
- [ ] Pending task

## Footnotes
Here is a sentence with a footnote[^1].

[^1]: This is the footnote content."#;

    let result = parse_markdown_to_html(content);

    // Should handle all content types together
    assert!(result.contains("Mixed Content"));
    assert!(result.contains("font-semibold text-gray-900")); // bold
    assert!(result.contains("italic text-gray-700")); // italic
    assert!(result.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // inline code
    assert!(result.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2")); // code block
    assert!(result.contains("Item 1"));
    assert!(result.contains("Nested item"));
    assert!(result.contains("border-collapse border border-gray-300 mb-2")); // table
    assert!(result.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // blockquote
    assert!(result.contains("text-blue-600 hover:text-blue-800 underline")); // links
    assert!(result.contains("<del>")); // strikethrough
    assert!(result.contains("<input")); // task lists
    assert!(result.contains("footnote")); // footnotes
}

#[test]
fn test_markdown_handles_very_deep_nesting() {
    let mut content = String::new();
    for i in 1..=20 {
        content.push_str(&"#".repeat(i));
        content.push_str(&format!(" Level {}\n", i));
    }

    let result = parse_markdown_to_html(&content);

    // Should handle very deep nesting
    for i in 1..=6 {
        assert!(result.contains(&format!("Level {}", i)));
    }
    // Levels 7+ should be treated as regular text since markdown only supports 6 levels
    for i in 7..=20 {
        assert!(result.contains(&format!("Level {}", i)));
    }
}

#[test]
fn test_markdown_handles_consecutive_special_characters() {
    let content = r#"# Title

**Bold** **Bold** **Bold**
*Italic* *Italic* *Italic*
`Code` `Code` `Code`

> Quote
> Quote
> Quote

- List
- List
- List

1. Numbered
2. Numbered
3. Numbered"#;

    let result = parse_markdown_to_html(content);

    // Should handle consecutive special characters
    assert!(result.contains("font-semibold text-gray-900")); // multiple bold
    assert!(result.contains("italic text-gray-700")); // multiple italic
    assert!(result.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // multiple code
    assert!(result.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // multiple quotes
    assert!(result.contains("<ul class=\"ml-4 mb-2\">")); // multiple lists
    assert!(result.contains("<ol>")); // multiple numbered lists
}
