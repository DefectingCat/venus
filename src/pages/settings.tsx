import { useBoolean, useMount } from 'ahooks';
import { Input, Select } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useTheme } from 'next-themes';
import dynamic from 'next/dynamic';
import { useMemo } from 'react';
import useStore from 'store';

const BasicSettings = dynamic(
  () => import('components/settings/basic-settings')
);

const Settings = () => {
  const core = useStore((s) => s.core);

  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);

  const { theme, setTheme } = useTheme();

  return (
    <MainLayout>
      <div className={clsx('mt-1 mb-4')}>
        <Title>Settings</Title>
      </div>

      <div className="flex">
        <div className={clsx('grid grid-cols-2', 'items-center gap-4')}>
          <div>Theme</div>
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

      <BasicSettings />
    </MainLayout>
  );
};

export default Settings;
