# Document Test Type and Environment Requirements

## Rule

- For every test module or file, document whether it is a unit test or browser test, and what environment/tools are required to run it.
- Include this in a module-level doc comment or at the top of the file.

## Why

- Prevents confusion about how to run tests and avoids wasted time running the wrong command.

## Example

```rust
//! This file contains browser-based tests for DOM rendering.
//! Run with: wasm-pack test --headless --firefox
```
