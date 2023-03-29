import { useBoolean, useMount } from 'ahooks';
import { Select } from 'antd';
import { useTheme } from 'next-themes';

const DarkMode = () => {
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);
  const { systemTheme, theme, setTheme } = useTheme();
  const currentTheme = theme === 'system' ? systemTheme : theme;

  return (
    <div className="flex items-center">
      <div className="mr-3">Theme</div>
      <Select
        value={mounted ? currentTheme : 'system'}
        options={[
          { value: 'system', label: 'System' },
          { value: 'light', label: 'Light' },
          { value: 'dark', label: 'Dark' },
        ]}
        onChange={(value) => setTheme(value)}
      />
    </div>
  );
};

export default DarkMode;
