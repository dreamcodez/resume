# Visual Testing Robustness Plan for wasm-pack

## Overview

This document outlines a comprehensive plan to enhance the visual testing robustness in our wasm-pack solution. The core principle is simple: **no screenshot means update the snapshot, existing screenshot means compare**. When comparisons fail, sibling files are generated for debugging.

## Core Requirements

### 1. Authentic Native Rendering Capture

**The most critical requirement is using the lowest-level native rendering capture technique available to ensure AUTHENTIC reproduction of how components are rendered in the specific browser environment.**

This means:

- **Browser-native screenshot APIs** take precedence over synthetic rendering
- **Pixel-perfect accuracy** that matches what users actually see
- **Engine-specific rendering** (Blink, Gecko, WebKit) captured authentically
- **No approximations** or synthetic HTML-to-canvas conversions
- **True font rendering**, anti-aliasing, and sub-pixel positioning
- **Authentic color profiles** and gamma correction

### 2. Native Rendering Technique Hierarchy

**Priority order for screenshot capture (most authentic to least):**

1. **Chrome DevTools Protocol (CDP)** - `Page.captureScreenshot`

   - Direct access to browser's internal rendering pipeline
   - Pixel-perfect reproduction of actual rendered content
   - Available in Chromium-based browsers during testing
   - Captures exactly what the rendering engine produces

2. **WebDriver Screenshot API** - `takeScreenshot()`

   - Cross-browser native screenshot capability
   - Handled by browser driver (ChromeDriver, GeckoDriver, etc.)
   - Authentic browser rendering without synthetic reproduction
   - Works across different browser engines

3. **Screen Capture API** - `getDisplayMedia()`

   - Modern browser API for capturing actual screen content
   - Captures the literal pixels displayed by the browser
   - Requires user permission but most authentic for user-facing tests
   - Works with any browser that supports the API

4. **OffscreenCanvas with native context**

   - Hardware-accelerated rendering in dedicated thread
   - Uses browser's native 2D/3D rendering capabilities
   - Higher performance than main thread canvas
   - Still native browser rendering, just off-screen

5. **High-fidelity Canvas 2D** (fallback only)
   - Main thread canvas with careful attention to native behavior
   - Uses `getComputedStyle()` and `getBoundingClientRect()` for exact metrics
   - Preserves device pixel ratio and color space
   - Last resort when native APIs unavailable

### 3. Prohibited Approaches

**These approaches are explicitly avoided as they don't provide authentic rendering:**

- ❌ **HTML-to-Canvas libraries** without native fallback
- ❌ **Synthetic DOM parsing** and manual drawing
- ❌ **Server-side rendering** screenshots
- ❌ **Headless browser simulations** that approximate rendering
- ❌ **CSS-to-Canvas** conversion libraries
- ❌ **Any approximation** of browser rendering behavior

### 4. Rendering Fidelity Requirements

**Every screenshot must preserve:**

- **Exact pixel positioning** from browser's layout engine
- **True typography** with browser's font rendering and hinting
- **Authentic anti-aliasing** and sub-pixel rendering
- **Correct color spaces** (sRGB, P3, etc.)
- **Device pixel ratio** handling for high-DPI displays
- **Browser-specific rendering quirks** (Blink vs Gecko vs WebKit)
- **Hardware acceleration effects** when applicable
- **Actual CSS transform results** (not calculated approximations)

## Current State Analysis

### Existing Implementation

- Basic screenshot capture via JavaScript in `yew/src/tests/browser/js/screenshot.js`
- Reference images stored in `yew/src/tests/visual/reference` directory using the exact naming of the test file
- Tests which result in mismatches should fail the test and write the reference to the same location except with a postfix. for example front-page.png becomes front-page.change.<short-timestamp>.png
- Simple test in `yew/src/tests/visual/front-page.rs` that only validates screenshot length
- `tests` is a mod that is only loaded for visual testing
- No systematic comparison or snapshot management

