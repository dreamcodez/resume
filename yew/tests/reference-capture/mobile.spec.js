const { test, expect } = require('@playwright/test');

test('capture reference mobile view', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 667 });
  await page.goto('https://dreamcodez.cc/');
  await page.waitForLoadState('networkidle');
  await page.screenshot({ 
    path: 'tests/reference-screenshots/home-page-mobile-reference.png',
    fullPage: true 
  });
});
