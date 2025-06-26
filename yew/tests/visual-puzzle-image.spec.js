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
  await page.waitForLoadState("networkidle");

  // Wait for the puzzle image to be in the DOM - look for the actual alt text
  const img = await page.waitForSelector('img[alt*="Sophisticated MacMan"]', {
    timeout: 10000,
  });

  // Check if the image loaded successfully
  const isLoaded = await img.evaluate(
    (el) => el.complete && el.naturalHeight !== 0
  );

  // If image is not loaded, check if it's a path issue
  if (!isLoaded) {
    const src = await img.getAttribute("src");
    console.log(`Image src: ${src}`);

    // Check if the image path is correct
    expect(src).toContain("sophisticated-macman.jpg");

    // Wait a bit more for the image to load
    await page.waitForTimeout(2000);

    // Check again
    const isLoadedAfterWait = await img.evaluate(
      (el) => el.complete && el.naturalHeight !== 0
    );

    if (!isLoadedAfterWait) {
      // If still not loaded, check if the file exists by trying to fetch it
      const response = await page.goto(src);
      expect(response.status()).toBe(200);
    }
  }

  // Check that the image has a reasonable size
  const { width, height } = await img.boundingBox();
  expect(width).toBeGreaterThan(100);
  expect(height).toBeGreaterThan(100);

  // Check that the image source is correct
  const src = await img.getAttribute("src");
  expect(src).toContain("sophisticated-macman.jpg");
});
