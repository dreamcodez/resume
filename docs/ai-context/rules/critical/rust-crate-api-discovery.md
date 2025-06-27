# Rust Crate API Discovery and Usage

## Context

When working with Rust crates, especially those with complex APIs or version differences, efficient API discovery prevents repeated compilation errors and saves significant time.

## Rule: Always Use Cargo Check for API Discovery

### When to Apply

- Working with new or unfamiliar Rust crates
- When compilation errors indicate API changes
- Before implementing complex functionality

### Actionable Steps

1. **Use `cargo check` for Fast API Validation**

   ```bash
   # Instead of full compilation, use check for API validation
   cargo check --tests
   ```

2. **Check Crate Documentation First**

   ```bash
   # Generate and open crate documentation
   cargo doc --open --package <crate-name>

   # For specific crates like headless_chrome
   cargo doc --open --package headless_chrome
   ```

3. **Use Cargo Search for API Clues**

   ```bash
   # Search for specific methods or types
   cargo search <method-name>
   ```

4. **Check Crate Examples**
   ```bash
   # Look for examples in the crate's repository
   # Or check if crate has built-in examples
   cargo run --example <example-name>
   ```

### Common Patterns

**For headless_chrome specifically:**

- API paths often change between versions
- Use `headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png`
- Tab is wrapped in `Arc<Tab>`
- Method signatures: `capture_screenshot(format, clip, quality, from_surface)`

**For image processing:**

- Use `Rgba<u8>` for ImageBuffer, not `Rgba<i32>`
- Import from `image::{DynamicImage, GenericImageView, ImageBuffer, Rgba}`

### Error Prevention

- Don't guess API paths - use `cargo check` to validate
- When imports fail, check multiple possible paths systematically
- Use type annotations to help compiler provide better error messages

### Examples from Today's Session

```rust
// ❌ Wrong - multiple attempts with different import paths
use headless_chrome::browser::tab::ScreenshotFormat;
use headless_chrome::types::ScreenshotFormat;
use headless_chrome::protocol::page::ScreenshotFormat;

// ✅ Correct - discovered via cargo check
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
```

## Impact

- Prevents 3-5 compilation cycles per API discovery
- Reduces time spent on import path guessing
- Ensures correct API usage from the start
