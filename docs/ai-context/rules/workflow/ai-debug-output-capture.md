# Workflow Rule: Debug Output Capture Before Test Update

## Context

When updating or fixing a test, especially for parser or rendering output, it is critical to capture the actual output before changing assertions. This prevents guesswork and ensures assertions match reality.

## Rule

- **Always add a debug print (e.g., `println!`) of the actual output before updating or fixing test assertions.**
- **Use the debug output to inform assertion updates.**
- **Remove or comment out debug prints after confirming test correctness.**

## Rationale

- Prevents assertion drift and repeated test failures.
- Accelerates debugging and test maintenance.
- Ensures test changes are based on real data, not assumptions.

## Example

**Bad:**

```rust
// Updates assertion without checking actual output
assert!(result.contains("expected-value"));
```

**Good:**

```rust
// Prints output, then updates assertion
printf!("Actual output: {}", result);
assert!(result.contains("actual-value"));
```
