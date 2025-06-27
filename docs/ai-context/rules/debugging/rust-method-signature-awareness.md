# Rust Method Signature Awareness

## Rule

- Always check the return type of methods (e.g., `has_attribute` returns `bool`, not `Option`).
- Do not chain `.unwrap()` or `.unwrap_or()` on methods that return `bool`.

## Why

- Misunderstanding method signatures leads to compile errors and wasted debugging time.

## Example

```rust
// Correct
assert!(element.has_attribute("class"));

// Incorrect
assert!(element.has_attribute("class").unwrap_or(false)); // Compile error
```
