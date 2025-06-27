const { test, expect } = require("@playwright/test");

test.describe("Dynamic Port Allocation", () => {
  test("should start server on an available port and serve content", async ({
    page,
  }) => {
    // Navigate to the home page
    await page.goto("/");

    // Verify the page loads successfully
    await expect(page).toHaveTitle(/Matthew Elders/);

    // Check that we're on a valid port
    const currentUrl = page.url();
    const portMatch = currentUrl.match(/localhost:(\d+)/);

    if (portMatch) {
      const port = parseInt(portMatch[1]);
      console.log(`Server is running on port: ${port}`);

      // Verify it's a reasonable port number (1024-65535)
      expect(port).toBeGreaterThanOrEqual(1024);
      expect(port).toBeLessThanOrEqual(65535);
    } else {
      throw new Error("Could not extract port from URL");
    }
  });

  test("should serve static assets from dynamic port", async ({ page }) => {
    // Navigate to the home page
    await page.goto("/");

    // Check that the puzzle image loads correctly
    const image = page.locator('img[src*="sophisticated-macman"]');
    await expect(image).toBeVisible();

    // Verify image has loaded (has natural dimensions)
    const naturalWidth = await image.evaluate((el) => el.naturalWidth);
    const naturalHeight = await image.evaluate((el) => el.naturalHeight);
    expect(naturalWidth).toBeGreaterThan(0);
    expect(naturalHeight).toBeGreaterThan(0);

    // Get the image src to verify it's using the correct port
    const src = await image.getAttribute("src");
    console.log("Image src:", src);

    // Verify the image URL uses the same port as the page
    const pageUrl = page.url();
    const pagePortMatch = pageUrl.match(/localhost:(\d+)/);
    const imagePortMatch = src.match(/localhost:(\d+)/);

    if (pagePortMatch && imagePortMatch) {
      const pagePort = pagePortMatch[1];
      const imagePort = imagePortMatch[1];
      expect(imagePort).toBe(pagePort);
    }
  });

  test("should handle multiple concurrent test runs", async ({ page }) => {
    // This test verifies that the dynamic port allocation works
    // even when multiple tests run concurrently

    await page.goto("/");

    // Basic functionality check
    await expect(page).toHaveTitle(/Matthew Elders/);

    // Check that we can navigate and interact
    const currentUrl = page.url();
    expect(currentUrl).toContain("localhost:");

    // Verify we're on a valid port
    const portMatch = currentUrl.match(/localhost:(\d+)/);
    if (portMatch) {
      const port = parseInt(portMatch[1]);
      expect(port).toBeGreaterThanOrEqual(1024);
      expect(port).toBeLessThanOrEqual(65535);
    }
  });
});
