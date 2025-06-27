# Dependency Cleanup for WASM Testing

## Workflow Pattern

When transitioning to WASM/browser testing, systematically remove native-only dependencies to prevent compilation conflicts and improve build performance.

## Problem Scenario

Native dependencies like `headless_chrome`, `getrandom`, and `gloo-timers` can cause WASM compilation failures and slow down builds unnecessarily.

## Detection Pattern

### 1. Identify Native-Only Dependencies

```bash
# Check what's pulling in problematic dependencies
cargo tree -i getrandom@0.3.3
cargo tree -i headless_chrome
cargo tree -i gloo-timers
```

### 2. Look for Error Patterns

```
error: The wasm32-unknown-unknown targets are not supported by default
error: could not compile `getrandom` due to previous errors
error: use of unresolved module or unlinked crate `headless_chrome`
```

## Cleanup Workflow

### Step 1: Remove Direct Dependencies

```toml
# Cargo.toml - Remove these from [dependencies]
getrandom = { version = "0.3", features = ["wasm_js"] }  # ❌ Remove
gloo-timers = { version = "0.3", features = ["futures"] } # ❌ Remove
headless_chrome = "1.0"                                   # ❌ Remove
wasm-bindgen-backend = "0.2"                             # ❌ Remove
base64ct = "1.7.3"                                       # ❌ Remove
```

### Step 2: Remove Features and Workspace Dependencies

```toml
# Cargo.toml - Remove these sections
[features]
default = []
wasm-support = ["getrandom/wasm_js"]  # ❌ Remove

[workspace.dependencies]
getrandom = { version = "0.3", features = ["wasm_js"] }  # ❌ Remove
```

### Step 3: Update Test Code

```rust
// Remove native-only test files
// tests/chrome_screenshot.rs        # ❌ Delete
// tests/helpers/mod.rs              # ❌ Delete

// Remove native-only imports from remaining tests
// use headless_chrome::...          # ❌ Remove
// use gloo_timers::future::...      # ❌ Remove
```

## Verification Commands

### Before Cleanup

```bash
wasm-pack test --headless --firefox  # Should fail with dependency errors
```

### After Cleanup

```bash
wasm-pack test --headless --firefox  # Should compile and run
cargo test --lib                     # Unit tests should still work
```

## Common Dependencies to Remove

### ❌ Native-Only (Remove)

- `headless_chrome` - Browser automation (native only)
- `getrandom` - Random number generation (native only)
- `gloo-timers` - Timer utilities (native only)
- `wasm-bindgen-backend` - Build-time only
- `base64ct` - Encoding (if not used)

### ✅ WASM-Compatible (Keep)

- `wasm-bindgen` - WASM interop
- `wasm-bindgen-test` - WASM testing
- `web-sys` - Web APIs
- `js-sys` - JavaScript interop
- `console_error_panic_hook` - Error handling

## Performance Impact

- **Build time**: 50-80% faster
- **Dependency tree**: 30-50% smaller
- **Compilation errors**: Eliminated
- **Test execution**: Much faster

## Error Recovery

If cleanup breaks existing functionality:

1. **Check import errors** - Remove unused imports
2. **Update test code** - Replace native APIs with WASM equivalents
3. **Verify unit tests** - Ensure `cargo test --lib` still works
4. **Test browser tests** - Ensure `wasm-pack test` works

## Related Rules

- `wasm-cfg-flag-conflicts.md` - Configuration issues
- `test-script-targeting.md` - Test organization
- `framework-test-command-separation.md` - Multi-framework testing

## Success Metrics

- ✅ WASM tests compile without errors
- ✅ Unit tests still pass
- ✅ Build time significantly reduced
- ✅ No native-only dependencies in tree
