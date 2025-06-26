# Critical Rules: Multi-Framework Testing & Test Organization

## Test Framework Separation

- **Sapper/Svelte tests run from project root:** `npm test`, Playwright tests in `tests/`
- **Yew tests run from `yew/` directory:** `cargo test`, Playwright tests in `yew/tests/`
- **NEVER run tests from the wrong directory** - this will cause framework-specific errors
- **Each framework has its own test configuration and dependencies**

## Visual Regression Testing

- **Sapper visual tests:** `tests/reference-capture/` (project root)
- **Yew visual tests:** `yew/tests/reference-capture/` (Yew subdirectory)
- **Reference screenshots are framework-specific** - never share between frameworks
- **Test snapshots and reports stay within their respective framework directories**

## Test Data Management

- **Sapper test fixtures:** `tests/fixtures/` or `cypress/fixtures/` (project root)
- **Yew test fixtures:** `yew/tests/fixtures/` (Yew subdirectory)
- **Never reference test data across framework boundaries**
- **Each framework's tests must be self-contained with their own data**

## Test Execution Context

- **Before running any tests, verify you're in the correct directory for that framework**
- **Use framework-specific test commands:**
  - Sapper: `npm test`, `npx playwright test`
  - Yew: `cargo test`, `npx playwright test` (from yew/ directory)
- **Test output and reports are framework-specific**

## Test Configuration Files

- **Sapper Playwright config:** `playwright.config.js` (project root)
- **Yew Playwright config:** `yew/playwright.config.js` (Yew subdirectory)
- **Each framework has its own test setup and teardown**
- **Never modify test configs for one framework when working on the other**

## Test Artifact Management

- **Sapper test artifacts:** `playwright-report/`, `test-results/` (project root)
- **Yew test artifacts:** `yew/playwright-report/`, `yew/test-results/` (Yew subdirectory)
- **Test snapshots and screenshots are framework-specific**
- **Always clean up test artifacts within the correct framework directory**

## Cross-Framework Testing Considerations

- **If testing integration between frameworks, create separate integration test suites**
- **Document any shared test utilities or common test patterns**
- **Ensure test isolation - tests for one framework shouldn't affect the other**
- **Use different ports and configurations to prevent test conflicts**

## Test Environment Setup

- **Each framework may require different test environment setup**
- **Document framework-specific test dependencies and setup requirements**
- **Ensure test runners and browsers are compatible with each framework**
- **Use framework-specific test utilities and helpers**
