# Yew Test Type Separation: Unit vs. Browser

## When to Use Each Test Type

- **Unit tests (`#[test]`)**: Use for pure logic, data structure, and string output validation. These run with `cargo test` and do not require a browser or WASM.
- **Browser tests (`#[wasm_bindgen_test]`)**: Use for anything that interacts with the DOM, simulates user events, or requires browser APIs. These run with `wasm-pack test --headless --firefox` or similar.

## Why This Matters

- Running all tests as browser tests is slow and can cause unnecessary complexity.
- Unit tests are faster, easier to debug, and run in CI without browser dependencies.

## Example

```rust
// Unit test (fast, no browser needed)
#[test]
fn test_markdown_props_default() {
    let props = MarkdownProps::default();
    assert_eq!(props.content, "");
}

// Browser test (DOM interaction)
use wasm_bindgen_test::*;
#[wasm_bindgen_test]
fn test_markdown_renders_heading() {
    // ... render to DOM and check HTML ...
}
```
