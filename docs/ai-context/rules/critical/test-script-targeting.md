# Critical Rule: Test Script Targeting and Separation

## Context

The project uses a specific test script structure to separate unit tests from browser tests, with clear targeting to avoid confusion and ensure proper test execution.

## Current Test Script Structure

### Test Scripts (package.json)

```json
{
  "test": "npm run test:all", // Default: runs all tests
  "test:unit": "cargo test --lib --exclude browser", // Regular Rust unit tests only
  "test:browser": "wasm-pack test --headless --firefox --no-default-features -- src/tests/browser", // Browser tests only
  "test:all": "npm run test:unit && npm run test:browser" // Both test suites
}
```

### Test Targeting Decisions

1. **`test:unit`** - Targets regular Rust unit tests only

   - Command: `cargo test --lib --exclude browser`
   - Purpose: Fast execution of pure Rust logic tests
   - Excludes: Browser-based tests to avoid WASM compilation overhead

2. **`test:browser`** - Targets browser tests in specific directory

   - Command: `wasm-pack test --headless --firefox --no-default-features -- src/tests/browser`
   - Purpose: Real browser testing via WASM in Firefox
   - Location: Tests must be in `yew/src/tests/browser/` to be included

3. **`test:all`** - Runs both test suites sequentially
   - Purpose: Comprehensive testing before commits/deployment
   - Order: Unit tests first (fast), then browser tests (slower)

## When to Apply

- When adding new test scripts
- When modifying test execution patterns
- When troubleshooting test discovery issues
- When setting up CI/CD pipelines

## Why This Matters

- **Clear separation** prevents confusion between unit and browser tests
- **Specific targeting** ensures tests run in the correct environment
- **Performance optimization** allows fast unit test iteration during development
- **Comprehensive coverage** ensures all tests run before important milestones

## Current Browser Test Structure

```
yew/src/tests/browser/
├── mod.rs           # Browser test module exports
├── capture.rs       # Screenshot capture utilities
├── compare.rs       # Visual comparison utilities
├── front_page.rs    # Front page visual regression tests
├── js/              # JavaScript utilities for browser tests
└── reference/       # Reference screenshots for visual regression
```

## Potential Future Considerations

- **Cross-browser testing**: Currently Firefox-only, could expand to Chrome/WebKit
- **Test parallelization**: Browser tests could potentially run in parallel
- **Test categorization**: Could add more specific test categories (e.g., `test:visual`, `test:integration`)
- **Performance optimization**: Could implement test caching or incremental testing

## Related Rules

- [Visual Reference Update](./visual-reference-update.md)
- [WASM Dependency Removal Checklist](../workflow/wasm-dependency-removal-checklist.md)
- [Test Module Discovery](../testing/rust-test-module-discovery.md)
