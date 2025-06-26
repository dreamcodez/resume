# Playwright Test Improvements

## Current Status

- ✅ Rust unit tests are all passing (149 tests)
- ❌ Playwright integration tests are failing due to missing reference screenshots and configuration issues

## Issues Identified

### 1. Missing Reference Screenshots

The visual parity tests are failing because reference screenshots don't exist:

```
Error: Reference screenshot not found: /Users/anon/dev/resume/yew/reference-visuals/home-page-reference.png
```

**Solution**: Need to generate reference screenshots by running the tests in capture mode or manually creating them.

### 2. Directory Structure Mismatch

The tests are looking for reference screenshots in `yew/reference-visuals/` but they might be in the parent directory `reference-visuals/`.

**Solution**: Update test paths or move reference screenshots to the correct location.

### 3. WebServer Configuration

Successfully configured Playwright to automatically start the Yew development server:

```javascript
webServer: {
  command: 'trunk serve --port 8080',
  url: 'http://localhost:8080',
  reuseExistingServer: !process.env.CI,
  timeout: 120 * 1000,
  cwd: __dirname,
},
```

## Next Steps

### 1. Generate Reference Screenshots

```bash
# Run tests in capture mode to generate reference screenshots
npx playwright test --update-snapshots
```

### 2. Fix Directory Paths

Check if reference screenshots exist in parent directory and either:

- Move them to `yew/reference-visuals/`
- Update test paths to point to parent directory

### 3. Test Categories to Address

#### Visual Parity Tests

- `tests/visual-parity-home.spec.js` - Desktop and mobile visual comparison
- `tests/visual-puzzle-image.spec.js` - Puzzle image rendering

#### Functional Tests

- `tests/puzzle-functionality.spec.js` - Interactive puzzle behavior
- `tests/example.spec.js` - Basic functionality

### 4. Test Configuration Improvements

#### Add Test Categories

Consider organizing tests into categories:

```javascript
// In playwright.config.js
projects: [
  {
    name: "unit",
    testMatch: /.*\.unit\.spec\.js/,
  },
  {
    name: "integration",
    testMatch: /.*\.spec\.js/,
  },
  {
    name: "visual",
    testMatch: /.*visual.*\.spec\.js/,
  },
];
```

#### Add Test Utilities

Create helper functions for common test operations:

```javascript
// tests/utils/test-helpers.js
export async function waitForPuzzleToLoad(page) {
  await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 10000,
  });
}

export async function completePuzzle(page) {
  await page.locator('button:has-text("🏗️")').click();
  await page.locator('button:has-text("⚡")').click();
  await page.locator('button:has-text("🔧")').click();
  await page.locator('button:has-text("🧩")').click();
}
```

### 5. CI/CD Integration

#### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: "18"
      - uses: actions/setup-rust@v3
      - run: cd yew && npm install
      - run: cd yew && cargo test
      - run: cd yew && npx playwright install
      - run: cd yew && npx playwright test
```

## Current Test Files

### Working Tests

- All Rust unit tests (149 tests) ✅
- Component tests for badge, button, card, icon, progress, layout ✅
- Interactive puzzle state management tests ✅
- Blog data processing tests ✅

### Failing Tests

- Visual parity tests (missing reference screenshots)
- Puzzle functionality tests (depend on visual tests)
- Example tests (may have similar issues)

## Environment Setup

### Prerequisites

```bash
# Install Rust and Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Trunk
cargo install trunk

# Install Node.js dependencies
cd yew && npm install

# Install Playwright browsers
npx playwright install
```

### Development Commands

```bash
# Run Rust tests
cd yew && cargo test

# Run Playwright tests
cd yew && npx playwright test

# Run specific test file
cd yew && npx playwright test tests/puzzle-functionality.spec.js

# Update screenshots
cd yew && npx playwright test --update-snapshots

# Show test report
cd yew && npx playwright show-report
```

## Performance Considerations

### Test Parallelization

- Current config runs tests in parallel with 5 workers
- Consider reducing workers if tests are resource-intensive
- Monitor memory usage during visual tests

### Timeout Settings

- WebServer timeout: 120 seconds (sufficient for Trunk build)
- Test timeout: 30 seconds for visual tests
- Consider adjusting based on CI environment performance

## Future Improvements

### 1. Test Data Management

- Create fixtures for consistent test data
- Mock external dependencies
- Add test database setup/teardown if needed

### 2. Visual Testing Strategy

- Implement visual regression testing
- Add baseline screenshot management
- Consider using visual testing services (Percy, Chromatic)

### 3. Accessibility Testing

- Add axe-core integration for accessibility testing
- Test keyboard navigation
- Screen reader compatibility tests

### 4. Performance Testing

- Add Lighthouse CI integration
- Core Web Vitals testing
- Bundle size monitoring

## Troubleshooting

### Common Issues

1. **Trunk build fails**: Check Rust dependencies and Trunk.toml configuration
2. **Port conflicts**: Ensure port 8080 is available
3. **Reference screenshots missing**: Run with `--update-snapshots` flag
4. **Browser installation issues**: Run `npx playwright install`

### Debug Commands

```bash
# Debug Trunk build
cd yew && trunk build --verbose

# Debug Playwright with headed mode
cd yew && npx playwright test --headed

# Debug specific test
cd yew && npx playwright test --debug tests/puzzle-functionality.spec.js
```

## Success Metrics

- [ ] All 42 Playwright tests passing
- [ ] Visual regression tests working
- [ ] CI/CD pipeline green
- [ ] Test coverage > 80%
- [ ] Test execution time < 5 minutes

## Critical Rules (NEVER Violate)

1. **❌ NEVER Break Existing Functionality** - Both Svelte and Yew versions must work
2. **❌ NEVER Ignore Test Failures** - All tests must pass before committing
3. **❌ NEVER Use Arbitrary Styling** - Follow established design patterns
4. **❌ NEVER Skip Documentation** - All changes must be documented
5. **📝 ALWAYS Run Tests** - Both Rust and Playwright tests must pass
6. **🔒 ALWAYS Maintain Visual Parity** - Yew version should match Svelte version
7. **🎯 ALWAYS Follow Component Patterns** - Use established component architecture
8. **⏱️ ALWAYS Use Global Timeouts** - Never use explicit timeouts in individual tests to prevent hanging
