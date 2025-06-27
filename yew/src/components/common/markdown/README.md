# Markdown Component

A Yew component that renders markdown content with custom styling using the `pulldown-cmark` library.

## Features

- **Full Markdown Support**: Renders all standard markdown elements including headings, lists, code blocks, tables, links, and more
- **Custom Styling**: Applies Tailwind CSS classes for consistent design
- **Accessibility**: Proper semantic HTML structure
- **Extensible**: Supports custom CSS classes
- **Performance**: Efficient rendering with minimal overhead

## Supported Markdown Elements

### Headings

```markdown
# H1 Title

## H2 Title

### H3 Title

#### H4 Title

##### H5 Title

###### H6 Title
```

### Text Formatting

```markdown
**Bold text**
_Italic text_
`Inline code`
~~Strikethrough text~~
```

### Lists

```markdown
- Unordered list item
- Another item
  - Nested item

1. Ordered list item
2. Another item
   1. Nested item
```

### Code Blocks

````markdown
```rust
fn main() {
    println!("Hello, World!");
}
```
````

````

### Links
```markdown
[Link text](https://example.com)
[Link with title](https://example.com "Link Title")
````

### Tables

```markdown
| Header 1 | Header 2 |
| -------- | -------- |
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |
```

### Blockquotes

```markdown
> This is a blockquote
> with multiple lines
```

### Task Lists

```markdown
- [x] Completed task
- [ ] Pending task
```

### Footnotes

```markdown
Here is a sentence with a footnote[^1].

[^1]: This is the footnote content.
```

## Usage

### Basic Usage

```rust
use yew::prelude::*;
use crate::components::common::markdown::Markdown;

#[function_component(MyComponent)]
fn my_component() -> Html {
    let markdown_content = "# Hello World\n\nThis is **bold** text.".to_string();

    html! {
        <Markdown content={markdown_content} />
    }
}
```

### With Custom Classes

```rust
use yew::prelude::*;
use crate::components::common::markdown::Markdown;

#[function_component(MyComponent)]
fn my_component() -> Html {
    let markdown_content = "# Custom Styled Content\n\nThis has custom styling.".to_string();

    html! {
        <Markdown
            content={markdown_content}
            class={classes!("prose-lg", "dark:prose-invert", "custom-class")}
        />
    }
}
```

## Props

| Prop      | Type      | Default          | Description                     |
| --------- | --------- | ---------------- | ------------------------------- |
| `content` | `String`  | Required         | The markdown content to render  |
| `class`   | `Classes` | `Classes::new()` | Additional CSS classes to apply |

## Styling

The component applies the following default Tailwind CSS classes:

- **Container**: `prose prose-sm max-w-none`
- **Content**: `whitespace-pre-line text-gray-700 leading-relaxed`
- **Headings**: Various text sizes and weights with proper spacing
- **Lists**: Proper indentation and spacing
- **Code**: Gray background with monospace font
- **Links**: Blue color with hover effects and underline
- **Tables**: Bordered with proper cell padding
- **Blockquotes**: Left border with italic styling

### Custom Styling

You can override the default styling by passing custom classes:

```rust
html! {
    <Markdown
        content={content}
        class={classes!(
            "prose-lg",           // Larger text
            "dark:prose-invert",  // Dark mode
            "prose-red",          // Red theme
            "max-w-4xl"           // Max width
        )}
    />
}
```

## Examples

### Blog Post Content

````rust
let blog_content = r#"# My Blog Post

## Introduction

This is a **fantastic** blog post about *interesting topics*.

### Key Points

- Point 1 with `code`
- Point 2 with [link](https://example.com)
- Point 3

```rust
fn example() {
    println!("Code example");
}
````

> Important note: This is a blockquote.

| Feature | Status |
| ------- | ------ |
| Working | ✅     |
| Pending | ⏳     |

- [x] Task 1
- [ ] Task 2

Here is a reference[^1].

[^1]: Reference details."#;

html! {
<Markdown content={blog_content.to_string()} />
}

````

### Documentation

```rust
let docs_content = r#"# API Documentation

## Usage

```typescript
import { Component } from 'library';

const instance = new Component();
instance.doSomething();
````

### Parameters

| Name     | Type     | Description      |
| -------- | -------- | ---------------- |
| `param1` | `string` | First parameter  |
| `param2` | `number` | Second parameter |

### Returns

Returns a `Promise<Result>` object.

> **Note**: This function is asynchronous."#;

html! {
<Markdown
content={docs_content.to_string()}
class={classes!("prose-lg", "max-w-4xl")}
/>
}

````

## Testing

The component includes comprehensive tests covering:

- **Props**: Property validation and default values
- **Rendering**: All markdown elements render correctly
- **Edge Cases**: Malformed markdown, empty content, special characters
- **Accessibility**: Proper semantic structure
- **Interactions**: Content updates and prop changes
- **Variants**: Different content types and styling

Run tests with:

```bash
wasm-pack test --headless --firefox
````

## Dependencies

- `pulldown-cmark`: Markdown parsing
- `yew`: Component framework
- `gloo`: Web utilities for testing

## Performance Considerations

- The component efficiently parses markdown using `pulldown-cmark`
- HTML generation is optimized for minimal overhead
- Custom styling is applied through string replacement for performance
- Large content is handled gracefully

## Browser Support

The component works in all modern browsers that support:

- WebAssembly (for `pulldown-cmark`)
- ES6 modules
- Custom elements

## Contributing

When contributing to this component:

1. Add tests for new features
2. Ensure accessibility compliance
3. Update documentation for API changes
4. Follow the existing code style
5. Test with various markdown content types

## License

This component is part of the larger project and follows the same license terms.
