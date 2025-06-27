# Testing Rules: Visual Test Verification

## Cross-Browser Testing Requirements

- **ALWAYS run visual tests across all configured browsers** - don't assume single-browser success means universal compatibility
- **Test on both desktop and mobile browsers** - responsive design and touch interactions may differ
- **Verify static assets load correctly in all browsers** - image serving and caching can vary
- **Check for browser-specific rendering differences** - CSS and layout may not be identical

## Visual Test Execution Pattern

```bash
# Run specific visual test across all browsers
npx playwright test tests/visual-puzzle-image.spec.js

# Run all visual tests
npx playwright test tests/visual.spec.js

# Run with specific browser
npx playwright test --project=chromium tests/visual-puzzle-image.spec.js
```

## Asset Loading Verification

- **Verify image elements are visible and loaded** - check both `isVisible()` and natural dimensions
- **Test asset paths are correct** - confirm requests go to expected URLs
- **Check for 404 errors in browser dev tools** - indicates asset serving issues
- **Verify assets are served with correct MIME types** - images should have image/\* content-type

## Test Implementation Best Practices

```javascript
// CORRECT: Check both visibility and loading
test("Puzzle image must not be broken", async ({ page }) => {
  await page.goto("/");

  const image = page.locator('img[src*="sophisticated-macman"]');

  // Wait for image to be visible
  await expect(image).toBeVisible();

  // Verify image has loaded (has natural dimensions)
  await expect(image).toHaveJSProperty("naturalWidth", (width) => width > 0);
  await expect(image).toHaveJSProperty("naturalHeight", (height) => height > 0);

  // Log the actual src for debugging
  const src = await image.getAttribute("src");
  console.log("Image src:", src);
});
```

## Browser-Specific Considerations

### Chromium/Firefox/WebKit

- **Image loading behavior is generally consistent** - but check for timing differences
- **CSS rendering may vary slightly** - focus on functional correctness over pixel-perfect matching
- **Network request handling can differ** - verify assets are requested and served

### Mobile Browsers

- **Touch interactions may affect image loading** - test on actual mobile devices when possible
- **Viewport handling can differ** - ensure responsive images work correctly
- **Network conditions may be different** - test with slower connections

## Debugging Visual Test Failures

1. **Check browser console for errors** - JavaScript errors can prevent asset loading
2. **Verify network requests in dev tools** - confirm assets are being requested
3. **Check file permissions and paths** - ensure assets are accessible
4. **Test asset URLs directly in browser** - verify they return expected content
5. **Compare working vs failing browsers** - identify browser-specific issues

## Test Data Management

- **Use consistent test images** - avoid flaky external resources
- **Store reference images in project** - don't rely on external URLs
- **Version control test assets** - ensure tests are reproducible
- **Document asset requirements** - specify size, format, and content expectations

## CI/CD Integration

- **Run visual tests in CI pipeline** - catch issues before deployment
- **Use consistent browser versions** - avoid version-specific issues
- **Generate test reports** - document failures with screenshots
- **Set appropriate timeouts** - account for asset loading delays

## Performance Considerations

- **Optimize test images** - use appropriate sizes and formats
- **Minimize asset loading time** - avoid large files that slow tests
- **Use efficient selectors** - prefer data attributes over complex CSS selectors
- **Batch related tests** - reduce browser startup overhead

## Common Failure Patterns

### Asset Not Found (404)

- **Check file exists in source directory** - verify asset is in correct location
- **Verify copy configuration** - ensure assets are copied to dist/
- **Check path references** - confirm URLs match copied file locations

### Asset Not Visible

- **Check CSS display properties** - ensure element is not hidden
- **Verify viewport and positioning** - element may be off-screen
- **Check for overlay elements** - other elements may be covering the asset

### Asset Not Loaded

- **Check network connectivity** - ensure assets can be downloaded
- **Verify MIME types** - incorrect content-type can prevent loading
- **Check for CORS issues** - cross-origin requests may be blocked

## Test Maintenance

- **Update tests when assets change** - keep selectors and expectations current
- **Review test coverage regularly** - ensure all critical assets are tested
- **Document test dependencies** - specify required assets and configurations
- **Version test assets** - track changes that affect test behavior
