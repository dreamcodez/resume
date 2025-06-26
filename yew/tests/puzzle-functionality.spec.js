const { test, expect } = require("@playwright/test");

/**
 * Test to verify the interactive puzzle functionality works correctly.
 * Tests both click and touch interactions, puzzle progression, and solution reveal.
 *
 * Run with: npx playwright test tests/puzzle-functionality.spec.js --config=playwright.config.js
 */

test("Puzzle should progress through states correctly with clicks", async ({
  page,
}) => {
  await page.goto("http://localhost:8080/", { timeout: 10000 });

  // Wait for the puzzle to be visible
  await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 10000,
  });

  // Check initial state - should show progress indicators
  const progressIndicators = await page.locator(".flex.gap-1 > div").count();
  expect(progressIndicators).toBe(4);

  // Check initial attempts counter
  const attemptsText = await page.locator("text=Attempts:").textContent();
  expect(attemptsText).toContain("Attempts: 0");

  // Click the first emoji (Foundation)
  await page.locator('button:has-text("🏗️")').click();

  // Verify first step is complete
  const firstIndicator = await page.locator(".flex.gap-1 > div").first();
  await expect(firstIndicator).toHaveClass(/bg-green-500/);

  // Click the second emoji (Performance)
  await page.locator('button:has-text("⚡")').click();

  // Verify second step is complete
  const secondIndicator = await page.locator(".flex.gap-1 > div").nth(1);
  await expect(secondIndicator).toHaveClass(/bg-green-500/);

  // Click the third emoji (Tools)
  await page.locator('button:has-text("🔧")').click();

  // Verify third step is complete
  const thirdIndicator = await page.locator(".flex.gap-1 > div").nth(2);
  await expect(thirdIndicator).toHaveClass(/bg-green-500/);

  // Click the final emoji (Solution)
  await page.locator('button:has-text("🧩")').click();

  // Verify solution is revealed
  await expect(page.locator("text=🎉 Puzzle Solved!")).toBeVisible();
  await expect(
    page.locator("text=Start with a solid foundation")
  ).toBeVisible();
  await expect(page.locator("text=Optimize for performance")).toBeVisible();
  await expect(
    page.locator("text=Use the right tools for the job")
  ).toBeVisible();
  await expect(
    page.locator("text=Piece together the perfect solution")
  ).toBeVisible();

  // Verify attempts counter increased
  const finalAttemptsText = await page.locator("text=Attempts:").textContent();
  expect(finalAttemptsText).toContain("Attempts: 4");
});

test("Puzzle should work with touch events on mobile", async ({ page }) => {
  // Set mobile viewport
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto("http://localhost:8080/", { timeout: 10000 });

  // Wait for the puzzle to be visible
  await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 10000,
  });

  // Touch the first emoji (Foundation)
  await page.locator('button:has-text("🏗️")').tap();

  // Verify first step is complete
  const firstIndicator = await page.locator(".flex.gap-1 > div").first();
  await expect(firstIndicator).toHaveClass(/bg-green-500/);

  // Touch the second emoji (Performance)
  await page.locator('button:has-text("⚡")').tap();

  // Verify second step is complete
  const secondIndicator = await page.locator(".flex.gap-1 > div").nth(1);
  await expect(secondIndicator).toHaveClass(/bg-green-500/);

  // Touch the third emoji (Tools)
  await page.locator('button:has-text("🔧")').tap();

  // Verify third step is complete
  const thirdIndicator = await page.locator(".flex.gap-1 > div").nth(2);
  await expect(thirdIndicator).toHaveClass(/bg-green-500/);

  // Touch the final emoji (Solution)
  await page.locator('button:has-text("🧩")').tap();

  // Verify solution is revealed
  await expect(page.locator("text=🎉 Puzzle Solved!")).toBeVisible();
});

test("Puzzle reset should work correctly", async ({ page }) => {
  await page.goto("http://localhost:8080/", { timeout: 10000 });

  // Wait for the puzzle to be visible
  await page.waitForSelector('img[alt*="Interactive Puzzle"]', {
    timeout: 10000,
  });

  // Complete the puzzle
  await page.locator('button:has-text("🏗️")').click();
  await page.locator('button:has-text("⚡")').click();
  await page.locator('button:has-text("🔧")').click();
  await page.locator('button:has-text("🧩")').click();

  // Verify solution is shown
  await expect(page.locator("text=🎉 Puzzle Solved!")).toBeVisible();

  // Click reset button
  await page.locator('button:has-text("Solve Again")').click();

  // Verify puzzle is reset
  await expect(page.locator("text=🎉 Puzzle Solved!")).not.toBeVisible();

  // Verify progress indicators are reset
  const firstIndicator = await page.locator(".flex.gap-1 > div").first();
  await expect(firstIndicator).toHaveClass(/bg-gray-300/);

  // Verify attempts counter is reset
  const attemptsText = await page.locator("text=Attempts:").textContent();
  expect(attemptsText).toContain("Attempts: 0");
});
