# Waiting for Server Readiness in Automation Scripts

**Category:** workflow

## When to Apply

- When starting a dev/test server before running integration or visual tests
- When using Playwright, Cypress, or similar tools with a custom server startup script
- In CI pipelines or local scripts where server startup time may vary

## Why It Matters

- Prevents race conditions where tests start before the server is ready
- Avoids connection errors and test flakiness
- Ensures reliable and repeatable test runs

## Rule

- Always implement a readiness check after starting a dev/test server and before running tests.
- Use a polling loop to attempt a connection to the server port, with a reasonable timeout (e.g., 30s).
- Only proceed to run tests after the server responds to HTTP requests.

## Example

```js
// Wait for server readiness in global-setup.js
await new Promise((resolve, reject) => {
  const checkPort = () => {
    const testServer = http.createServer();
    testServer.listen(port, () => {
      testServer.close(() => resolve());
    });
    testServer.on("error", () => {
      setTimeout(checkPort, 100);
    });
  };
  setTimeout(checkPort, 500);
  setTimeout(() => reject(new Error("Timeout")), 30000);
});
```

## Error Scenario

- "Error: Timed out waiting for http://localhost:PORT" in Playwright or Cypress
- Tests fail intermittently due to server not being ready

## Command Pattern

```js
// After spawn(...)
waitForServerReady(port, timeoutMs);
runTests();
```
