# Critical Rule: Immediate Reference Asset Update After Visual Output Change

## When to Apply

- After any change to routing, rendering, or component output that affects visual regression tests.

## Why

- Prevents repeated test failures and wasted cycles on outdated reference screenshots.

## Rule

- **Immediately update the reference screenshot or asset after confirming a visual output change is intentional and correct.**
- **Document the change in the test or commit message.**
- **Never leave a failing visual regression test unaddressed.**

## Example

```sh
# After router or UI change
rm tests/reference-screenshots/home-page-reference.png
cargo test --test chrome_screenshot
# Confirm new reference is correct, then commit
```
