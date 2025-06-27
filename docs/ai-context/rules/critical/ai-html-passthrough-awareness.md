# Critical Rule: HTML Passthrough/Escaping Awareness in Markdown Components

## Context

Markdown parsers (e.g., pulldown-cmark) may pass through raw HTML or escape it, depending on configuration and version. Test expectations must match the actual behavior.

## Rule

- **Document the expected HTML passthrough/escaping behavior for the markdown component.**
- **Tests must assert on the actual passthrough/escaping behavior, not assumptions.**
- **If parser behavior changes (e.g., after an upgrade), update both documentation and tests.**

## Rationale

- Prevents test failures and confusion due to mismatched expectations.
- Ensures security and rendering correctness.
- Makes future upgrades and debugging easier.

## Example

**Bad:**

```rust
// Assumes HTML is escaped, but parser passes through raw HTML
assert!(result.contains("&lt;div&gt;"));
```

**Good:**

```rust
// Asserts on actual passthrough behavior
assert!(result.contains("<div>"));
```
