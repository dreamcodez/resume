const { test, expect } = require('@playwright/test');

test('capture reference navigation', async ({ page }) => {
  await page.goto('https://dreamcodez.cc/');
  await page.waitForLoadState('networkidle');
  const nav = page.locator('nav');
  await nav.screenshot({ 
    path: 'tests/reference-screenshots/navigation-reference.png'
  });
});
