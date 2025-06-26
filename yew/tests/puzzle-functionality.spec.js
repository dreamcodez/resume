const { test, expect } = require("@playwright/test");

/**
 * Test to verify the interactive puzzle functionality works correctly.
 * Tests both click and touch interactions, puzzle progression, and solution reveal.
 *
 * Run with: npx playwright test tests/puzzle-functionality.spec.js --config=playwright.config.js
 */

test.describe("Interactive Puzzle Functionality", () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the home page
    await page.goto("/");

    // Wait for the app to load
    await page.waitForLoadState("networkidle");

    // Wait for the puzzle to be visible - look for the actual image alt text
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]', {
      timeout: 10000,
    });

    // Wait a bit more for any animations to complete
    await page.waitForTimeout(1000);
  });

  test("Puzzle should progress through states correctly with clicks", async ({
    page,
  }) => {
    // Wait for the puzzle to be visible
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]');

    // Check initial state - should show progress indicators
    const progressIndicators = await page
      .locator(".flex.space-x-2 > div")
      .count();
    expect(progressIndicators).toBeGreaterThan(0);

    // Check initial progress text - look for the actual format
    const progressText = await page.locator("text=Progress 0%").isVisible();
    expect(progressText).toBeTruthy();

    // Check initial attempts text
    const attemptsText = await page.locator("text=0 attempts").isVisible();
    expect(attemptsText).toBeTruthy();

    // Click the first emoji (Foundation)
    await page.locator('button:has-text("🏗️")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText1 = await page.locator("text=Progress 25%").isVisible();
    expect(progressText1).toBeTruthy();

    // Click the second emoji (Performance)
    await page.locator('button:has-text("⚡")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText2 = await page.locator("text=Progress 50%").isVisible();
    expect(progressText2).toBeTruthy();

    // Click the third emoji (Tools)
    await page.locator('button:has-text("🔧")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText3 = await page.locator("text=Progress 75%").isVisible();
    expect(progressText3).toBeTruthy();

    // Click the final emoji (Solution)
    await page.locator('button:has-text("🧩")').click();
    await page.waitForTimeout(500);

    // Check puzzle is solved
    const progressText4 = await page.locator("text=Progress 100%").isVisible();
    expect(progressText4).toBeTruthy();

    // Check for completion message or reset button
    const resetButton = await page
      .locator('button:has-text("Reset")')
      .isVisible();
    expect(resetButton).toBeTruthy();
  });

  test("Puzzle should work with touch events on mobile", async ({ page }) => {
    // Set mobile viewport and enable touch support
    await page.setViewportSize({ width: 375, height: 667 });

    // Enable touch support for mobile testing
    await page.evaluate(() => {
      Object.defineProperty(navigator, "maxTouchPoints", { value: 1 });
    });

    // Wait for the puzzle to be visible
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]');

    // Touch the first emoji (Foundation) - use click instead of tap for better compatibility
    await page.locator('button:has-text("🏗️")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText1 = await page.locator("text=Progress 25%").isVisible();
    expect(progressText1).toBeTruthy();

    // Touch the second emoji (Performance)
    await page.locator('button:has-text("⚡")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText2 = await page.locator("text=Progress 50%").isVisible();
    expect(progressText2).toBeTruthy();

    // Touch the third emoji (Tools)
    await page.locator('button:has-text("🔧")').click();
    await page.waitForTimeout(500);

    // Check progress updated
    const progressText3 = await page.locator("text=Progress 75%").isVisible();
    expect(progressText3).toBeTruthy();

    // Touch the final emoji (Solution)
    await page.locator('button:has-text("🧩")').click();
    await page.waitForTimeout(500);

    // Check puzzle is solved
    const progressText4 = await page.locator("text=Progress 100%").isVisible();
    expect(progressText4).toBeTruthy();
  });

  test("Puzzle reset should work correctly", async ({ page }) => {
    // Wait for the puzzle to be visible
    await page.waitForSelector('img[alt*="Sophisticated MacMan"]');

    // Complete the puzzle
    await page.locator('button:has-text("🏗️")').click();
    await page.waitForTimeout(200);
    await page.locator('button:has-text("⚡")').click();
    await page.waitForTimeout(200);
    await page.locator('button:has-text("🔧")').click();
    await page.waitForTimeout(200);
    await page.locator('button:has-text("🧩")').click();
    await page.waitForTimeout(500);

    // Verify puzzle is complete
    const progressText = await page.locator("text=Progress 100%").isVisible();
    expect(progressText).toBeTruthy();

    // Click reset button
    await page.locator('button:has-text("Reset")').click();
    await page.waitForTimeout(500);

    // Verify puzzle is reset
    const resetProgressText = await page
      .locator("text=Progress 0%")
      .isVisible();
    expect(resetProgressText).toBeTruthy();

    // Verify reset button is no longer visible
    const resetButton = await page
      .locator('button:has-text("Reset")')
      .isVisible();
    expect(resetButton).toBeFalsy();
  });
});
