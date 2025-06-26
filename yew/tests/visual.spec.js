const { test, expect } = require("@playwright/test");

// Visual tests for the Yew application
test.describe("Visual Regression Tests", () => {
  test.beforeEach(async ({ page }) => {
    // Wait for the page to load completely
    await page.waitForLoadState("networkidle");
    // Wait a bit more for any animations to complete
    await page.waitForTimeout(1000);
  });

  test("home page visual comparison", async ({ page }) => {
    await page.goto("/");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Wait for the puzzle image to load
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]', {
      timeout: 10000,
    });

    // Take a full page screenshot
    await expect(page).toHaveScreenshot("home-page.png", {
      fullPage: true,
      timeout: 10000,
    });
  });

  test("about page visual comparison", async ({ page }) => {
    await page.goto("/about");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Take a full page screenshot
    await expect(page).toHaveScreenshot("about-page.png", {
      fullPage: true,
      timeout: 10000,
    });
  });

  test("resume page visual comparison", async ({ page }) => {
    await page.goto("/resume");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Take a full page screenshot
    await expect(page).toHaveScreenshot("resume-page.png", {
      fullPage: true,
      timeout: 10000,
    });
  });

  test("blog page visual comparison", async ({ page }) => {
    await page.goto("/blog");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Take a full page screenshot
    await expect(page).toHaveScreenshot("blog-page.png", {
      fullPage: true,
      timeout: 10000,
    });
  });

  test("navigation visual comparison", async ({ page }) => {
    await page.goto("/");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Take a screenshot of just the navigation area
    const nav = page.locator("nav");
    await expect(nav).toHaveScreenshot("navigation.png", {
      timeout: 10000,
    });
  });

  test("mobile responsive visual comparison", async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto("/");

    // Wait for the app to be fully loaded
    await page.waitForLoadState("networkidle");

    // Wait for the puzzle image to load
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]', {
      timeout: 10000,
    });

    // Take a full page screenshot for mobile
    await expect(page).toHaveScreenshot("home-page-mobile.png", {
      fullPage: true,
      timeout: 10000,
    });
  });
});
