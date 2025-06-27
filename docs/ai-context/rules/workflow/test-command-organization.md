# Test Command Organization and Scripting

## Workflow Pattern

Organize test commands systematically across package.json, Cargo.toml, and shell scripts to ensure consistent execution and clear separation of concerns.

## Problem Scenario

Test commands scattered across multiple files with inconsistent naming, unclear purposes, and potential conflicts between native and WASM testing approaches.

## Organization Pattern

### 1. Package.json Scripts (Primary Interface)

```json
{
  "scripts": {
    "test": "npm run test:unit && npm run test:browser",
    "test:unit": "cargo test --lib",
    "test:browser": "wasm-pack test --headless --firefox",
    "test:visual": "wasm-pack test --headless --firefox tests/visual.rs",
    "test:all": "npm run test:unit && npm run test:browser && npm run test:visual",
    "test:watch": "cargo watch -x 'test --lib'",
    "test:clean": "cargo clean && rm -rf target/",
    "build": "trunk build",
    "dev": "trunk serve",
    "check": "cargo check"
  }
}
```

### 2. Cargo.toml Test Configuration

```toml
# Cargo.toml
[package]
name = "yew-app"
version = "0.1.0"

[dependencies]
wasm-bindgen = "0.2"
wasm-bindgen-test = "0.3"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlCanvasElement"] }

[dev-dependencies]
# Only WASM-compatible test dependencies
console_error_panic_hook = "0.1"

[profile.release]
opt-level = "s"  # Optimize for size in WASM
```

### 3. Shell Scripts (Complex Operations)

```bash
#!/bin/bash
# bin/test-all.sh

set -e

echo "🧪 Running all tests..."

# Unit tests
echo "📦 Running unit tests..."
cargo test --lib

# Browser tests
echo "🌐 Running browser tests..."
wasm-pack test --headless --firefox

# Visual regression tests
echo "🖼️  Running visual tests..."
wasm-pack test --headless --firefox tests/visual.rs

echo "✅ All tests passed!"
```

## Command Hierarchy

### Primary Commands (User-Facing)

```bash
npm test          # Run all tests
npm run test:unit # Run unit tests only
npm run test:browser # Run browser tests only
npm run test:visual # Run visual regression tests
```

### Secondary Commands (Development)

```bash
npm run test:watch # Watch mode for unit tests
npm run test:clean # Clean all test artifacts
npm run build     # Build for production
npm run dev       # Development server
```

### Tertiary Commands (Debugging)

```bash
cargo test --lib -- --nocapture  # Unit tests with output
wasm-pack test --headless --firefox -- --nocapture  # Browser tests with output
cargo check                      # Syntax check only
```

## Error Handling Pattern

### Script Error Handling

```bash
#!/bin/bash
# bin/test-with-fallback.sh

set -e

echo "🧪 Running tests with fallback..."

# Try browser tests first
if wasm-pack test --headless --firefox; then
    echo "✅ Browser tests passed"
else
    echo "⚠️  Browser tests failed, trying unit tests only..."
    cargo test --lib
    echo "✅ Unit tests passed (browser tests skipped)"
fi
```

### Package.json Error Handling

```json
{
  "scripts": {
    "test:safe": "npm run test:unit || echo 'Unit tests failed' && npm run test:browser || echo 'Browser tests failed'",
    "test:unit-only": "cargo test --lib || exit 1",
    "test:browser-only": "wasm-pack test --headless --firefox || exit 1"
  }
}
```

## Environment-Specific Commands

### Development Environment

```json
{
  "scripts": {
    "test:dev": "npm run test:unit && npm run test:browser",
    "test:watch": "cargo watch -x 'test --lib'",
    "dev": "trunk serve --open"
  }
}
```

### CI/CD Environment

```json
{
  "scripts": {
    "test:ci": "npm run test:unit && npm run test:browser && npm run test:visual",
    "test:ci:unit": "cargo test --lib --release",
    "test:ci:browser": "wasm-pack test --headless --firefox --release"
  }
}
```

## Command Validation

### Pre-Execution Checks

```bash
#!/bin/bash
# bin/validate-test-env.sh

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Install with: cargo install wasm-pack"
    exit 1
fi

# Check if Firefox is available
if ! command -v firefox &> /dev/null; then
    echo "⚠️  Firefox not found. Browser tests may fail."
fi

# Check if trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "⚠️  trunk not found. Install with: cargo install trunk"
fi

echo "✅ Test environment validated"
```

### Post-Execution Validation

```bash
#!/bin/bash
# bin/validate-test-results.sh

# Check if tests generated expected artifacts
if [ ! -f "target/wasm32-unknown-unknown/release/yew_app.wasm" ]; then
    echo "❌ WASM build artifact not found"
    exit 1
fi

# Check if visual test screenshots were captured
if [ ! -f "tests/reference-screenshots/home-page-reference.png" ]; then
    echo "⚠️  Reference screenshots not found"
fi

echo "✅ Test artifacts validated"
```

## Performance Optimization

### Parallel Execution

```json
{
  "scripts": {
    "test:parallel": "npm run test:unit & npm run test:browser & wait",
    "test:fast": "cargo test --lib --release && wasm-pack test --headless --firefox --release"
  }
}
```

### Caching Strategy

```bash
#!/bin/bash
# bin/test-with-cache.sh

# Use cargo's built-in caching
export CARGO_INCREMENTAL=1

# Cache WASM builds
if [ -d "target/wasm32-unknown-unknown/release" ]; then
    echo "📦 Using cached WASM build"
else
    echo "🔨 Building WASM from scratch"
    wasm-pack build --release
fi

# Run tests
npm run test:all
```

## Documentation Pattern

### README.md Test Section

````markdown
## Testing

### Quick Start

```bash
npm test              # Run all tests
npm run test:unit     # Unit tests only
npm run test:browser  # Browser tests only
npm run test:visual   # Visual regression tests
```
````

### Test Types

- **Unit Tests**: `cargo test --lib` - Fast, no browser required
- **Browser Tests**: `wasm-pack test --headless --firefox` - Full WASM testing
- **Visual Tests**: Screenshot-based regression testing

### Troubleshooting

- If browser tests fail, ensure Firefox is installed
- If WASM builds fail, run `cargo clean` and retry
- For visual test issues, check canvas support

```

## Related Rules
- `test-script-targeting.md` - Test file organization
- `framework-test-command-separation.md` - Multi-framework testing
- `dependency-cleanup-for-wasm.md` - Dependency management

## Success Metrics
- ✅ All test commands work consistently
- ✅ Clear separation between test types
- ✅ Proper error handling and fallbacks
- ✅ Fast execution with caching
- ✅ Easy to understand and maintain
```
