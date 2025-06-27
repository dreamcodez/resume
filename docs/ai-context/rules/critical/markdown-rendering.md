# Robust Markdown Rendering with Standardized Styling

**Category:** critical

## When to Apply

- When displaying markdown content in Yew or Svelte components.
- When integrating or updating markdown parsing/rendering logic.

## Why It Matters

- Ensures all markdown is rendered consistently and styled according to design system.
- Prevents XSS, broken layouts, and inconsistent typography.

## Rule

- **Always use a well-maintained markdown parser** (e.g., `pulldown-cmark` for Rust/Yew).
- **Post-process HTML output** to add required Tailwind classes for typography and layout.
- **Never render raw markdown or use ad-hoc string replacements.**

## Example

```rust
// CORRECT: Use pulldown-cmark and add classes
let html = render_markdown_with_classes(markdown_input);

// INCORRECT: Render raw markdown or use naive replacements
html! { <div>{markdown_input}</div> }
```
