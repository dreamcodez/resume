# Refactor Test Helpers to Minimize API Surface

## Context

Test helpers can become verbose and duplicated across test files, leading to maintenance overhead and potential inconsistencies. Concise, reusable helpers improve test clarity and reduce errors.

## Rule

- **Design test helpers to be as concise and general as possible.**
- **Centralize helpers in a shared module when used across multiple test files.**
- **Refactor verbose or duplicated helpers into single, reusable functions.**

## Example

**Before:**

```rust
// Duplicated in each test file
async fn mount_button(props: ButtonProps) -> HtmlButtonElement { /* ... */ }
```

**After:**

```rust
// Centralized in src/tests/mod.rs
pub async fn mount_component_as_button<T: Component + 'static>(props: T::Properties, selector: &str) -> HtmlButtonElement { /* ... */ }
```

## When to Apply

- When you notice similar helpers in multiple test files
- When adding new interaction tests for components

## Benefits

- Less code duplication
- Easier maintenance
- Consistent test patterns
