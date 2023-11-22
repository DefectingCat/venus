import { useBoolean, useMount } from 'ahooks';
import { Checkbox, Input, Select } from 'antd';
import clsx from 'clsx';
import { useTheme } from 'next-themes';
import { useEffect, useState } from 'react';
import useStore from 'store';
import { disable, enable, isEnabled } from 'tauri-plugin-autostart-api';
import ApplyBtn from './apply-btn';

const VenusSetting = () => {
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);

  const { theme, setTheme } = useTheme();

  const rua = useStore((s) => s.rua);
  const updateConfig = useStore((s) => s.updateConfig);

  // auto start
  const [isAuto, setIsAuto] = useState(false);
  const updateAuto = async () => {
    const target = await isEnabled();
    setIsAuto(target);
  };
  useEffect(() => {
    updateAuto();
  }, []);

  return (
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
          checked={rua.saveWindows}
          onChange={(e) =>
            updateConfig((config) => {
              config.rua.saveWindows = e.target.checked;
            })
          }
        />

        <div>Auto start</div>
        <Checkbox
          checked={isAuto}
          onClick={async () => {
            isAuto ? await disable() : await enable();
            await updateAuto();
          }}
        />

        <div>Speed test url</div>
        <Input
          value={rua.settings.speedUrl}
          className="w-60"
          onChange={(e) => {
            updateConfig((config) => {
              config.rua.settings.speedUrl = e.target.value;
            });
          }}
        />

        <div>
          <ApplyBtn />
        </div>
      </div>
    </div>
  );
};

export default VenusSetting;
