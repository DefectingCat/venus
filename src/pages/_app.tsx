import type { AppProps } from 'next/app';
import 'styles/global.css';
import { ThemeProvider } from 'next-themes';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <ThemeProvider
      attribute="class"
      storageKey="rua-theme"
      enableSystem
      themes={['light', 'dark']}
    >
      <Component {...pageProps} />
    </ThemeProvider>
  );
}
