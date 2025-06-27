// @ts-check
const { defineConfig, devices } = require("@playwright/test");

/**
 * Get the port from environment variable or use fallback
 */
function getPort() {
  return process.env.TRUNK_PORT || "8080";
}

/**
 * @see https://playwright.dev/docs/test-configuration
 */
module.exports = defineConfig({
  testDir: "./tests",
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: process.env.CI ? 1 : undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: [["html", { open: "never" }]],
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    /* Base URL to use in actions like `await page.goto('/')`. */
    baseURL: `http://localhost:${getPort()}`,

    /* Collect trace when retrying the failed test. See https://playwright.dev/docs/trace-viewer */
    trace: "on-first-retry",

    /* Take screenshot on failure */
    screenshot: "only-on-failure",

    /* Enable touch support for mobile testing */
    hasTouch: true,

    /* Aggressive timeouts to prevent hanging - nothing longer than 5 seconds */
    actionTimeout: 3000, // 3 seconds for actions
    navigationTimeout: 5000, // 5 seconds for navigation
    expect: {
      timeout: 3000, // 3 seconds for assertions
    },
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: "chromium",
      use: { ...devices["Desktop Chrome"] },
    },

    {
      name: "firefox",
      use: { ...devices["Desktop Firefox"] },
    },

    {
      name: "webkit",
      use: { ...devices["Desktop Safari"] },
    },

    /* Test against mobile viewports. */
    {
      name: "Mobile Chrome",
      use: { ...devices["Pixel 5"] },
    },
    {
      name: "Mobile Safari",
      use: { ...devices["iPhone 12"] },
    },

    /* Test against branded browsers. */
    // {
    //   name: 'Microsoft Edge',
    //   use: { ...devices['Desktop Edge'], channel: 'msedge' },
    // },
    // {
    //   name: 'Google Chrome',
    //   use: { ...devices['Desktop Chrome'], channel: 'chrome' },
    // },
  ],

  /* Run your local dev server before starting the tests */
  webServer: {
    command: "node trunk-serve-dynamic.js",
    url: `http://localhost:${getPort()}`,
    reuseExistingServer: !process.env.CI,
    timeout: 30000, // 30 seconds for server startup (only startup, not tests)
    cwd: __dirname,
  },

  /* Global timeout for all tests - prevents hanging */
  timeout: 10000, // 10 seconds per test
  expect: {
    timeout: 3000, // 3 seconds for assertions
  },
  /* Global setup timeout */
  globalSetup: undefined,
  /* Global teardown timeout */
  globalTeardown: undefined,
});
