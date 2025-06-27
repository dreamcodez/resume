# Use `#[cfg(test)]` for Test-Only Modules

## Context

Test helper modules and code should only be compiled and included during test builds. Including them in normal builds can cause build errors due to missing dev dependencies or test-only APIs.

## Rule

- **Always wrap test-only modules with `#[cfg(test)]`.**
- **Never expose test helpers in production builds.**

## Example

```rust
#[cfg(test)]
pub mod tests;
```

## When to Apply

- When creating a module or file that is only used for tests (e.g., test helpers, mock data)
- When adding new test infrastructure

## Why

- Prevents build errors in non-test builds
- Keeps production binary size minimal
- Ensures test dependencies are only required for testing

## Error Scenario

- If you forget `#[cfg(test)]`, you may see errors like:
  - `unresolved import 'wasm_bindgen_test'`
  - Missing dev-dependency errors