### Identified Issues

1. **Inconsistent naming**: Test results use complex auto-generated names
2. **No comparison logic**: Only checks if screenshot exists, not content
3. **No update mechanism**: No clear way to update reference snapshots
4. **Poor debugging**: No diff images when tests fail
5. **Manual management**: Reference images must be manually managed

## Proposed Enhanced Architecture

### 1. Snapshot Management System

Create a central snapshot manager that handles all visual testing operations:

```rust
// New module: src/tests/visual/snapshot_manager.rs
pub struct SnapshotManager {
    test_name: String,
    reference_path: PathBuf,
    temp_current_path: PathBuf,
    mismatch_path: PathBuf,
    diff_path: PathBuf,
}

impl SnapshotManager {
    pub fn new(test_name: &str) -> Self {
        let reference_dir = PathBuf::from("src/tests/visual/reference");
        let temp_dir = PathBuf::from("src/tests/visual/temp");
        let timestamp = chrono::Utc::now().timestamp();

        Self {
            test_name: test_name.to_string(),
            reference_path: reference_dir.join(format!("{}.png", test_name)),
            temp_current_path: temp_dir.join(format!("{}-current.png", test_name)),
            mismatch_path: reference_dir.join(format!("{}.change.{}.png", test_name, timestamp)),
            diff_path: reference_dir.join(format!("{}.diff.{}.png", test_name, timestamp)),
        }
    }

    pub fn generate_short_timestamp() -> String {
        // Generate short timestamp like 1704123456
        chrono::Utc::now().timestamp().to_string()
    }
}
```

### 2. Test Naming Convention

**Standardized file naming pattern (building on existing):**

- **Reference snapshots**: `src/tests/visual/reference/{exact_test_name}.png`
- **Mismatch captures**: `src/tests/visual/reference/{exact_test_name}.change.{short_timestamp}.png`
- **Current captures**: `src/tests/visual/temp/{exact_test_name}-current.png`
- **Optional diff images**: `src/tests/visual/reference/{exact_test_name}.diff.{short_timestamp}.png`

**Examples:**

- `front-page.png` (reference snapshot)
- `front-page.change.1704123456.png` (mismatch capture with timestamp)
- `front-page-current.png` (temporary current capture in temp directory)
- `front-page.diff.1704123456.png` (optional visual diff highlighting changes)

### 3. Enhanced JavaScript Screenshot Module

Upgrade the screenshot capture to use the most native rendering capture techniques available:

