# Testing Rule: Assertion vs. Parser Output Verification

## Context

When writing or updating tests for parser output (e.g., markdown to HTML), it is common to assume the output based on documentation or prior knowledge. However, parser behavior may change between versions or differ from expectations.

## Rule

- **Always verify test assertions against the actual output of the parser/library in use.**
- **Do not rely solely on documentation or assumptions about output.**
- **If updating a test, print or inspect the actual output before changing assertions.**

## Rationale

- Prevents false positives/negatives in tests due to parser/library changes.
- Reduces rework and debugging time.
- Ensures tests remain accurate and robust across upgrades.

## Example

**Bad:**

```rust
// Assumes HTML is escaped, but parser passes through raw HTML
test!(assert!(result.contains("&lt;div&gt;")));
```

**Good:**

```rust
// Prints actual output, then asserts on what is actually returned
println!("Actual output: {}", result);
assert!(result.contains("<div>"));
```
