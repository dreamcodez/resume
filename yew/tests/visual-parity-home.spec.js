const { test, expect } = require("@playwright/test");
const fs = require("fs");
const path = require("path");
const { PNG } = require("pngjs");

test.describe("Home Page Visual Parity", () => {
  test(
    "compare Yew home page with reference screenshot using pixelmatch",
    async ({ page }) => {
      // Import pixelmatch dynamically since it's an ES module
      const pixelmatch = (await import("pixelmatch")).default;

      // Set viewport to match reference screenshot dimensions
      await page.setViewportSize({ width: 1280, height: 756 });

      // Navigate to our local Yew app
      await page.goto("http://localhost:8080/");
      await page.waitForLoadState("networkidle");

      // Take a screenshot of our Yew home page
      const yewScreenshotPath = path.resolve("tests/yew-home-current.png");
      await page.screenshot({ path: yewScreenshotPath, fullPage: true });

      // Load the reference screenshot
      const referencePath = path.resolve(
        "../reference-visuals/home-page-reference.png"
      );
      if (!fs.existsSync(referencePath)) {
        throw new Error(`Reference screenshot not found: ${referencePath}`);
      }
      const referenceImg = PNG.sync.read(fs.readFileSync(referencePath));
      const yewImg = PNG.sync.read(fs.readFileSync(yewScreenshotPath));

      // Ensure dimensions match
      if (
        referenceImg.width !== yewImg.width ||
        referenceImg.height !== yewImg.height
      ) {
        throw new Error(
          `Image dimensions do not match. Reference: ${referenceImg.width}x${referenceImg.height}, Yew: ${yewImg.width}x${yewImg.height}`
        );
      }

      // Compare images
      const diff = new PNG({
        width: referenceImg.width,
        height: referenceImg.height,
      });
      const numDiffPixels = pixelmatch(
        referenceImg.data,
        yewImg.data,
        diff.data,
        referenceImg.width,
        referenceImg.height,
        { threshold: 0.1 }
      );
      const totalPixels = referenceImg.width * referenceImg.height;
      const percentDiff = (numDiffPixels / totalPixels) * 100;

      // Output diff image
      const diffPath = path.resolve("tests/yew-home-diff.png");
      fs.writeFileSync(diffPath, PNG.sync.write(diff));

      // Log and assert
      console.log(
        `Home page visual diff: ${percentDiff.toFixed(
          2
        )}% (${numDiffPixels} pixels)`
      );
      expect(percentDiff).toBeLessThanOrEqual(20);
    },
    { timeout: 30000 }
  );

  test("compare Yew home page mobile view with reference", async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Navigate to our local Yew app
    await page.goto("http://localhost:8080/");

    // Wait for the page to load completely
    await page.waitForLoadState("networkidle");

    // Take a screenshot of our Yew home page mobile view
    await page.screenshot({
      path: "tests/yew-home-mobile-current.png",
      fullPage: true,
    });

    // Compare with reference mobile screenshot
    await expect(page).toHaveScreenshot("home-page-mobile-reference.png", {
      threshold: 0.1,
      maxDiffPixels: 1000,
    });
  });
});
