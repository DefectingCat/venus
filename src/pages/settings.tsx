import { useBoolean, useMount } from 'ahooks';
import { Select } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useTheme } from 'next-themes';

const Settings = () => {
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);
  const { theme, setTheme } = useTheme();

  return (
    <MainLayout>
      <div className={clsx('mt-1 mb-4')}>
        <Title>Settings</Title>
      </div>

      <div>
        <div className="flex items-center">
          <div className="mr-3">Theme</div>
          <Select
            className="w-24"
            value={mounted ? theme : 'system'}
            options={[
              { value: 'system', label: 'System' },
              { value: 'light', label: 'Light' },
              { value: 'dark', label: 'Dark' },
            ]}
            onChange={(value) => setTheme(value)}
          />
        </div>
      </div>
    </MainLayout>
  );
};

export default Settings;
