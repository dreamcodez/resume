const { test, expect } = require("@playwright/test");

// ⚠️  CRITICAL WARNING: DO NOT ENABLE THIS TEST UNLESS YOU ARE UPDATING REFERENCE SCREENSHOTS ⚠️
//
// 🚫 THIS TEST IS INTENTIONALLY SKIPPED TO PREVENT UNNECESSARY EXTERNAL API CALLS
//
// 📋 WHEN TO RE-ENABLE:
//   1. ONLY when the live site (dreamcodez.cc) has been updated with new visual changes
//   2. ONLY when you need to capture new baseline screenshots for visual regression testing
//   3. ONLY run manually, never in CI/CD pipelines
//
// 🔄 TO RE-ENABLE: Remove the .skip from test below
// 🔄 TO DISABLE AGAIN: Add .skip back after capturing new screenshots
//
// 💡 REMEMBER: This test makes external HTTP requests to dreamcodez.cc and should only
//    be run when absolutely necessary for updating visual baselines.

test.skip("capture reference home page", async ({ page }) => {
  await page.goto("https://dreamcodez.cc/");
  await page.waitForLoadState("networkidle");
  await page.screenshot({
    path: "tests/reference-screenshots/home-page-reference.png",
    fullPage: true,
  });
});
