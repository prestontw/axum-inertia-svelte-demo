import { expect, test } from '@playwright/test';

test('update titles', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page
    .locator('#app div')
    .filter({ hasText: 'Hello, there John Smith (Mr' })
    .getByRole('button')
    .click();
  await page.getByRole('button', { name: 'Add Title' }).click();
  await page.locator('input:nth-child(3)').click();
  await page.locator('input:nth-child(3)').fill('Sr.');
  await page.getByRole('button', { name: 'Submit' }).click();
  await expect(page.getByRole('article')).toContainText('Sr.');
});
