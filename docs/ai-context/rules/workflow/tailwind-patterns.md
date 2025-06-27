# Universal Tailwind Typography and Component Patterns

**Category:** workflow

## When to Apply

- When creating or refactoring any UI component or page.
- When porting styles from raw CSS to Tailwind.

## Why It Matters

- Ensures visual consistency and maintainability.
- Reduces custom CSS and duplication.

## Rule

- **Always use Tailwind utility classes for all layout, spacing, and typography.**
- **Extract reusable patterns** (e.g., card, section, heading) into Tailwind `@layer components` or Yew component wrappers.
- **Never add custom CSS unless a Tailwind utility or component cannot achieve the desired result.**

## Example

```rust
// CORRECT: Tailwind classes for headings
html! { <h1 class="text-3xl font-bold mb-4">{"Resume"}</h1> }

// INCORRECT: Inline style or custom CSS class
html! { <h1 class="my-custom-title">{"Resume"}</h1> }
```
