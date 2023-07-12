import { ConfigProvider, theme as antTheme } from 'antd';
import { useTheme } from 'next-themes';
import { AppProps } from 'next/app';
import { App } from 'antd';

const themeMap = {
  light: antTheme.defaultAlgorithm,
  dark: antTheme.darkAlgorithm,
};

const defaultTheme = {
  colorPrimary: '#a6a6a6',
  colorInfo: '#778dab',
  wireframe: false,
};

const ThemeSwitcher = ({ Component, pageProps }: AppProps) => {
  const { systemTheme, theme } = useTheme();
  const currentTheme = theme === 'system' ? systemTheme : theme;

  return (
    <ConfigProvider
      theme={{
        algorithm: themeMap[currentTheme],
        token: {
          ...defaultTheme,
        },
        components: {
          Tabs: {
            fontSize: 16,
          },
          Table: {
            borderRadius: 8,
          },
        },
      }}
    >
      <App>
        <Component {...pageProps} />
      </App>
    </ConfigProvider>
  );
};

export default ThemeSwitcher;
