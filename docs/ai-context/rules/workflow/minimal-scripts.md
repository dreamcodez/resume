# Minimal, Documented Script Usage for Dev and Build

**Category:** workflow

## When to Apply

- When adding or modifying npm scripts, shell scripts, or build commands.

## Why It Matters

- Reduces confusion, duplication, and maintenance burden.
- Ensures all developers and AI agents use the same, reliable commands.

## Rule

- **Only add scripts for non-trivial, multi-step, or cross-platform tasks.**
- **Document every script** in the README with its purpose and usage.
- **Prefer direct use of `trunk`, `cargo`, and `npm` for standard tasks.**

## Example

```json
// CORRECT: Minimal scripts
"scripts": {
  "dev": "trunk serve",
  "build:css": "tailwindcss -i styles/input.css -o styles/output.css"
}

// INCORRECT: Redundant or undocumented scripts
"scripts": {
  "start": "npm run dev",
  "run": "npm run start"
}
```
