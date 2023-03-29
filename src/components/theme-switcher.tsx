import { ConfigProvider, theme as antTheme } from 'antd';
import { useTheme } from 'next-themes';
import { AppProps } from 'next/app';

const themeMap = {
  light: antTheme.defaultAlgorithm,
  dark: antTheme.darkAlgorithm,
};

const ThemeSwitcher = ({ Component, pageProps }: AppProps) => {
  const { systemTheme, theme } = useTheme();
  const currentTheme = theme === 'system' ? systemTheme : theme;

  return (
    <ConfigProvider
      theme={{
        algorithm: themeMap[currentTheme],
      }}
    >
      <Component {...pageProps} />
    </ConfigProvider>
  );
};

export default ThemeSwitcher;