```javascript
// Enhanced src/tests/browser/js/screenshot.js
export async function capture_visual_snapshot(testName, options = {}) {
  const screenshot = await captureAuthenticScreenshot(options);

  return {
    testName,
    imageData: screenshot,
    metadata: {
      timestamp: Date.now(),
      viewport: { width: window.innerWidth, height: window.innerHeight },
      userAgent: navigator.userAgent,
      renderingEngine: detectRenderingEngine(),
      pixelRatio: window.devicePixelRatio,
      colorDepth: window.screen.colorDepth,
    },
  };
}

async function captureAuthenticScreenshot(options = {}) {
  // Priority order: Most native -> Least native

  // 1. Chrome DevTools Protocol (most authentic for Chromium-based)
  if (window.chrome && window.chrome.debugger) {
    return await captureViaCDP(options);
  }

  // 2. Web Driver Screenshot API (if available in test environment)
  if (window.__webdriver_screenshot) {
    return await captureViaWebDriver(options);
  }

  // 3. Screen Capture API (modern browsers)
  if (navigator.mediaDevices && navigator.mediaDevices.getDisplayMedia) {
    return await captureViaScreenCapture(options);
  }

  // 4. OffscreenCanvas with native rendering context
  if (window.OffscreenCanvas) {
    return await captureViaOffscreenCanvas(options);
  }

  // 5. Canvas 2D with high-fidelity rendering (last resort)
  return await captureViaHighFidelityCanvas(options);
}

// Chrome DevTools Protocol - Most authentic for Chromium
async function captureViaCDP(options) {
  try {
    // Use CDP Page.captureScreenshot for pixel-perfect browser rendering
    const result = await new Promise((resolve, reject) => {
      chrome.debugger.sendCommand(
        { tabId: options.tabId },
        "Page.captureScreenshot",
        {
          format: "png",
          quality: 100,
          clip: options.clip,
          captureBeyondViewport: false,
        },
        resolve
      );
    });

    // Convert base64 to Uint8Array
    const binaryString = atob(result.data);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i);
    }
    return bytes;
  } catch (error) {
    console.warn("CDP capture failed:", error);
    throw error;
  }
}

// WebDriver screenshot - Cross-browser authentic capture
async function captureViaWebDriver(options) {
  try {
    // Use WebDriver's native screenshot capability
    const screenshot = await window.__webdriver_screenshot(options);
    return new Uint8Array(screenshot);
  } catch (error) {
    console.warn("WebDriver capture failed:", error);
    throw error;
  }
}

// High-fidelity canvas rendering - Authentic DOM representation
async function captureViaHighFidelityCanvas(options) {
  const canvas = document.createElement("canvas");
  const ctx = canvas.getContext("2d", {
    alpha: false,
    colorSpace: "srgb",
    imageSmoothingEnabled: false, // Preserve sharp pixels
  });

  // Set canvas to match viewport exactly with device pixel ratio
  canvas.width = window.innerWidth * window.devicePixelRatio;
  canvas.height = window.innerHeight * window.devicePixelRatio;
  canvas.style.width = window.innerWidth + "px";
  canvas.style.height = window.innerHeight + "px";

  // Scale context to handle device pixel ratio
  ctx.scale(window.devicePixelRatio, window.devicePixelRatio);

  // Render with maximum fidelity to native browser rendering
  await renderDOMWithNativeFidelity(ctx, document.body, options);

  return new Promise((resolve, reject) => {
    canvas.toBlob((blob) => {
      const reader = new FileReader();
      reader.onloadend = () => resolve(new Uint8Array(reader.result));
      reader.onerror = reject;
      reader.readAsArrayBuffer(blob);
    }, "image/png");
  });
}

// Helper to detect rendering engine for metadata
function detectRenderingEngine() {
  const ua = navigator.userAgent;
  if (ua.includes("Chrome")) return "Blink";
  if (ua.includes("Firefox")) return "Gecko";
  if (ua.includes("Safari") && !ua.includes("Chrome")) return "WebKit";
  return "Unknown";
}

// Native-fidelity DOM rendering that preserves authentic browser behavior
async function renderDOMWithNativeFidelity(ctx, element, options) {
  // Use the browser's own computed styles and layout metrics
  // This ensures we capture EXACTLY what the browser renders

  const walker = document.createTreeWalker(
    element,
    NodeFilter.SHOW_ELEMENT | NodeFilter.SHOW_TEXT,
    null,
    false
  );

  const elementsToRender = [];
  let node;

  // Collect all renderable elements with their exact computed properties
  while ((node = walker.nextNode())) {
    if (node.nodeType === Node.ELEMENT_NODE) {
      const style = window.getComputedStyle(node);
      const rect = node.getBoundingClientRect();

      if (
        style.display !== "none" &&
        style.visibility !== "hidden" &&
        rect.width > 0 &&
        rect.height > 0
      ) {
        elementsToRender.push({
          element: node,
          style: style,
          rect: rect,
          type: "element",
        });
      }
    } else if (node.nodeType === Node.TEXT_NODE && node.textContent.trim()) {
      const range = document.createRange();
      range.selectNode(node);
      const rect = range.getBoundingClientRect();

      if (rect.width > 0 && rect.height > 0) {
        elementsToRender.push({
          element: node,
          rect: rect,
          type: "text",
        });
      }
    }
  }

  // Render elements in document order to preserve layering
  for (const item of elementsToRender) {
    await renderSingleElementAuthentically(ctx, item);
  }
}

async function renderSingleElementAuthentically(ctx, item) {
  const { element, style, rect, type } = item;

  if (type === "text") {
    // Render text with parent element's computed font properties
    const parent = element.parentElement;
    const parentStyle = window.getComputedStyle(parent);

    ctx.font = `${parentStyle.fontStyle} ${parentStyle.fontVariant} ${parentStyle.fontWeight} ${parentStyle.fontSize} ${parentStyle.fontFamily}`;
    ctx.fillStyle = parentStyle.color;
    ctx.textAlign = "left";
    ctx.textBaseline = "top";

    // Use exact font metrics from the browser
    const text = element.textContent;
    ctx.fillText(text, rect.left, rect.top);

    return;
  }

  // Render element backgrounds with exact browser colors
  if (style.backgroundColor && style.backgroundColor !== "rgba(0, 0, 0, 0)") {
    ctx.fillStyle = style.backgroundColor;
    ctx.fillRect(rect.left, rect.top, rect.width, rect.height);
  }

  // Render borders with exact pixel precision
  const borderWidth = parseFloat(style.borderTopWidth) || 0;
  if (borderWidth > 0) {
    ctx.strokeStyle = style.borderTopColor;
    ctx.lineWidth = borderWidth;
    ctx.strokeRect(
      rect.left + borderWidth / 2,
      rect.top + borderWidth / 2,
      rect.width - borderWidth,
      rect.height - borderWidth
    );
  }

  // Handle images with their exact loaded content
  if (
    element.tagName === "IMG" &&
    element.complete &&
    element.naturalWidth > 0
  ) {
    ctx.drawImage(element, rect.left, rect.top, rect.width, rect.height);
  }

  // Handle canvas elements by copying their exact pixel data
  if (element.tagName === "CANVAS") {
    ctx.drawImage(element, rect.left, rect.top, rect.width, rect.height);
  }

  // Handle SVG elements (preserve vector rendering quality)
  if (element.tagName === "SVG") {
    // Convert SVG to image with original resolution
    const svgData = new XMLSerializer().serializeToString(element);
    const img = new Image();
    const svgBlob = new Blob([svgData], { type: "image/svg+xml" });
    const url = URL.createObjectURL(svgBlob);

    return new Promise((resolve) => {
      img.onload = () => {
        ctx.drawImage(img, rect.left, rect.top, rect.width, rect.height);
        URL.revokeObjectURL(url);
        resolve();
      };
      img.src = url;
    });
  }
}
```

