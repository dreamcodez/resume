# Test File Organization Patterns

## Context

Complex projects with multiple frameworks and test types require clear file organization to prevent confusion and ensure maintainability.

## Rule: Separate Test Types by Framework and Purpose

### When to Apply

- Working in multi-framework projects
- Adding new test types or frameworks
- When test discovery becomes confusing
- Before creating new test files

### Directory Structure Pattern

```
project/
├── src/
│   └── tests/                    # Unit tests (same framework as src)
│       ├── browser/             # Browser-specific unit tests
│       │   ├── capture.rs       # Screenshot capture logic
│       │   ├── compare.rs       # Image comparison logic
│       │   └── mod.rs
│       └── visual.rs            # Visual unit tests
├── tests/                       # Integration tests (separate from src)
│   ├── chrome_screenshot.rs     # Chrome integration tests
│   ├── helpers/                 # Reusable test helpers
│   │   └── mod.rs
│   └── reference-screenshots/   # Visual regression artifacts
└── cypress/                     # E2E tests (different framework)
    └── integration/
```

### Test Type Separation

#### **Unit Tests (`src/tests/`)**

```rust
// Same framework as source code
// Fast, isolated, no external dependencies
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_rendering() {
        // Unit test logic
    }
}
```

#### **Integration Tests (`tests/`)**

```rust
// Separate from source code
// Can use external dependencies
// Run with: cargo test --test <filename>
#[test]
fn test_chrome_screenshot() {
    // Integration test logic
}
```

#### **Framework-Specific Tests**

```bash
# Yew tests (Rust/WASM)
cd yew/
cargo test                    # Unit tests
cargo test --test chrome_screenshot  # Integration tests

# Sapper tests (Node.js)
cd ../
npm run test                 # Unit + E2E tests
npx playwright test         # Visual tests
```

### File Naming Conventions

#### **Unit Tests**

- `src/tests/` - Same directory as source
- `mod.rs` - Module definitions
- `*_test.rs` - Test files (optional)

#### **Integration Tests**

- `tests/` - Separate directory
- `*_test.rs` - Test files
- `helpers/` - Reusable test utilities

#### **Visual Regression**

- `tests/reference-screenshots/` - Artifacts
- `*-reference.png` - Baseline images
- `*-current.png` - Current screenshots
- `*-diff.png` - Difference images

### Helper Organization

#### **Reusable Helpers (`tests/helpers/mod.rs`)**

```rust
// Cross-test utilities
pub fn launch_browser() -> Browser { /* ... */ }
pub fn capture_screenshot(tab: &Tab) -> Vec<u8> { /* ... */ }
pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage) -> bool { /* ... */ }
```

#### **Test-Specific Helpers**

```rust
// In individual test files
mod helpers;

#[test]
fn test_specific_functionality() {
    let helper = helpers::SpecificHelper::new();
    // Test logic
}
```

### Common Patterns

#### **Browser Tests**

```rust
// tests/chrome_screenshot.rs
mod helpers;

#[test]
fn test_homepage_screenshot() {
    let (browser, tab) = helpers::launch_and_navigate("http://localhost:8080/");
    // Test logic
}
```

#### **Unit Tests**

```rust
// src/components/button/tests/interactions.rs
use super::*;

#[test]
fn test_button_click() {
    // Unit test logic
}
```

### Error Prevention

**❌ Don't:**

- Mix unit and integration tests in same directory
- Use unclear file names
- Put framework-specific tests in wrong directories
- Create circular dependencies between test helpers

**✅ Do:**

- Use clear directory separation
- Follow consistent naming conventions
- Organize helpers by reusability
- Document test organization patterns

### Examples from Today's Session

**✅ Correct Organization:**

```
yew/
├── src/tests/browser/          # Unit tests for browser logic
│   ├── capture.rs
│   ├── compare.rs
│   └── mod.rs
├── tests/                      # Integration tests
│   ├── chrome_screenshot.rs
│   ├── helpers/mod.rs
│   └── reference-screenshots/
└── cypress/                    # E2E tests (different framework)
```

## Impact

- Prevents test discovery confusion
- Enables clear separation of concerns
- Improves maintainability and debugging
- Facilitates CI/CD pipeline organization
