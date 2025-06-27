# WASM Configuration Flag Conflicts

## Critical Issue

Manual `--cfg` flags in `.cargo/config.toml` can conflict with automatic target-specific configuration, causing WASM compilation failures.

## Problem Scenario

When `.cargo/config.toml` manually sets `--cfg target_arch="wasm32"` or `--cfg target_os="unknown"`, these conflict with the Rust toolchain's automatic target configuration, resulting in:

```
error: unexpected `--cfg target_arch="wasm32"` flag
error: unexpected `--cfg target_os="unknown"` flag
```

## Root Cause

The Rust toolchain automatically sets these flags when using `--target wasm32-unknown-unknown`. Manual configuration creates duplicate/conflicting flags.

## Solution Pattern

### ❌ Avoid This Configuration

```toml
# .cargo/config.toml - WRONG
[target.wasm32-unknown-unknown]
rustflags = [
    "--cfg", "target_arch=\"wasm32\"",    # ❌ Manual - conflicts
    "--cfg", "target_os=\"unknown\"",     # ❌ Manual - conflicts
    "--cfg", "wasm_js",                   # ✅ OK - custom flag
]
```

### ✅ Use This Configuration

```toml
# .cargo/config.toml - CORRECT
[target.wasm32-unknown-unknown]
rustflags = [
    "--cfg", "wasm_js",                   # ✅ Only custom flags
]

[env]
CARGO_CFG_WASM_JS = "1"                  # ✅ Environment variable
```

## Detection Pattern

Look for these error messages during WASM builds:

- `error: unexpected --cfg target_arch="wasm32" flag`
- `error: unexpected --cfg target_os="unknown" flag`
- `config target_arch is only supposed to be controlled by --target`

## Immediate Action

1. **Remove manual target flags** from `.cargo/config.toml`
2. **Keep only custom flags** like `wasm_js`
3. **Use environment variables** for custom configuration
4. **Let Rust toolchain handle** target-specific flags automatically

## Verification

After fixing, run:

```bash
wasm-pack test --headless --firefox
```

Should compile without `--cfg` flag errors.

## Related Rules

- `wasm-dependency-conflicts.md` - Other WASM build issues
- `test-script-targeting.md` - Test command organization
- `framework-test-command-separation.md` - Multi-framework testing

## Impact

- **Build failures** prevented
- **WASM testing** enabled
- **Development velocity** improved
- **Configuration complexity** reduced