### 4. Visual Test Framework

Introduce a macro-driven framework for easy test creation:

```rust
// New: src/tests/visual/framework.rs
use wasm_bindgen_test::*;

#[macro_export]
macro_rules! visual_test {
    ($test_name:ident, $render_fn:expr) => {
        paste::paste! {
            #[wasm_bindgen_test(async)]
            async fn [<visual_ $test_name>]() {
                let snapshot_manager = SnapshotManager::new(stringify!($test_name));

                // Render the component/page
                $render_fn().await;

                // Capture screenshot
                let current_screenshot = capture_visual_snapshot(
                    stringify!($test_name),
                    Default::default()
                ).await?;

                // Compare or update
                match snapshot_manager.process_snapshot(current_screenshot).await {
                    SnapshotResult::Match => {
                        // Test passes
                        snapshot_manager.cleanup_temporary_files().await;
                    },
                    SnapshotResult::NewSnapshot => {
                        // First run - snapshot saved
                        web_sys::console::log_1(&format!(
                            "New visual snapshot created: {}",
                            stringify!($test_name)
                        ).into());
                    },
                    SnapshotResult::Mismatch { diff_percentage } => {
                        // Generate diff image and fail test
                        panic!(
                            "Visual regression detected in {}: {:.2}% difference",
                            stringify!($test_name),
                            diff_percentage
                        );
                    }
                }
            }
        }
    };
}
```

