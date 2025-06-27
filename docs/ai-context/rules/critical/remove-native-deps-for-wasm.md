# Remove Unused Native Dependencies for WASM Builds

## Context

Native dependencies (e.g., crates with C/C++ code or system bindings) often do not compile to WASM and can cause build failures. These should be removed or replaced with WASM-compatible alternatives when targeting WASM.

## Rule

- **Review and remove all unused or unnecessary native dependencies when building for WASM.**
- **Prefer pure Rust or WASM-compatible crates.**

## Example

- Remove unused crates like `gloo-net`, `gloo-storage` if not required:

```toml
[dependencies]
# gloo-net = "0.4"  # Remove if not used
# gloo-storage = "0.3"  # Remove if not used
```

## When to Apply

- When adding or updating dependencies
- When encountering WASM build errors related to native code

## Error Scenario

- Build fails with errors about missing C toolchain, `bzip2-sys`, `zstd-sys`, etc.
- Native crates do not support `wasm32-unknown-unknown` or `wasm32-unknown-emscripten` targets

## Benefits

- Successful WASM builds
- Smaller binary size
- Fewer cross-compilation headaches
