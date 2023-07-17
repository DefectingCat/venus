import { useBoolean, useMount } from 'ahooks';
import { App, Button, Checkbox, Select, Tooltip } from 'antd';
import clsx from 'clsx';
import useBackend from 'hooks/use-backend';
import { useTheme } from 'next-themes';
import { useEffect, useState } from 'react';
import useStore from 'store';
import { enable, isEnabled, disable } from 'tauri-plugin-autostart-api';

const VenusSetting = () => {
  const { message } = App.useApp();
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);

  const { theme, setTheme } = useTheme();

  const { writeConfig } = useBackend();
  const rua = useStore((s) => s.rua);
  const coreStatus = useStore((s) => s.rua.core_status);
  const updateConfig = useStore((s) => s.updateConfig);
  const handleApply = async () => {
    try {
      updateConfig((config) => {
        config.rua.core_status = 'Restarting';
      });
      writeConfig('rua');
    } catch (err) {
      message.error(err);
    }
  };

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
          checked={rua.save_windows}
          onChange={(e) =>
            updateConfig((config) => {
              config.rua.save_windows = e.target.checked;
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

        <div>
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
  );
};

export default VenusSetting;