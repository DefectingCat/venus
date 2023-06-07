import type { AppProps } from 'next/app';
import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import 'styles/global.css';
import 'modern-normalize';
import { ThemeProvider } from 'next-themes';
import ThemeSwitcher from 'components/theme-switcher';
import { useEffect } from 'react';
import useBackend from 'hooks/use-backend';
import useStore, { RConfig } from 'store';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  const updateRconfig = useStore((s) => s.updateRconfig);
  const { reloadRconfig } = useBackend();

  useEffect(() => {
    const listeners: UnlistenFn[] = [];
    (async () => {
      listeners.push(
        await listen<RConfig>('rua://update-rua-config', (e) => {
          updateRconfig(e.payload);
        })
      );

      emit('ready');

      reloadRconfig();
    })();

    return () => {
      listeners.forEach((listener) => listener());
    };
  }, []);

  return (
    <ThemeProvider attribute="class" storageKey="rua-theme" enableSystem>
      <ThemeSwitcher Component={Component} {...pageProps} />
    </ThemeProvider>
  );
}
