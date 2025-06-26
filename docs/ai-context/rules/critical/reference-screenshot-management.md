# Reference Screenshot Management

## Overview

Visual tests rely on reference screenshots for comparison. Proper management prevents test failures and ensures consistent visual regression testing across different environments.

## Critical Rules

### 1. Centralized Reference Screenshot Location

**When**: Setting up or updating visual tests
**Why**: Prevents path confusion and ensures consistency

**Standard Location:**

```
tests/reference-screenshots/
├── about-page-reference.png
├── blog-page-reference.png
├── home-page-mobile-reference.png
├── home-page-reference.png
├── navigation-reference.png
└── resume-page-reference.png
```

### 2. Path Configuration in Playwright Tests

**When**: Writing or updating visual tests
**Why**: Ensures tests can find reference screenshots regardless of execution context

```javascript
// ✅ Correct - Use relative paths from test file location
const referencePath = "../reference-screenshots/home-page-reference.png";

// ❌ Avoid - Hardcoded absolute paths
const referencePath =
  "/Users/username/project/tests/reference-screenshots/home-page-reference.png";
```

### 3. Screenshot Update Workflow

**When**: UI changes require new reference screenshots
**Why**: Prevents test failures due to intentional UI changes

```bash
# ✅ Correct workflow
# 1. Make UI changes
# 2. Start dev server: cd yew && npm run dev
# 3. Update screenshots: npm run test:update-screenshots
# 4. Verify changes: npm run test:playwright
# 5. Commit new screenshots with UI changes
```

### 4. Cross-Directory Path Resolution

**When**: Tests run from different directories
**Why**: Ensures tests work regardless of execution context

**Path Resolution Strategy:**

```javascript
// In yew/tests/visual.spec.js
const referencePath = "../tests/reference-screenshots/home-page-reference.png";

// In tests/visual.spec.js (root level)
const referencePath = "./reference-screenshots/home-page-reference.png";
```

### 5. Screenshot Naming Conventions

**When**: Creating new reference screenshots
**Why**: Ensures clear identification and prevents conflicts

**Naming Pattern:**

```
{page-name}-{context}-reference.png
```

**Examples:**

- `home-page-reference.png` - Desktop home page
- `home-page-mobile-reference.png` - Mobile home page
- `about-page-reference.png` - About page
- `blog-page-reference.png` - Blog page

## Common Pitfalls

### ❌ Don't Use Absolute Paths

```javascript
// ❌ Wrong - Platform-specific and non-portable
const referencePath =
  "/Users/username/project/tests/reference-screenshots/home-page-reference.png";

// ✅ Correct - Relative and portable
const referencePath = "../reference-screenshots/home-page-reference.png";
```

### ❌ Don't Ignore Screenshot Updates

```bash
# ❌ Wrong - UI changes without updating references
# 1. Change UI
# 2. Run tests (will fail)
# 3. Ignore failures

# ✅ Correct - Update references with UI changes
# 1. Change UI
# 2. Update screenshots: npm run test:update-screenshots
# 3. Verify tests pass
```

### ❌ Don't Mix Screenshot Locations

```bash
# ❌ Wrong - Screenshots in multiple locations
tests/reference-screenshots/
yew/tests/reference-screenshots/
tests/visual-parity-home.spec.js-snapshots/

# ✅ Correct - Single centralized location
tests/reference-screenshots/
```

## Error Recovery

### Screenshot Not Found

```bash
# Error: ENOENT: no such file or directory
# Solution: Check path and update if needed
npm run test:update-screenshots
```

### Screenshot Mismatch

```bash
# Error: Screenshot comparison failed
# Solution: Review changes and update if intentional
npm run test:update-screenshots
```

### Path Resolution Issues

```bash
# Error: Cannot find reference screenshot
# Solution: Verify relative path from test file
# Check: ls -la tests/reference-screenshots/
```

## Success Indicators

- All visual tests pass consistently
- Reference screenshots are in expected locations
- Screenshot updates happen with UI changes
- No hardcoded absolute paths in test files
- Clear naming conventions followed

## Related Rules

- [Test Script Organization](./test-script-organization.md)
- [Playwright Timeout Management](./playwright-timeout-management.md)
- [Multi-Framework Testing](./multi-framework-testing.md)
