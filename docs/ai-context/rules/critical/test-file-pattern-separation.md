# Test File Pattern Separation for Multi-Framework Projects

**Category:** critical

## When to Apply

- When using multiple test frameworks (e.g., Jest for unit, Playwright for integration/visual)
- When test files for different frameworks are in the same directory
- When test runners are picking up each other's files, causing errors

## Why It Matters

- Prevents test runner conflicts and spurious errors (e.g., Jest running Playwright specs)
- Ensures each framework only runs its intended tests
- Streamlines CI and local test runs

## Rule

- Use distinct file patterns for each test framework:
  - `.test.js`/`.test.ts` for Jest/unit
  - `.spec.js`/`.spec.ts` for Playwright/integration/visual
- Configure each runner to ignore the other's patterns:
  - Jest: `testPathIgnorePatterns` for `.spec.js`
  - Playwright: `testIgnore` for `.test.js`

## Example

```json
// jest config
{
  "testPathIgnorePatterns": ["<rootDir>/tests/.*.spec.js$"]
}
```

```js
// playwright.config.js
module.exports = {
  testIgnore: ["**/*.test.js", "**/*.test.ts"],
  ...
};
```

## Error Scenario

- Jest: `Test suite failed to run ... Playwright Test needs to be invoked via 'npx playwright test' ...`
- Playwright: `ReferenceError: describe is not defined` (from Jest test file)

## Command Pattern

```sh
npx jest
npx playwright test
```
