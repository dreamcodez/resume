const { test, expect } = require("@playwright/test");

/**
 * Test to verify that the interactive puzzle image loads correctly.
 * This test proves that the static asset copying in Trunk.toml is working
 * and the image is accessible at /static/sophisticated-macman.jpg
 *
 * Run with: npx playwright test tests/visual-puzzle-image.spec.js --config=playwright.config.js
 */
test("Puzzle image must not be broken", async ({ page }) => {
  // Use port 8080 where the Yew app is actually running
  await page.goto("http://localhost:8080/", { timeout: 10000 });

  // Wait for the puzzle image to be in the DOM
  const img = await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 10000,
  });

  // Check if the image loaded successfully
  const isLoaded = await img.evaluate(
    (el) => el.naturalWidth > 0 && el.naturalHeight > 0
  );

  expect(isLoaded).toBe(true);
});
