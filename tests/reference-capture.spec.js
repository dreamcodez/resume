const { test, expect } = require("@playwright/test");

// ⚠️  CRITICAL WARNING: DO NOT ENABLE THESE TESTS UNLESS YOU ARE UPDATING REFERENCE SCREENSHOTS ⚠️
//
// 🚫 THESE TESTS ARE INTENTIONALLY SKIPPED TO PREVENT UNNECESSARY EXTERNAL API CALLS
//
// 📋 WHEN TO RE-ENABLE:
//   1. ONLY when the live site (dreamcodez.cc) has been updated with new visual changes
//   2. ONLY when you need to capture new baseline screenshots for visual regression testing
//   3. ONLY run manually, never in CI/CD pipelines
//
// 🔄 TO RE-ENABLE: Remove the .skip from test.describe below
// 🔄 TO DISABLE AGAIN: Add .skip back after capturing new screenshots
//
// 💡 REMEMBER: These tests make external HTTP requests to dreamcodez.cc and should only
//    be run when absolutely necessary for updating visual baselines.

test.describe.skip("Reference Site Capture", () => {
  test("capture reference home page", async ({ page }) => {
    // Navigate to the reference site
    await page.goto("https://dreamcodez.cc/");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Take a full page screenshot
    await page.screenshot({
      path: "tests/reference-screenshots/home-page-reference.png",
      fullPage: true,
    });
  });

  test("capture reference about page", async ({ page }) => {
    await page.goto("https://dreamcodez.cc/about");
    await page.waitForLoadState("networkidle");

    await page.screenshot({
      path: "tests/reference-screenshots/about-page-reference.png",
      fullPage: true,
    });
  });

  test("capture reference resume page", async ({ page }) => {
    await page.goto("https://dreamcodez.cc/resume");
    await page.waitForLoadState("networkidle");

    await page.screenshot({
      path: "tests/reference-screenshots/resume-page-reference.png",
      fullPage: true,
    });
  });

  test("capture reference blog page", async ({ page }) => {
    await page.goto("https://dreamcodez.cc/blog");
    await page.waitForLoadState("networkidle");

    await page.screenshot({
      path: "tests/reference-screenshots/blog-page-reference.png",
      fullPage: true,
    });
  });

  test("capture reference navigation", async ({ page }) => {
    await page.goto("https://dreamcodez.cc/");
    await page.waitForLoadState("networkidle");

    const nav = page.locator("nav");
    await nav.screenshot({
      path: "tests/reference-screenshots/navigation-reference.png",
    });
  });

  test("capture reference mobile view", async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto("https://dreamcodez.cc/");
    await page.waitForLoadState("networkidle");

    await page.screenshot({
      path: "tests/reference-screenshots/home-page-mobile-reference.png",
      fullPage: true,
    });
  });
});
