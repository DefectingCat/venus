import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean, useMount } from 'ahooks';
import {
  Button,
  Checkbox,
  Select,
  Tabs,
  TabsProps,
  Tooltip,
  message,
} from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useTheme } from 'next-themes';
import dynamic from 'next/dynamic';
import { useState } from 'react';
import useStore from 'store';

const BasicSettings = dynamic(
  () => import('components/settings/basic-settings')
);

const Settings = () => {
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);

  const { theme, setTheme } = useTheme();

  const rua = useStore((s) => s.rua);
  const coreStatus = useStore((s) => s.rua.core_status);
  const updateConfig = useStore((s) => s.updateConfig);
  const handleApply = async () => {
    try {
      updateConfig((config) => {
        config.rua.core_status = 'Restarting';
      });
      await invoke('update_config', { ruaConfig: rua });
      message.success('Update config success');
    } catch (err) {
      console.error(err);
    }
  };

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

          <div>Rember window size</div>
          <Checkbox
            checked={rua.save_windows}
            onChange={(e) =>
              updateConfig((config) => {
                config.rua.save_windows = e.target.checked;
              })
            }
          />

          <div className="mt-4">
            <Tooltip placement="top" title="Apply and restart core">
              <Button
                loading={coreStatus === 'Restarting'}
                disabled={coreStatus === 'Stopped'}
                onClick={handleApply}
              >
                Apply
              </Button>
            </Tooltip>
          </div>
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
