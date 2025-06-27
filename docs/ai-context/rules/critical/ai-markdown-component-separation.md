# Critical Rule: Markdown Component Styling Separation

## Context

In Yew and similar frameworks, the Markdown component is responsible solely for converting markdown to HTML. Styling must be handled by the markdown content itself or by external CSS (e.g., Tailwind), not by the component.

## Rule

- **Never inject custom HTML classes, wrappers, or inline styles in the Markdown component.**
- **All styling must be applied via the markdown content or external CSS.**
- **Tests for the Markdown component should assert semantic HTML structure and content, not specific classes.**

## Rationale

- Prevents coupling between content rendering and design system.
- Ensures markdown remains portable and framework-agnostic.
- Simplifies test maintenance and improves reusability.

## Example

**Bad:**

```rust
// Injects custom classes
html! { <div class="markdown-body">{ html_content }</div> }
```

**Good:**

```rust
// Only renders HTML from markdown
html! { <div>{ html_content }</div> }
// Styling handled by Tailwind or markdown content
```
