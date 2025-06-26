# Playwright Timeout Management

## Rule: Use Global Timeouts, Never Explicit Timeouts

### ⚠️ CRITICAL RULE: NEVER use explicit timeouts in individual test files

**Why this rule exists:**

- Prevents tests from hanging indefinitely
- Ensures consistent timeout behavior across all tests
- Makes timeout management centralized and maintainable
- Reduces test flakiness and improves reliability

### ✅ CORRECT: Use Global Timeouts

**Configuration in `playwright.config.js`:**

```javascript
module.exports = defineConfig({
  use: {
    // Global timeout settings
    actionTimeout: 3000, // 3 seconds for actions
    navigationTimeout: 5000, // 5 seconds for navigation
  },

  // Global test timeout
  timeout: 10000, // 10 seconds per test

  webServer: {
    timeout: 20000, // 20 seconds for server startup
  },
});
```

**Test files should NOT specify timeouts:**

```javascript
// ✅ CORRECT - No explicit timeouts
test("My test", async ({ page }) => {
  await page.goto("http://localhost:8080/");
  await page.waitForSelector('img[alt*="Interactive Puzzle"]');
  // Test logic...
});
```

### ❌ INCORRECT: Explicit Timeouts in Tests

**Never do this:**

```javascript
// ❌ WRONG - Explicit timeouts cause hanging
test("My test", async ({ page }) => {
  await page.goto("http://localhost:8080/", { timeout: 10000 });
  await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 5000,
  });
  // Test logic...
});
```

### Timeout Values for Local Development

**Recommended timeout values for local Yew development server:**

| Setting              | Value   | Reason                             |
| -------------------- | ------- | ---------------------------------- |
| `actionTimeout`      | 3000ms  | Quick actions should complete fast |
| `navigationTimeout`  | 5000ms  | Local server navigation is fast    |
| `timeout` (per test) | 10000ms | Individual tests should be quick   |
| `webServer.timeout`  | 20000ms | Yew dev server startup time        |

### Debugging Timeout Issues

**If tests are timing out:**

1. **Check server startup:**

   ```bash
   cd yew && trunk serve --port 8080
   ```

2. **Verify server is accessible:**

   ```bash
   curl http://localhost:8080/
   ```

3. **Check for port conflicts:**

   ```bash
   lsof -i :8080
   ```

4. **Run tests with headed mode for debugging:**
   ```bash
   npx playwright test --headed --timeout=30000
   ```

### Migration Guide

**To fix existing tests with explicit timeouts:**

1. **Remove all `{ timeout: X }` parameters**
2. **Remove all `waitForSelector(selector, { timeout: X })`**
3. **Remove all `page.goto(url, { timeout: X })`**
4. **Rely on global timeouts in `playwright.config.js`**

**Example migration:**

```javascript
// Before (❌ WRONG)
await page.goto("http://localhost:8080/", { timeout: 10000 });
await page.waitForSelector('img[alt*="Interactive Puzzle"]', { timeout: 5000 });

// After (✅ CORRECT)
await page.goto("http://localhost:8080/");
await page.waitForSelector('img[alt*="Interactive Puzzle"]');
```

### Exception Handling

**Only use explicit timeouts when:**

- Testing against external services (not local dev server)
- Testing slow network conditions (with explicit test purpose)
- Testing timeout behavior itself

**Document any exceptions:**

```javascript
// Exception: Testing external API with known slow response
await page.waitForSelector(".api-response", { timeout: 30000 });
```

### Related Rules

- [Multi-Framework Testing](./multi-framework-testing.md)
- [Test Data Management](./test-data-management.md)
- [Yew Component Testing](./yew-component-testing.md)

### Enforcement

**This rule is enforced by:**

- Code review process
- Linting rules (where possible)
- CI/CD pipeline checks
- Documentation in test files

**Violation consequences:**

- Test hanging and CI/CD failures
- Inconsistent test behavior
- Difficult debugging and maintenance
