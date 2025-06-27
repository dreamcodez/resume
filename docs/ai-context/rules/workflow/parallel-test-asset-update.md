# Workflow Rule: Parallel Test and Asset Update After Major Refactor

## When to Apply

- After a major refactor (e.g., router migration, UI overhaul) that affects both code and test assets.

## Why

- Accelerates project stabilization and prevents piecemeal fixes.

## Rule

- **Update all related test files and reference assets in parallel, not sequentially.**
- **Run all tests (unit, integration, visual) in a single session after updates.**
- **Document the scope of the update in the commit or PR.**

## Example

```sh
# After router migration
cargo test
cargo test --test chrome_screenshot
# Update all failing tests and assets together, then commit
```
