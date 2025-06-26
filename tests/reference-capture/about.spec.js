const { test, expect } = require("@playwright/test");

test("capture reference about page", async ({ page }) => {
  await page.goto("https://dreamcodez.cc/about");
  await page.waitForLoadState("networkidle");
  await page.screenshot({
    path: "tests/reference-screenshots/about-page-reference.png",
    fullPage: true,
  });
});
