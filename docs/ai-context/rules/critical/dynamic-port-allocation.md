# Dynamic Port Allocation for Dev Servers

**Category:** critical

## When to Apply

- When running integration or visual tests that start a dev server (e.g., Playwright, Cypress, etc.)
- In CI environments where multiple jobs may run concurrently
- When port conflicts are possible due to other local services

## Why It Matters

- Prevents test flakiness and CI failures due to port conflicts
- Enables parallel test execution and multi-framework workflows
- Ensures robust automation and developer experience

## Rule

- Always allocate a random available port for dev/test servers unless a fixed port is required by legacy systems.
- Pass the chosen port to the server via environment variable or command-line argument.
- Ensure the test runner (e.g., Playwright) reads the port dynamically from the environment or a setup script.

## Example

```js
// global-setup.js (Playwright)
const port = await findFreePort();
process.env.TRUNK_PORT = String(port);
spawn("trunk", ["serve", "--port", port], { env: { ...process.env } });
```

```js
// playwright.config.js
const port = process.env.TRUNK_PORT;
module.exports = { use: { baseURL: `http://localhost:${port}` }, ... };
```

## Error Scenario

- "Error: Timed out waiting for http://localhost:8080" — indicates a port conflict or static port usage.

## Command Pattern

```sh
node trunk-serve-dynamic.js $PORT
export TRUNK_PORT=$PORT
npx playwright test
```
