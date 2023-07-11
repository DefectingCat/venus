import { UnlistenFn, emit, listen } from '@tauri-apps/api/event';
import { ContextID } from 'components/context-menu';
import ThemeSwitcher from 'components/theme-switcher';
import 'modern-normalize';
import { ThemeProvider } from 'next-themes';
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import useStore from 'store';
import { CoreConfig, RConfig } from 'store/config-store';
import 'styles/global.css';

// This default export is required in a new `pages/_app.js` file.
export default function MyApp({ Component, pageProps }: AppProps) {
  const updateRConfig = useStore((s) => s.updateRConfig);
  const updateCoreConfig = useStore((s) => s.updateCoreConfig);
  const updateLogging = useStore((s) => s.updateLogging);
  const toggleUI = useStore((s) => s.toggleUI);

  // Update configs
  useEffect(() => {
    const listeners: UnlistenFn[] = [];
    (async () => {
      listeners.push(
        await listen<RConfig>('rua://update-rua-config', (e) => {
          const rua = e.payload;
          toggleUI((ui) => {
            ui.loading.subCrad = rua.subscriptions.map((sub) => ({
              url: sub.url,
              loading: false,
            }));
          });
          updateRConfig(rua);
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
            if (log.logs.length > 1_000) {
              log.logs.shift();
            }
            log.logs.push(e.payload);
          });
        })
      );

      emit('ready');
    })();

    return () => {
      listeners.forEach((listener) => listener());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Custom context menu
  useEffect(() => {
    const contextHandler = (e: MouseEvent) => {
      e.stopPropagation();
      e.preventDefault();
      toggleUI((ui) => {
        ui.showMenu = 'global';
        ui.mousePos = {
          x: e.clientX,
          y: e.clientY,
        };
      });
    };
    document.addEventListener('contextmenu', contextHandler);

    const contextClose = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      const id = target.getAttribute('id');
      if (id === ContextID) return;
      toggleUI((ui) => {
        ui.showMenu = null;
      });
    };
    document.addEventListener('click', contextClose);
    return () => {
      document.removeEventListener('contextmenu', contextHandler);
      document.removeEventListener('click', contextClose);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <ThemeProvider attribute="class" storageKey="rua-theme" enableSystem>
      <ThemeSwitcher Component={Component} {...pageProps} />
    </ThemeProvider>
  );
}
