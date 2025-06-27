# Avoid Unnecessary Browser Test Overhead

## Rule

- Only use `wasm_bindgen_test` for tests that require browser APIs or DOM access.
- Prefer `#[test]` for logic, parsing, and data validation.

## Why

- Browser tests are much slower and require more setup.
- Keeping logic tests as unit tests speeds up development and CI.

## Example

```rust
// Prefer this for logic:
#[test]
fn test_edge_case_parsing() { ... }

// Only use this for DOM:
#[wasm_bindgen_test]
fn test_dom_rendering() { ... }
```
