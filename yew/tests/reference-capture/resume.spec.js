const { test, expect } = require('@playwright/test');

test('capture reference resume page', async ({ page }) => {
  await page.goto('https://dreamcodez.cc/resume');
  await page.waitForLoadState('networkidle');
  await page.screenshot({ 
    path: 'tests/reference-screenshots/resume-page-reference.png',
    fullPage: true 
  });
});
