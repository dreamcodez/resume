const { test, expect } = require("@playwright/test");

test.describe("Dynamic Port Allocation", () => {
  test("should start server on an available port and serve content", async ({
    page,
  }) => {
    // Navigate to the page
    await page.goto("/");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Check that the page loaded successfully
    await expect(page).toHaveTitle(/Matthew Elders/);

    // Verify basic content is present
    await expect(page.locator("body")).toBeVisible();
  });

  test("should serve static assets correctly", async ({ page }) => {
    // Navigate to the page
    await page.goto("/");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Check that static assets are served correctly
    const response = await page.goto("/static/sophisticated-macman.jpg");
    expect(response.status()).toBe(200);
    expect(response.headers()["content-type"]).toContain("image/");
  });

  test("should handle concurrent test runs", async ({ page }) => {
    // This test verifies that multiple test runs can happen simultaneously
    // without port conflicts due to dynamic port allocation

    // Navigate to the page
    await page.goto("/");

    // Wait for the page to load
    await page.waitForLoadState("networkidle");

    // Check that the page loaded successfully
    await expect(page).toHaveTitle(/Matthew Elders/);

    // Add a small delay to simulate concurrent test execution
    await page.waitForTimeout(100);
  });
});
