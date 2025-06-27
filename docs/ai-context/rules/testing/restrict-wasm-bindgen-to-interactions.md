# Restrict `wasm_bindgen_test` to Real DOM Interactions

## Context

`wasm_bindgen_test` is designed for tests that require a real browser environment, such as simulating user events or interacting with the DOM. Using it for pure logic or class generation tests slows down the test suite and adds unnecessary complexity.

## Rule

- **Use `#[wasm_bindgen_test]` ONLY for tests that require real DOM events or browser APIs.**
- **Use pure Rust `#[test]` for all logic, class generation, and props tests.**

## Examples

**Correct:**

```rust
#[wasm_bindgen_test]
async fn test_button_click_event() {
    // Simulate click, assert DOM changes
}
```

**Incorrect:**

```rust
#[wasm_bindgen_test]
async fn test_button_class_generation() {
    // Just checks class string, no DOM needed
}
```

**Correct:**

```rust
#[test]
fn test_button_class_generation() {
    // Pure logic, no DOM
}
```

## When to Apply

- When writing or refactoring tests for Yew components
- When deciding between `#[test]` and `#[wasm_bindgen_test]`

## Benefits

- Faster test suite
- Clear separation of test types
- Fewer unnecessary dependencies
