# Workflow Rule: Dependency Removal and WASM Compatibility Checklist

## When to Apply

- When removing or replacing dependencies in a WASM-targeted project.

## Why

- Prevents build failures and wasted debugging time due to incompatible or lingering native dependencies.

## Rule

- **After removing a dependency, always run `cargo check --target wasm32-unknown-unknown` to verify WASM compatibility.**
- **Check for transitive dependencies that may still pull in problematic crates.**
- **Document the rationale for removal and any alternatives used.**

## Example

```sh
# Remove yew-router, check for WASM build issues
cargo check --target wasm32-unknown-unknown
# If errors, run:
cargo tree --target wasm32-unknown-unknown | grep -E "(getrandom|gloo|system)"
```
