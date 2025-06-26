const { test, expect } = require("@playwright/test");

test("capture reference home page", async ({ page }) => {
  await page.goto("https://dreamcodez.cc/");
  await page.waitForLoadState("networkidle");
  await page.screenshot({
    path: "tests/reference-screenshots/home-page-reference.png",
    fullPage: true,
  });
});