### 5. Snapshot Comparison Logic

Implement intelligent comparison with configurable thresholds:

```rust
// src/tests/visual/comparison.rs
pub enum SnapshotResult {
    Match,
    NewSnapshot,
    Mismatch { diff_percentage: f64 },
}

impl SnapshotManager {
    pub async fn process_snapshot(&self, current: VisualSnapshot) -> SnapshotResult {
        // Check if reference exists
        if !self.reference_exists().await {
            // No reference = update mode
            self.save_as_reference(current).await;
            return SnapshotResult::NewSnapshot;
        }

        // Load reference and compare
        let reference = self.load_reference().await?;
        let comparison = self.compare_images(&reference, &current).await?;

                if comparison.is_match(VISUAL_THRESHOLD) {
            SnapshotResult::Match
        } else {
            // Save mismatch capture to reference directory with timestamp
            self.save_mismatch_capture(current).await;
            // Optionally save diff image to reference directory
            self.save_diff(comparison.diff_image).await;
            // Save current capture to temp directory for debugging
            self.save_temp_current(current).await;
            SnapshotResult::Mismatch {
                diff_percentage: comparison.difference_percentage
            }
        }
    }
}
```

### 6. Directory Structure

**Enhanced structure building on existing pattern:**

```
yew/
└── src/tests/visual/           # Visual testing module (only loaded for visual testing)
    ├── mod.rs                  # Main visual testing module
    ├── reference/              # Reference snapshots (existing pattern)
    │   ├── front-page.png                  # Reference snapshots using exact test names
    │   ├── home-page-mobile.png
    │   ├── navigation-component.png
    │   ├── resume-page-desktop.png
    │   ├── about-page-desktop.png
    │   ├── front-page.change.1704123456.png    # Mismatch captures with timestamp
    │   ├── home-page-mobile.change.1704123789.png
    │   └── navigation-component.diff.1704124000.png  # Optional diff images
    ├── temp/                   # Temporary files during test runs
    │   ├── front-page-current.png         # Current captures during comparison
    │   └── front-page-processing.png      # Processing files
    ├── framework.rs            # Test framework and macros
    ├── snapshot_manager.rs     # Snapshot file management
    ├── comparison.rs           # Image comparison logic
    ├── front-page.rs           # Individual test files
    ├── home-page-mobile.rs     # Mobile responsive tests
    ├── navigation-component.rs # Component-specific tests
    └── resume-page-desktop.rs  # Page-specific tests
```

### 7. Module Structure & Loading

**Visual testing module configuration:**

```rust
// src/tests/mod.rs - Only loaded for visual testing
#[cfg(feature = "visual-tests")]
pub mod visual;

// src/tests/visual/mod.rs - Main visual testing module
pub mod framework;
pub mod snapshot_manager;
pub mod comparison;

// Individual test files
pub mod front_page;
pub mod home_page_mobile;
pub mod navigation_component;
pub mod resume_page_desktop;
```

**Cargo.toml feature configuration:**

```toml
[features]
default = []
visual-tests = ["wasm-bindgen-test", "image", "paste"]

[dependencies]
# ... existing dependencies

[dev-dependencies]
wasm-bindgen-test = { version = "0.3", optional = true }
image = { version = "0.24", optional = true }
paste = { version = "1.0", optional = true }
```

### 8. Usage Examples

**Simple test creation:**

```rust
// src/tests/visual/front-page.rs
use crate::tests::visual::framework::*;

visual_test!(front_page, || async {
    // Navigate to home page
    navigate_to("/").await;
    wait_for_load().await;
});

// src/tests/visual/home-page-mobile.rs
use crate::tests::visual::framework::*;

visual_test!(home_page_mobile, || async {
    // Set mobile viewport
    set_viewport(375, 667).await;
    navigate_to("/").await;
    wait_for_load().await;
});

// src/tests/visual/navigation-component.rs
use crate::tests::visual::framework::*;

visual_test!(navigation_component, || async {
    // Render just the navigation
    render_component::<Nav>().await;
});
```

