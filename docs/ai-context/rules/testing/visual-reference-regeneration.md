# Testing Rule: Systematic Reference Asset Regeneration Pattern

## When to Apply

- After intentional UI or routing changes that affect visual regression tests.

## Why

- Ensures that reference assets are always in sync with the current codebase, preventing false positives in CI and local runs.

## Rule

- **Regenerate all affected reference screenshots/assets in a single, systematic pass after confirming output changes.**
- **Do not update only the failing asset; review all related references for consistency.**
- **Run the full visual test suite after regeneration.**

## Example

```sh
# Regenerate all references after router change
rm tests/reference-screenshots/*.png
cargo test --test chrome_screenshot
# Review and commit all updated references
```
