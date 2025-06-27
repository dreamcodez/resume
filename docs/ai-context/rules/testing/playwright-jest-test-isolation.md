# Playwright and Jest Test Isolation

**Category:** testing

## When to Apply

- When using both Playwright and Jest in the same project
- When test files for both frameworks are in the same directory
- When encountering errors due to test runner cross-pickup

## Why It Matters

- Prevents test runner errors and confusion
- Ensures each framework only runs its intended tests
- Improves reliability and clarity of test results

## Rule

- Use `.spec.js`/`.spec.ts` for Playwright/integration/visual tests
- Use `.test.js`/`.test.ts` for Jest/unit tests
- Configure Playwright to ignore `.test.js` files via `testIgnore`
- Configure Jest to ignore `.spec.js` files via `testPathIgnorePatterns`

## Example

```js
// playwright.config.js
module.exports = {
  testIgnore: ["**/*.test.js", "**/*.test.ts"],
  ...
};
```

```json
// jest config
{
  "testPathIgnorePatterns": ["<rootDir>/tests/.*.spec.js$"]
}
```

## Error Scenario

- Playwright: `ReferenceError: describe is not defined` (from Jest test file)
- Jest: `Playwright Test needs to be invoked via 'npx playwright test'` (from Playwright spec)

## Command Pattern

```sh
npx playwright test
npx jest
```