### 9. Configuration & Environment Support

**Flexible configuration system:**

```rust
// src/tests/visual/config.rs
pub struct VisualTestConfig {
    pub threshold: f64,           // Pixel difference threshold (0.0-1.0)
    pub update_snapshots: bool,   // Force update mode
    pub fail_on_missing: bool,    // Fail if no reference exists
    pub cleanup_temp: bool,       // Clean up temporary files
}

impl Default for VisualTestConfig {
    fn default() -> Self {
        Self {
            threshold: 0.02,      // 2% difference allowed
            update_snapshots: std::env::var("UPDATE_SNAPSHOTS").is_ok(),
            fail_on_missing: false,
            cleanup_temp: true,
        }
    }
}
```

### 10. wasm-pack Native Rendering Integration

**Special considerations for wasm-pack testing environment:**

```rust
// src/tests/visual/wasm_native.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

// Enable native screenshot APIs in wasm-pack environment
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__wasm_test_harness"])]
    static NATIVE_SCREENSHOT: bool;

    #[wasm_bindgen(js_namespace = ["window", "__wasm_test_harness"], catch)]
    async fn capture_native_screenshot(options: JsValue) -> Result<JsValue, JsValue>;
}

// Integration with native test runner capabilities
pub async fn capture_wasm_native_screenshot(test_name: &str) -> Result<Vec<u8>, JsValue> {
    if unsafe { NATIVE_SCREENSHOT } {
        // Use test harness native screenshot capability
        let options = js_sys::Object::new();
        js_sys::Reflect::set(&options, &"testName".into(), &test_name.into())?;
        js_sys::Reflect::set(&options, &"format".into(), &"png".into())?;
        js_sys::Reflect::set(&options, &"quality".into(), &1.0.into())?;

        let result = capture_native_screenshot(options.into()).await?;
        let uint8_array = js_sys::Uint8Array::from(result);
        Ok(uint8_array.to_vec())
    } else {
        // Fall back to JavaScript-based capture
        let promise = capture_visual_snapshot(test_name, js_sys::Object::new().into());
        let result = JsFuture::from(promise).await?;
        let data = js_sys::Reflect::get(&result, &"imageData".into())?;
        let uint8_array = js_sys::Uint8Array::from(data);
        Ok(uint8_array.to_vec())
    }
}
```

**Test harness configuration:**

```javascript
// Enhanced test harness setup for wasm-pack
// This runs before wasm-pack tests to enable native screenshot APIs

window.__wasm_test_harness = {
  NATIVE_SCREENSHOT:
    typeof window.chrome !== "undefined" &&
    typeof window.chrome.debugger !== "undefined",

  async capture_native_screenshot(options) {
    const testName = options.testName;
    const format = options.format || "png";
    const quality = options.quality || 1.0;

    // Use the most native API available in this environment
    if (window.chrome && window.chrome.debugger) {
      return await this.captureViaCDP(options);
    } else if (window.__webdriver && window.__webdriver.takeScreenshot) {
      return await this.captureViaWebDriver(options);
    } else {
      throw new Error("No native screenshot API available");
    }
  },

  async captureViaCDP(options) {
    const tabId = await this.getCurrentTabId();
    return new Promise((resolve, reject) => {
      chrome.debugger.sendCommand(
        { tabId },
        "Page.captureScreenshot",
        {
          format: options.format,
          quality: Math.round(options.quality * 100),
          captureBeyondViewport: false,
        },
        (result) => {
          if (chrome.runtime.lastError) {
            reject(new Error(chrome.runtime.lastError.message));
          } else {
            const binaryString = atob(result.data);
            const bytes = new Uint8Array(binaryString.length);
            for (let i = 0; i < binaryString.length; i++) {
              bytes[i] = binaryString.charCodeAt(i);
            }
            resolve(bytes);
          }
        }
      );
    });
  },

  async getCurrentTabId() {
    return new Promise((resolve) => {
      chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
        resolve(tabs[0].id);
      });
    });
  },
};
```

