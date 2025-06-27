# Mandatory Error Visibility for Compilation and Build Steps

**Category:** critical

## When to Apply

- When running `cargo build`, `trunk serve`, or any build/test command.
- When scripting or automating dev workflows.

## Why It Matters

- Prevents silent failures and wasted debugging time.
- Ensures all errors are visible to both humans and AI agents.

## Rule

- **Always surface the full error output** in logs, terminal, or chat.
- **Never summarize or hide errors**—copy/paste or pipe the full output.
- **If using scripts,** ensure `set -e` and proper error propagation.

## Example

```sh
# CORRECT: Show all errors
cargo build

# INCORRECT: Suppress or summarize errors
cargo build 2>/dev/null
```
