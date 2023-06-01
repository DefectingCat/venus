import { expect, test } from 'vitest';
import { render, screen } from '@testing-library/react';
import App from 'pages';

test('Index page', async () => {
  render(<App />);

  await screen.findByRole('heading');

  expect(
    screen.getByRole('heading', { level: 1, name: /Proxies/i })
  ).toBeDefined();
});
