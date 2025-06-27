# Visual Regression Testing Setup Pattern

## Testing Strategy

Establish a systematic approach for visual regression testing in WASM applications using canvas-based screenshot capture and JavaScript interop.

## Problem Scenario

Traditional visual testing approaches using native browser automation tools (like headless Chrome) don't work in WASM environments, requiring a different approach using web APIs and canvas.

## Setup Pattern

### 1. Canvas-Based Screenshot Capture

```rust
// tests/visual.rs
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlCanvasElement, Window};

#[wasm_bindgen]
pub fn capture_screenshot() -> Result<String, JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    // Create canvas element
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    // Set canvas size to body size
    let body_rect = body.get_bounding_client_rect();
    canvas.set_width(body_rect.width() as u32);
    canvas.set_height(body_rect.height() as u32);

    // Get canvas context and draw body
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    // Convert body to data URL
    let data_url = canvas.to_data_url()?;
    Ok(data_url)
}
```

### 2. JavaScript Interop for Screenshot

```javascript
// tests/browser/js/screenshot.js
export function captureScreenshot() {
  return new Promise((resolve) => {
    // Wait for page to render
    setTimeout(() => {
      const canvas = document.createElement("canvas");
      const body = document.body;
      const rect = body.getBoundingClientRect();

      canvas.width = rect.width;
      canvas.height = rect.height;

      const ctx = canvas.getContext("2d");
      ctx.drawWindow(window, 0, 0, rect.width, rect.height, "rgb(255,255,255)");

      const dataUrl = canvas.toDataURL();
      resolve(dataUrl);
    }, 100);
  });
}
```

### 3. Test Integration

```rust
// tests/visual.rs
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_home_page_screenshot() {
        // Navigate to home page
        let window = web_sys::window().unwrap();
        let location = window.location();
        location.set_hash("#/").unwrap();

        // Wait for render
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Capture screenshot
        let screenshot = capture_screenshot().unwrap();

        // Log screenshot size as basic regression check
        let base64_data = screenshot.split(',').nth(1).unwrap();
        let decoded = base64::decode(base64_data).unwrap();
        println!("Screenshot size: {} bytes", decoded.len());

        // Basic assertions
        assert!(decoded.len() > 1000, "Screenshot too small");
        assert!(screenshot.starts_with("data:image/png;base64,"));
    }
}
```

## Command Pattern

### Test Execution

```bash
# Run visual regression tests
wasm-pack test --headless --firefox tests/visual.rs

# Run with specific test
wasm-pack test --headless --firefox tests/visual.rs::tests::test_home_page_screenshot
```

### Package.json Scripts

```json
{
  "scripts": {
    "test:visual": "wasm-pack test --headless --firefox tests/visual.rs",
    "test:browser": "wasm-pack test --headless --firefox",
    "test:unit": "cargo test --lib"
  }
}
```

## File Organization

### Directory Structure

```
tests/
├── visual.rs                    # Main visual test file
├── browser/
│   └── js/
│       └── screenshot.js        # JavaScript screenshot utilities
└── reference-screenshots/       # Baseline images
    ├── home-page-reference.png
    └── about-page-reference.png
```

### Test Categories

```rust
// tests/visual.rs
mod home_page_tests {
    #[wasm_bindgen_test]
    fn test_home_page_layout() { /* ... */ }

    #[wasm_bindgen_test]
    fn test_home_page_mobile() { /* ... */ }
}

mod component_tests {
    #[wasm_bindgen_test]
    fn test_navigation_bar() { /* ... */ }

    #[wasm_bindgen_test]
    fn test_button_styles() { /* ... */ }
}
```

## Comparison Strategy

### Basic Size Comparison

```rust
fn compare_screenshot_size(current: &str, expected_size: usize) -> bool {
    let base64_data = current.split(',').nth(1).unwrap();
    let decoded = base64::decode(base64_data).unwrap();
    let size_diff = (decoded.len() as i32 - expected_size as i32).abs();
    size_diff < 1000 // Allow 1KB variance
}
```

### Future Enhancement: Image Comparison

```rust
// TODO: Implement pixel-by-pixel comparison
fn compare_screenshots(current: &str, reference: &str) -> f64 {
    // Decode base64 images
    // Compare pixel values
    // Return similarity percentage
}
```

## Error Handling

### Common Issues

```rust
// Handle canvas not available
if canvas.get_context("2d").is_err() {
    eprintln!("Canvas 2D context not available");
    return Err("Canvas error".into());
}

// Handle empty page
if body.get_bounding_client_rect().width() == 0.0 {
    eprintln!("Page has zero width");
    return Err("Empty page".into());
}
```

## Performance Considerations

### Screenshot Optimization

- **Canvas size**: Match viewport exactly
- **Format**: Use PNG for quality, JPEG for size
- **Timing**: Wait for animations to complete
- **Memory**: Clean up canvas after capture

### Test Execution

- **Parallel**: Run tests in parallel when possible
- **Caching**: Cache reference screenshots
- **Incremental**: Only test changed components

## Related Rules

- `test-script-targeting.md` - Test organization
- `framework-test-command-separation.md` - Multi-framework testing
- `reference-screenshot-management.md` - Screenshot organization

## Success Metrics

- ✅ Screenshots capture successfully
- ✅ Tests run in headless browser
- ✅ Basic regression detection works
- ✅ Performance acceptable (< 5s per test)
