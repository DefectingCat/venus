import type { AppProps } from 'next/app';
import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import 'styles/global.css';
import 'modern-normalize';
import { ThemeProvider } from 'next-themes';
import ThemeSwitcher from 'components/theme-switcher';
import { useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  useEffect(() => {
    const listeners: UnlistenFn[] = [];
    (async () => {
      listeners.push(
        await listen('rua://update-nodes', (e) => {
          console.log(e);
        })
      );

      emit('ready');

      try {
        const nodes = await invoke('get_rua_nodes');
        console.log(nodes);
      } catch (err) {
        message.error('Get nodes failed');
      }
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
