# Public Test Helpers for Cross-Module Testing

## Rule

- Any helper function (e.g., markdown parsing, string transformation) that is used in both production code and tests must be marked `pub` to allow use in test modules.

## Why

- Private helpers cannot be accessed from integration or submodule tests, leading to code duplication or test failures.

## Example

```rust
// Good
pub fn parse_markdown_to_html(content: &str) -> String { ... }

// Bad
fn parse_markdown_to_html(content: &str) -> String { ... } // Not accessible in tests
```