### 11. Dependencies

**Required additions to Cargo.toml:**

```toml
[dev-dependencies]
wasm-bindgen-test = "0.3"
image = "0.24"              # For image processing and comparison
paste = "1.0"               # For macro magic
tokio = "1.0"              # For async file operations
js-sys = "0.3"             # For JavaScript interop
wasm-bindgen-futures = "0.4" # For Promise handling
pixelmatch = "0.1"         # Native pixel comparison (if available)

[dependencies.web-sys]
version = "0.3"
features = [
  "CanvasRenderingContext2d",
  "HtmlCanvasElement",
  "ImageData",
  "OffscreenCanvas",
  "OffscreenCanvasRenderingContext2d",
  "MediaDevices",
  "MediaStream",
  "MediaStreamConstraints",
  "DisplayMediaStreamConstraints"
]
```

### 12. Test Runner Integration

**Enhanced test runner script with native rendering support:**

```bash
#!/bin/bash
# bin/visual-test.sh

echo "Running visual regression tests with native rendering capture..."

# Build the project first
trunk build

# Set up native screenshot capabilities
export ENABLE_NATIVE_SCREENSHOTS=1
export CHROME_DEBUG_PORT=9222

# Choose browser based on native capabilities
BROWSER="chrome"
BROWSER_FLAGS="--enable-automation --disable-background-timer-throttling --disable-backgrounding-occluded-windows --disable-renderer-backgrounding --disable-features=TranslateUI --disable-ipc-flooding-protection --enable-features=NetworkService,NetworkServiceLogging --disable-extensions --disable-component-extensions-with-background-pages --remote-debugging-port=9222"

if [ "$1" = "--update" ]; then
    echo "UPDATE MODE: Updating all snapshots with native rendering..."
    UPDATE_SNAPSHOTS=1 wasm-pack test --chrome --headless -- --test visual
elif [ "$1" = "--debug" ]; then
    echo "DEBUG MODE: Running with visible browser for debugging native rendering..."
    wasm-pack test --chrome -- --test visual
else
    echo "COMPARE MODE: Comparing against existing snapshots with native rendering..."
    wasm-pack test --chrome --headless -- --test visual
fi

# Run across multiple browsers for cross-browser native rendering validation
if [ "$1" = "--cross-browser" ]; then
    echo "CROSS-BROWSER MODE: Testing native rendering across engines..."

    echo "Testing with Chromium (Blink engine)..."
    wasm-pack test --chrome --headless -- --test visual

    echo "Testing with Firefox (Gecko engine)..."
    wasm-pack test --firefox --headless -- --test visual

    # Note: Safari testing would require different setup
    # echo "Testing with Safari (WebKit engine)..."
    # wasm-pack test --safari --headless -- --test visual
fi

# Clean up temporary files
rm -rf yew/src/tests/visual/temp/

echo "Native rendering visual tests completed."
```

**Package.json script updates:**

```json
{
  "scripts": {
    "test:visual": "./bin/visual-test.sh",
    "test:visual:update": "./bin/visual-test.sh --update",
    "test:visual:debug": "./bin/visual-test.sh --debug",
    "test:visual:cross-browser": "./bin/visual-test.sh --cross-browser"
  }
}
```

**Native rendering validation script:**

