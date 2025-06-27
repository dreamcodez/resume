# Visual Regression Strategy Evolution

## Context

Visual regression testing often starts with simple approaches but must evolve to robust, production-ready solutions. Understanding the progression prevents wasted effort on inadequate solutions.

## Rule: Start with Real Browser Protocol, Not In-Browser Hacks

### When to Apply

- Implementing visual regression testing
- Evaluating screenshot capture strategies
- When in-browser solutions fail or are inadequate
- Before investing time in complex workarounds

### Strategy Progression (Avoid Common Pitfalls)

#### ❌ **Phase 1: In-Browser Hacks (Avoid)**

```javascript
// Don't start here - these are inadequate for real testing
- html2canvas (renders differently than browser)
- Canvas-based DOM rendering (inaccurate)
- SVG foreignObject (security restrictions)
- Screen Capture API (requires user permission)
```

#### ❌ **Phase 2: Browser Limitations (Recognize)**

```javascript
// These approaches have fundamental limitations
- No access to browser's native screenshot APIs from JS
- Canvas cannot capture full browser rendering
- Security restrictions prevent pixel-perfect capture
- Inconsistent across browsers and environments
```

#### ✅ **Phase 3: Protocol-Driven Approach (Use)**

```rust
// This is the correct approach from the start
use headless_chrome::{Browser, Tab, LaunchOptionsBuilder};
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;

// Launch real browser, use DevTools Protocol
let browser = Browser::new(LaunchOptionsBuilder::default().headless(true).build()?)?;
let tab = browser.new_tab()?;
tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
```

### Actionable Implementation Steps

1. **Start with Protocol-Driven Tools**

   ```rust
   // Use these from the beginning:
   - headless_chrome (Rust)
   - Playwright (Node.js)
   - Puppeteer (Node.js)
   - Selenium (multiple languages)
   ```

2. **Implement Proper File Organization**

   ```bash
   tests/reference-screenshots/
   ├── home-page-reference.png
   ├── home-page-current.png
   └── home-page-diff.png
   ```

3. **Use Robust Comparison Logic**
   ```rust
   // Pixel-perfect diffing with configurable thresholds
   pub fn compare_images(img1: &DynamicImage, img2: &DynamicImage, threshold: f32) -> bool {
       // Implement proper pixel-by-pixel comparison
       // Save diff images for inspection
       // Use appropriate thresholds (0.01 = 1%)
   }
   ```

### Common Mistakes to Avoid

**❌ Don't:**

- Start with html2canvas or canvas-based solutions
- Try to capture screenshots from within browser JS
- Use approaches that require user interaction
- Implement complex workarounds for browser limitations

**✅ Do:**

- Use protocol-driven tools from the start
- Implement proper file organization and naming
- Use pixel-perfect comparison with configurable thresholds
- Save artifacts (current, diff) for inspection
- Make tests CI/CD friendly

### Implementation Checklist

- [ ] Use protocol-driven screenshot capture
- [ ] Implement proper file organization
- [ ] Add pixel-perfect comparison logic
- [ ] Save artifacts for failed tests
- [ ] Make tests headless and CI-friendly
- [ ] Add configurable thresholds
- [ ] Document the approach

### Examples from Today's Session

**❌ Started with html2canvas:**

```javascript
// Wasted time on inadequate approach
const canvas = await html2canvas(document.body);
```

**✅ Evolved to protocol-driven:**

```rust
// Correct approach from the start
tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)
```

## Impact

- Prevents wasted time on inadequate solutions
- Ensures production-ready visual regression from start
- Provides consistent, reliable results across environments
- Enables proper CI/CD integration
