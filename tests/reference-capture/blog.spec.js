const { test, expect } = require("@playwright/test");

test("capture reference blog page", async ({ page }) => {
  await page.goto("https://dreamcodez.cc/blog");
  await page.waitForLoadState("networkidle");
  await page.screenshot({
    path: "tests/reference-screenshots/blog-page-reference.png",
    fullPage: true,
  });
});
