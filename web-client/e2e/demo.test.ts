import { expect, test } from '@playwright/test';

test('create new user and edit', async ({ page }) => {
  await page.goto('http://localhost:3000/');
  await page.getByRole('button', { name: 'Add User' }).click();
  await page.getByRole('textbox').click();
  await page.getByRole('textbox').fill('New Person');
  await page.getByRole('button', { name: 'Add Title' }).click();
  await page.getByRole('textbox').nth(1).click();
  await page.getByRole('textbox').nth(1).fill('Sr.');
  await page.getByRole('button', { name: 'Submit' }).click();
  await expect(page.getByRole('article')).toContainText('Hello, there New Person (Sr.)');

  await page
    .locator('#app div')
    .filter({ hasText: 'Hello, there New Person (Sr' })
    .getByRole('button')
    .click();
  await page.getByRole('textbox').nth(1).click();
  await page.getByRole('textbox').nth(1).fill('Jr.');
  await page.getByRole('button', { name: 'Add Title' }).click();
  await page.getByRole('textbox').nth(2).click();
  await page.getByRole('textbox').nth(2).fill('Esq.');
  await page.getByRole('button', { name: 'Submit' }).click();
  await expect(page.getByRole('article')).toContainText('Hello, there New Person (Jr. Esq.)');
});
