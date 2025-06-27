# Child Process Output Piping for Dev/Test Servers

**Category:** critical

## When to Apply

- When starting dev/test servers from automation scripts (e.g., Playwright global setup, CI jobs)
- When using Node.js `child_process.spawn` or similar APIs
- When server logs are needed for debugging or CI visibility

## Why It Matters

- Ensures all server output is visible in the main process logs
- Greatly improves debugging of startup failures and runtime errors
- Makes CI logs complete and actionable

## Rule

- Always use `stdio: "inherit"` when spawning dev/test servers from automation scripts.
- Avoid default or silent stdio settings unless output must be suppressed for a specific reason.

## Example

```js
// Node.js spawn for dev server
const child = spawn("trunk", ["serve", "--port", port], {
  stdio: "inherit",
  env: { ...process.env },
});
```

## Error Scenario

- CI logs missing server output, making it hard to debug failures
- Local runs where server errors are not visible in the terminal

## Command Pattern

```js
spawn("trunk", ["serve", "--port", port], {
  stdio: "inherit",
  env: { ...process.env },
});
```
