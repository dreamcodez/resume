const { test, expect } = require("@playwright/test");

// This test captures screenshots of the reference site
// Run this once to establish baseline for comparison
// Note: This should be run manually and not in CI

test.describe("Reference Site Capture", () => {
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
