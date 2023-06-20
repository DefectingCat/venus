import { useBoolean, useMount } from 'ahooks';
import { Input, Select } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useTheme } from 'next-themes';
import { useMemo } from 'react';
import useStore from 'store';

const Settings = () => {
  const core = useStore((s) => s.core);
  const updateConfig = useStore((s) => s.updateConfig);
  const socksInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'socks'),
    [core?.inbounds]
  );

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

      <div className="mt-1 mb-4">
        <Title.h2>Core basic</Title.h2>
      </div>
      <div className="flex">
        <div className={clsx('grid grid-cols-2', 'items-center gap-4')}>
          <div>Local socks port: </div>
          <Input
            className="w-24"
            value={socksInbound?.port}
            onChange={(e) => {
              updateConfig((config) => {
                const socks = config.core.inbounds.find(
                  (i) => i.tag === 'socks'
                );
                if (!socks) throw new Error('Cannot find socks inbound');
                socks.port = Number(e.target.value.trimEnd());
              });
            }}
          />
        </div>
      </div>
    </MainLayout>
  );
};

export default Settings;
