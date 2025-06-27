# WASM Dependency Conflicts and Resolution

## Context

WebAssembly (WASM) projects often encounter dependency conflicts, particularly with crates that don't properly support WASM targets. This rule addresses identifying and resolving these conflicts proactively.

## Rule: Proactively Check WASM Compatibility

### When This Applies

- Adding new dependencies to WASM projects
- Encountering compilation errors in WASM builds
- Working with Yew, wasm-bindgen, or other WASM frameworks
- Building for `wasm32-unknown-unknown` target

### The Problem

Many Rust crates have transitive dependencies that don't support WASM targets, causing compilation failures. Common culprits include:

- `getrandom` without proper WASM features
- `gloo` ecosystem dependencies
- System-specific crates (file I/O, networking, etc.)

### The Solution

Proactively identify and resolve WASM compatibility issues:

1. **Check Dependencies Early**: Verify WASM compatibility before implementation
2. **Use WASM-Specific Alternatives**: Choose crates designed for WASM
3. **Feature Flags**: Enable WASM-specific features where available
4. **Custom Implementations**: Build minimal alternatives when needed

### Implementation Strategy

#### 1. Dependency Analysis

```bash
# Check dependency tree for WASM compatibility
cargo tree --target wasm32-unknown-unknown

# Identify problematic dependencies
cargo check --target wasm32-unknown-unknown

# Check specific crate compatibility
cargo search <crate-name> --target wasm32-unknown-unknown
```

#### 2. WASM-Compatible Alternatives

**Instead of:**

```toml
[dependencies]
yew-router = "0.18"  # Pulls in gloo, getrandom issues
```

**Use:**

```toml
[dependencies]
# Custom minimal router (as implemented today)
# Or WASM-specific alternatives
web-sys = { version = "0.3", features = ["History", "Location"] }
js-sys = "0.3"
```

#### 3. Feature Flag Management

```toml
[dependencies]
gloo-timers = { version = "0.3", features = ["futures"] }
```

### Example from Today's Session

**Problem Identified:**

```
error[E0433]: failed to resolve: use of unresolved module or unlinked crate `yew_router`
error: cannot find derive macro `Routable` in this scope
```

**Root Cause:**

- `yew-router` depends on `gloo`
- `gloo` depends on `getrandom = "0.2.16"`
- `getrandom` lacks `wasm_js` feature for WASM compatibility

**Solution Implemented:**

- Created minimal router using only `web-sys` and `js-sys`
- Avoided problematic transitive dependencies
- Maintained all required functionality

### Prevention Checklist

- [ ] Check `cargo tree` for WASM-incompatible dependencies
- [ ] Verify target compatibility: `cargo check --target wasm32-unknown-unknown`
- [ ] Research WASM-specific alternatives before implementation
- [ ] Test compilation in WASM environment early
- [ ] Document dependency decisions and alternatives

### Common WASM-Compatible Crates

**Web APIs:**

- `web-sys` - Web API bindings
- `js-sys` - JavaScript interop
- `wasm-bindgen` - Core WASM bindings

**Utilities:**

- `wasm-bindgen-futures` - Async support
- `console_error_panic_hook` - Error handling
- `wasm-logger` - Logging

**Avoid:**

- `gloo` ecosystem (unless specifically needed)
- System-specific crates
- Dependencies requiring native compilation

### Troubleshooting Steps

1. **Identify the Culprit:**

   ```bash
   cargo tree --target wasm32-unknown-unknown | grep -E "(getrandom|gloo|system)"
   ```

2. **Check Feature Availability:**

   ```bash
   cargo search <crate-name> --target wasm32-unknown-unknown
   ```

3. **Find Alternatives:**

   - Search for WASM-specific crates
   - Check if features can be disabled
   - Consider custom implementation

4. **Test Incrementally:**
   ```bash
   cargo check --target wasm32-unknown-unknown --lib
   ```

### Success Indicators

- `cargo check --target wasm32-unknown-unknown` succeeds
- No dependency conflicts in WASM builds
- All functionality works in browser environment
- Minimal dependency footprint
- Clear documentation of dependency choices

### Related Rules

- **Test Module Discovery**: Ensure tests work in WASM environment
- **Build Process Optimization**: Streamline WASM compilation
- **Dependency Management**: Maintain minimal, compatible dependencies
