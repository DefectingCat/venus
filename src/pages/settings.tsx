import { useBoolean, useMount } from 'ahooks';
import { Select, Tabs, TabsProps } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useTheme } from 'next-themes';
import dynamic from 'next/dynamic';
import { useState } from 'react';

const BasicSettings = dynamic(
  () => import('components/settings/basic-settings')
);

const Settings = () => {
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);

  const { theme, setTheme } = useTheme();

  const tabItems: TabsProps['items'] = [
    {
      key: '1',
      label: 'Basic Setting',
    },
    {
      key: '2',
      label: 'Core Basic',
    },
  ];
  const [current, setCurrent] = useState('1');

  const children = {
    1: (
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
    ),
    2: <BasicSettings />,
  };

  return (
    <MainLayout>
      <div className={clsx('mt-1 mb-4')}>
        <Title>Settings</Title>
      </div>

      <Tabs
        accessKey={current}
        items={tabItems}
        onChange={(key) => setCurrent(key)}
      />
      {children[current]}
    </MainLayout>
  );
};

export default Settings;
