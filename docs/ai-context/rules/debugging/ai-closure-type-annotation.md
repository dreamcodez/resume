# Debugging Rule: Explicit Type Annotation for AI-Generated Closures

## When to Apply

- When the AI generates or edits Rust closures, especially for Yew callbacks.

## Why

- Prevents compilation errors due to missing or ambiguous types in closures, which are common in AI-generated code.

## Rule

- **Always provide explicit type annotations for closure parameters in AI-generated or edited code, especially for Yew `Callback::from` and similar APIs.**
- **If a compilation error occurs, immediately add the required type.**

## Example

```rust
// BAD: let cb = Callback::from(|route_info| { ... });
let cb = Callback::from(|route_info: RouteInfo| { ... });
```
