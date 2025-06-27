# Centralized DOM-Mounting Helper for Interaction Tests Only

## Context

When writing Yew component tests, only interaction tests (those that simulate real DOM events) should require mounting components in the browser DOM. All other tests (unit, rendering, accessibility) should remain pure Rust unit tests.

## Rule

- **Centralize all DOM-mounting helpers in a single module (e.g., `src/tests/mod.rs`).**
- **Use these helpers ONLY in interaction tests that require real DOM events.**
- **Do NOT use DOM-mounting helpers for unit, rendering, or accessibility tests.**

## Example Usage

```rust
use crate::tests::mount_component_as_button;

#[wasm_bindgen_test]
async fn test_button_click_event() {
    let props = ButtonProps { /* ... */ };
    let button = mount_component_as_button::<Button>(props, "button").await;
    // Simulate click, assert DOM changes
}
```

## Anti-Patterns

- ❌ Duplicating DOM-mounting logic in each test file
- ❌ Using DOM-mounting helpers in pure logic or class generation tests

## When to Apply

- Any time you need to simulate real user interactions (click, focus, keyboard, etc.) in a test
- When adding new interaction tests for Yew components

## Benefits

- Reduces code duplication
- Prevents accidental use of slow browser tests for pure logic
- Makes test intent clear and maintainable
