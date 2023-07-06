import type { AppProps } from 'next/app';
import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import 'styles/global.css';
import 'modern-normalize';
import { ThemeProvider } from 'next-themes';
import ThemeSwitcher from 'components/theme-switcher';
import { useEffect } from 'react';
import useBackend from 'hooks/use-backend';
import useStore from 'store';
import { CoreConfig, RConfig } from 'store/config-store';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  const updateRConfig = useStore((s) => s.updateRConfig);
  const updateCoreConfig = useStore((s) => s.updateCoreConfig);
  const updateLogging = useStore((s) => s.updateLogging);
  const { reloadConfig } = useBackend();

  useEffect(() => {
    const listeners: UnlistenFn[] = [];
    (async () => {
      listeners.push(
        await listen<RConfig>('rua://update-rua-config', (e) => {
          updateRConfig(e.payload);
        })
      );
      listeners.push(
        await listen<CoreConfig>('rua://update-core-config', (e) => {
          updateCoreConfig(e.payload);
        })
      );

      listeners.push(
        await listen<string>('rua://emit-log', (e) => {
          updateLogging((log) => {
            log.logs.push(e.payload);
          });
        })
      );

      emit('ready');

      reloadConfig('core');
      reloadConfig('rua');
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
