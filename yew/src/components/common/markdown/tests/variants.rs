use crate::components::common::markdown::parse_markdown_to_html;

#[test]
fn test_markdown_variant_basic_text() {
    let content = "This is basic text content.";

    let html = parse_markdown_to_html(content);

    // Should render basic text with proper styling
    assert!(html.contains("This is basic text content"));
    assert!(html.contains("mb-2")); // paragraph styling
}

#[test]
fn test_markdown_variant_headings_only() {
    let content = r#"# H1 Title
## H2 Title
### H3 Title
#### H4 Title
##### H5 Title
###### H6 Title"#;

    let html = parse_markdown_to_html(content);

    // Should render all heading levels with proper styling
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1
    assert!(html.contains("text-xl font-semibold mb-3 text-gray-900")); // h2
    assert!(html.contains("text-lg font-medium mb-2 text-gray-900")); // h3
    assert!(html.contains("text-base font-medium mb-2 text-gray-900")); // h4
    assert!(html.contains("text-sm font-medium mb-1 text-gray-900")); // h5
    assert!(html.contains("text-xs font-medium mb-1 text-gray-900")); // h6
}

#[test]
fn test_markdown_variant_lists_only() {
    let content = r#"- Unordered item 1
- Unordered item 2
- Unordered item 3

1. Ordered item 1
2. Ordered item 2
3. Ordered item 3"#;

    let html = parse_markdown_to_html(content);

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

#[test]
fn test_markdown_variant_code_only() {
    let content = r#"Inline `code` text.

```rust
fn main() {
    println!("Hello, World!");
}
```

More inline `code`."#;

    let html = parse_markdown_to_html(content);

    // Should render code with proper styling
    assert!(html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // inline code
    assert!(html.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2")); // code block
    assert!(html.contains("fn main()"));
    assert!(html.contains("println!"));
}

#[test]
fn test_markdown_variant_links_only() {
    let content = r#"[External Link](https://example.com)
[Internal Link](/about)
[Link with Title](https://example.com "Link Title")"#;

    let html = parse_markdown_to_html(content);

    // Should render links with proper styling
    assert!(html.contains("text-blue-600 hover:text-blue-800 underline"));
    assert!(html.contains("href=\"https://example.com\""));
    assert!(html.contains("href=\"/about\""));
    assert!(html.contains("External Link"));
    assert!(html.contains("Internal Link"));
    assert!(html.contains("Link with Title"));
}

#[test]
fn test_markdown_variant_tables_only() {
    let content = r#"| Header 1 | Header 2 | Header 3 |
|----------|----------|----------|
| Cell 1   | Cell 2   | Cell 3   |
| Cell 4   | Cell 5   | Cell 6   |"#;

    let html = parse_markdown_to_html(content);

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

#[test]
fn test_markdown_variant_blockquotes_only() {
    let content = r#"> This is a single line blockquote.

> This is a multi-line
> blockquote with
> multiple lines.

> Blockquote with **bold** and *italic* text."#;

    let html = parse_markdown_to_html(content);

    // Should render blockquotes with proper styling
    assert!(html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2"));
    assert!(html.contains("This is a single line blockquote"));
    assert!(html.contains("This is a multi-line"));
    assert!(html.contains("blockquote with"));
    assert!(html.contains("multiple lines"));
    assert!(html.contains("font-semibold text-gray-900")); // bold in blockquote
    assert!(html.contains("italic text-gray-700")); // italic in blockquote
}

#[test]
fn test_markdown_variant_emphasis_only() {
    let content = r#"This is **bold** text.
This is *italic* text.
This is ***bold and italic*** text.
This is ~~strikethrough~~ text."#;

    let html = parse_markdown_to_html(content);

    // Should render emphasis with proper styling
    assert!(html.contains("font-semibold text-gray-900")); // bold
    assert!(html.contains("italic text-gray-700")); // italic
    assert!(html.contains("<del>")); // strikethrough
    assert!(html.contains("bold"));
    assert!(html.contains("italic"));
    assert!(html.contains("strikethrough"));
}

#[test]
fn test_markdown_variant_task_lists_only() {
    let content = r#"- [x] Completed task 1
- [ ] Pending task 1
- [x] Completed task 2
- [ ] Pending task 2"#;

    let html = parse_markdown_to_html(content);

    // Should render task lists with proper structure
    assert!(html.contains("<input"));
    assert!(html.contains("type=\"checkbox\""));
    assert!(html.contains("Completed task 1"));
    assert!(html.contains("Pending task 1"));
    assert!(html.contains("Completed task 2"));
    assert!(html.contains("Pending task 2"));
}

#[test]
fn test_markdown_variant_footnotes_only() {
    let content = r#"Here is a sentence with a footnote[^1].

Another sentence with a footnote[^2].

[^1]: This is the first footnote content.
[^2]: This is the second footnote content."#;

    let html = parse_markdown_to_html(content);

    // Should render footnotes with proper structure
    assert!(html.contains("footnote"));
    assert!(html.contains("This is the first footnote content"));
    assert!(html.contains("This is the second footnote content"));
}

#[test]
fn test_markdown_variant_mixed_content() {
    let content = r#"# Main Title

This is a paragraph with **bold** and *italic* text.

## Subtitle

- List item 1
- List item 2 with `inline code`
- List item 3

> This is a blockquote with [a link](https://example.com).

```rust
fn main() {
    println!("Hello, World!");
}
```

| Column 1 | Column 2 |
|----------|----------|
| Data 1   | Data 2   |

- [x] Completed task
- [ ] Pending task"#;

    let html = parse_markdown_to_html(content);

    // Should render mixed content with proper styling
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1
    assert!(html.contains("text-xl font-semibold mb-3 text-gray-900")); // h2
    assert!(html.contains("font-semibold text-gray-900")); // bold
    assert!(html.contains("italic text-gray-700")); // italic
    assert!(html.contains("<ul class=\"ml-4 mb-2\">")); // unordered list
    assert!(html.contains("bg-gray-100 px-1 py-0.5 rounded text-sm font-mono")); // inline code
    assert!(html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // blockquote
    assert!(html.contains("text-blue-600 hover:text-blue-800 underline")); // link
    assert!(html.contains("bg-gray-100 p-3 rounded-lg overflow-x-auto mb-2")); // code block
    assert!(html.contains("border-collapse border border-gray-300 mb-2")); // table
    assert!(html.contains("type=\"checkbox\"")); // task list
}

#[test]
fn test_markdown_variant_empty_content() {
    let content = "";

    let html = parse_markdown_to_html(content);

    // Should handle empty content gracefully
    assert!(html.is_empty() || html.trim().is_empty());
}

#[test]
fn test_markdown_variant_whitespace_only() {
    let content = "   \n  \t  \n  ";

    let html = parse_markdown_to_html(content);

    // Should handle whitespace-only content gracefully
    assert!(html.is_empty() || html.trim().is_empty());
}

#[test]
fn test_markdown_variant_unicode_content() {
    let content = r#"# 🚀 Unicode Title

This is content with emojis: 🎉 🎯 ⚡

- List item with 🏗️
- Another item with 🎨

> Blockquote with 🌟 and 💎

**Bold text with 🎭** and *italic with 🎪*"#;

    let html = parse_markdown_to_html(content);

    // Should render unicode content with proper styling
    assert!(html.contains("🚀 Unicode Title"));
    assert!(html.contains("🎉 🎯 ⚡"));
    assert!(html.contains("🏗️"));
    assert!(html.contains("🎨"));
    assert!(html.contains("🌟 and 💎"));
    assert!(html.contains("🎭"));
    assert!(html.contains("🎪"));
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1 styling
    assert!(html.contains("font-semibold text-gray-900")); // bold styling
    assert!(html.contains("italic text-gray-700")); // italic styling
}

#[test]
fn test_markdown_variant_special_characters() {
    let content = r#"# Title with & < > " '

This has special chars: & < > " ' \ / | ! @ # $ % ^ * ( ) _ + = { } [ ] : ; , . ? ~ ` - =

- List with & < > chars
- Another with " ' chars

> Blockquote with special chars: & < > " '

**Bold with & < >** and *italic with " '*"#;

    let html = parse_markdown_to_html(content);

    // Should render special characters with proper styling
    assert!(html.contains("Title with &amp; &lt; &gt; &quot; &#39;"));
    assert!(html.contains("This has special chars: &amp; &lt; &gt; &quot; &#39;"));
    assert!(html.contains("List with &amp; &lt; &gt; chars"));
    assert!(html.contains("Another with &quot; &#39; chars"));
    assert!(html.contains("Blockquote with special chars: &amp; &lt; &gt; &quot; &#39;"));
    assert!(html.contains("Bold with &amp; &lt; &gt;"));
    assert!(html.contains("italic with &quot; &#39;"));
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1 styling
    assert!(html.contains("font-semibold text-gray-900")); // bold styling
    assert!(html.contains("italic text-gray-700")); // italic styling
}

#[test]
fn test_markdown_variant_nested_structures() {
    let content = r#"# Main Title

## Subtitle

### Sub-subtitle

This is a paragraph.

> This is a blockquote
> 
> - With a list inside
> - And another item
> 
> > Nested blockquote
> > 
> > With **bold** and *italic*

- Main list item
  - Nested list item
    - Deeply nested item
  - Another nested item
- Another main item

1. Ordered list
   1. Nested ordered
      1. Deeply nested
   2. Another nested

| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | - List in table
|          | - Another item"#;

    let html = parse_markdown_to_html(content);

    // Should render nested structures with proper styling
    assert!(html.contains("text-2xl font-bold mb-4 text-gray-900")); // h1
    assert!(html.contains("text-xl font-semibold mb-3 text-gray-900")); // h2
    assert!(html.contains("text-lg font-medium mb-2 text-gray-900")); // h3
    assert!(html.contains("border-l-4 border-gray-300 pl-4 italic text-gray-600 mb-2")); // blockquote
    assert!(html.contains("<ul class=\"ml-4 mb-2\">")); // unordered list
    assert!(html.contains("<ol>")); // ordered list
    assert!(html.contains("border-collapse border border-gray-300 mb-2")); // table
    assert!(html.contains("font-semibold text-gray-900")); // bold
    assert!(html.contains("italic text-gray-700")); // italic
}
