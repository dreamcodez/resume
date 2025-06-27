const { test, expect } = require("@playwright/test");

/**
 * Test to verify that the interactive puzzle image loads correctly.
 * This test proves that the static asset copying in Trunk.toml is working
 * and the image is accessible at /static/sophisticated-macman.jpg
 *
 * Run with: npx playwright test tests/visual-puzzle-image.spec.js --config=playwright.config.js
 */
test("Puzzle image must not be broken", async ({ page }) => {
  // Navigate to the home page
  await page.goto("/");

  // Wait for the page to load
  await page.waitForLoadState("networkidle", { timeout: 5000 });

  // Wait for the puzzle image to be in the DOM - look for the actual alt text
  const imgLocator = page.locator('img[alt*="Sophisticated MacMan"]');
  await imgLocator.waitFor({ timeout: 3000 });

  // Get the src attribute
  const src = await imgLocator.getAttribute("src");
  console.log(`Image src: ${src}`);

  // Check that the image source is correct
  expect(src).toContain("sophisticated-macman.jpg");

  // Check that the image is visible
  await expect(imgLocator).toBeVisible({ timeout: 3000 });

  // Check that the image has loaded
  const isLoaded = await imgLocator.evaluate(
    (el) => el.complete && el.naturalHeight !== 0
  );

  expect(isLoaded).toBe(true);
});
