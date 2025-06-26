# Visual Test Maintenance

## Overview

Visual tests require ongoing maintenance to remain reliable. Proper maintenance prevents false failures and ensures visual regression testing continues to provide value.

## Testing Rules

### 1. Screenshot Update Triggers

**When**: UI changes are made
**Why**: Prevents test failures due to intentional changes

**Update Required When:**

- [ ] Component styling changes
- [ ] Layout modifications
- [ ] Content updates that affect visual appearance
- [ ] CSS framework updates
- [ ] Browser-specific styling changes

**Update Not Required When:**

- [ ] Backend logic changes (no UI impact)
- [ ] Test-only code changes
- [ ] Documentation updates
- [ ] Performance optimizations (no visual impact)

### 2. Screenshot Update Workflow

**When**: UI changes require new reference screenshots
**Why**: Ensures consistent and reliable updates

```bash
# ✅ Correct update workflow
# 1. Make UI changes
# 2. Start dev server
cd yew && npm run dev

# 3. Update screenshots
npm run test:update-screenshots

# 4. Verify changes
npm run test:playwright

# 5. Review diff (if any)
# 6. Commit changes together
git add tests/reference-screenshots/
git add src/  # UI changes
git commit -m "Update UI and reference screenshots"
```

### 3. Screenshot Review Process

**When**: Screenshots are updated
**Why**: Ensures changes are intentional and correct

**Review Checklist:**

- [ ] Screenshots match expected UI changes
- [ ] No unintended visual regressions
- [ ] Screenshots are clear and representative
- [ ] File sizes are reasonable
- [ ] Naming follows conventions

### 4. Test Configuration Maintenance

**When**: Setting up or modifying visual tests
**Why**: Ensures consistent test behavior

**Configuration Elements:**

```javascript
// ✅ Correct configuration
const config = {
  use: {
    // Consistent viewport sizes
    viewport: { width: 1280, height: 720 },

    // Reliable screenshot settings
    screenshot: "only-on-failure",

    // Appropriate timeouts
    actionTimeout: 3000,
    navigationTimeout: 5000,
  },

  // Global test timeout
  timeout: 10000,
};
```

### 5. Cross-Browser Visual Testing

**When**: Ensuring visual consistency across browsers
**Why**: Catches browser-specific rendering issues

**Browser Coverage:**

```javascript
// Test against multiple browsers
const browsers = ["chromium", "firefox", "webkit"];

browsers.forEach((browser) => {
  test(`Visual test on ${browser}`, async ({ page }) => {
    // Test implementation
  });
});
```

## Common Issues and Solutions

### Screenshot Mismatch Due to Timing

```javascript
// ❌ Problem: Screenshot taken before page is fully loaded
await page.goto("/");
await page.screenshot({ path: "screenshot.png" });

// ✅ Solution: Wait for specific elements
await page.goto("/");
await page.waitForSelector(".main-content");
await page.screenshot({ path: "screenshot.png" });
```

### Flaky Visual Tests

```javascript
// ❌ Problem: Tests fail intermittently
await page.screenshot({ path: "screenshot.png" });

// ✅ Solution: Stabilize before screenshot
await page.waitForLoadState("networkidle");
await page.screenshot({ path: "screenshot.png" });
```

### Browser-Specific Differences

```javascript
// ❌ Problem: Tests fail on different browsers
await page.screenshot({ path: "screenshot.png" });

// ✅ Solution: Use browser-specific references
const browserName = process.env.BROWSER || "chromium";
await page.screenshot({
  path: `screenshot-${browserName}.png`,
});
```

## Maintenance Schedule

### Daily

- [ ] Run visual tests as part of development workflow
- [ ] Address any immediate failures

### Weekly

- [ ] Review screenshot update needs
- [ ] Clean up outdated reference screenshots
- [ ] Verify test configuration is optimal

### Monthly

- [ ] Review visual test coverage
- [ ] Update test configurations if needed
- [ ] Document any new visual testing patterns

## Error Recovery

### Screenshot Not Found

```bash
# Error: ENOENT: no such file or directory
# Solution: Regenerate missing screenshots
npm run test:update-screenshots
```

### Screenshot Mismatch

```bash
# Error: Screenshot comparison failed
# Solution: Review and update if intentional
# 1. Check if change is expected
# 2. Update if intentional: npm run test:update-screenshots
# 3. Fix if unintended: revert UI changes
```

### Test Configuration Issues

```bash
# Error: Tests fail due to configuration
# Solution: Review and update configuration
# 1. Check timeout settings
# 2. Verify viewport configuration
# 3. Update if needed
```

## Success Indicators

- Visual tests pass consistently
- Screenshots are updated with UI changes
- No false positive failures
- Clear documentation of visual changes
- Efficient screenshot update process

## Related Rules

- [Reference Screenshot Management](../critical/reference-screenshot-management.md)
- [Playwright Timeout Management](../critical/playwright-timeout-management.md)
- [Test Script Organization](../critical/test-script-organization.md)