```bash
#!/bin/bash
# bin/validate-native-rendering.sh

echo "Validating native rendering capabilities..."

# Check Chrome DevTools Protocol availability
if command -v google-chrome &> /dev/null; then
    echo "✓ Chrome available for CDP screenshots"
    google-chrome --version
else
    echo "⚠️  Chrome not available - CDP screenshots disabled"
fi

# Check WebDriver availability
if command -v chromedriver &> /dev/null; then
    echo "✓ ChromeDriver available for WebDriver screenshots"
    chromedriver --version
else
    echo "⚠️  ChromeDriver not available - WebDriver screenshots disabled"
fi

if command -v geckodriver &> /dev/null; then
    echo "✓ GeckoDriver available for Firefox WebDriver screenshots"
    geckodriver --version
else
    echo "⚠️  GeckoDriver not available - Firefox WebDriver screenshots disabled"
fi

# Test basic wasm-pack functionality
echo "Testing wasm-pack native rendering integration..."
cd yew
wasm-pack test --chrome --headless -- --test native_rendering_validation

echo "Native rendering validation completed."
```

## Implementation Phases

### Phase 1: Foundation (Week 1)

1. Create directory structure
2. Implement `SnapshotManager`
3. Enhance JavaScript screenshot module
4. Basic file I/O operations

### Phase 2: Comparison Engine (Week 2)

1. Implement image comparison logic
2. Create diff image generation
3. Configure thresholds and tolerances
4. Add metadata tracking

### Phase 3: Test Framework (Week 3)

1. Create visual test macros
2. Implement test helper functions
3. Add viewport management
4. Create component rendering utilities

### Phase 4: Integration & Polish (Week 4)

1. Integrate with existing test suite
2. Create test runner scripts
3. Add documentation and examples
4. Migrate existing visual tests

## Benefits of This Approach

### 1. **Clarity & Simplicity**

- Each test has exactly one snapshot file named after the test
- Clear visual indication of what each file represents
- Intuitive update vs. compare logic

### 2. **Debugging Support**

- Failed tests generate current and diff images
- Easy to see exactly what changed
- Metadata helps identify environmental issues

### 3. **Developer Experience**

- Macro-driven test creation reduces boilerplate
- Environment variables control behavior
- Automatic cleanup of temporary files

### 4. **Maintainability**

- Centralized snapshot management
- Configurable thresholds and behavior
- Rich metadata for troubleshooting

### 5. **Scalability**

- Easy to add new tests
- Supports component and page-level testing
- Handles responsive design testing

## Migration Strategy

### From Current System

1. **Audit existing tests**: Identify all current visual tests
2. **Migrate reference images**: Move from `reference-visuals/` to `visual-snapshots/`
3. **Rename systematically**: Use consistent naming convention
4. **Update test code**: Convert to new macro-based system
5. **Validate results**: Ensure all tests pass with new system

### Rollback Plan

- Keep existing system parallel during migration
- Use feature flags to switch between systems
- Maintain backward compatibility until migration complete

## Success Metrics

### Quantitative

- **Test creation time**: Reduce from 30 minutes to 5 minutes per test
- **Debug time**: Reduce visual regression debugging by 80%
- **False positives**: Less than 2% false positive rate
- **Coverage**: Achieve 90% visual coverage of critical user flows

### Qualitative

- **Developer satisfaction**: Easy to create and maintain tests
- **Reliability**: Consistent results across environments
- **Debugging**: Clear understanding of what changed
- **Maintenance**: Minimal ongoing maintenance required

## Risk Mitigation

### Technical Risks

- **wasm-pack limitations**: Thorough testing of screenshot capabilities
- **Browser compatibility**: Test across target browsers
- **Performance impact**: Benchmark test execution times
- **File size management**: Monitor snapshot file sizes

### Process Risks

- **Team adoption**: Provide training and documentation
- **Migration complexity**: Phased rollout with validation
- **Maintenance burden**: Automated cleanup and management

## Conclusion

This visual testing robustness plan transforms our basic screenshot testing into a comprehensive visual regression testing system. By following the core principle of "no screenshot means update, existing means compare" and generating sibling files for failed comparisons, we create a system that is both intuitive and powerful.

The macro-driven approach reduces boilerplate while the centralized snapshot management ensures consistency. The result is a maintainable, scalable visual testing system that provides clear feedback and supports rapid development cycles.
