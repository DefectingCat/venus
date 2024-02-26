import { useBoolean, useMount } from 'ahooks';
import { Checkbox, Input, Select, Tooltip } from 'antd';
import SettingCard, {
  Setting,
  SettingLine,
} from 'components/common/setting-line';
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
    <>
      <Setting>
        <SettingCard title="Windows">
          <SettingLine title="Theme">
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
          </SettingLine>

          <SettingLine
            title={
              <Tooltip title="Restore last window position and size">
                Restore window
              </Tooltip>
            }
          >
            <Checkbox
              checked={rua.saveWindows}
              onChange={(e) =>
                updateConfig((config) => {
                  config.rua.saveWindows = e.target.checked;
                })
              }
            />
          </SettingLine>

          <SettingLine title="Auto start">
            <Checkbox
              checked={isAuto}
              onClick={async () => {
                isAuto ? await disable() : await enable();
                await updateAuto();
              }}
            />
          </SettingLine>

          <SettingLine title="Speed test url">
            <Input
              value={rua.settings.speedUrl}
              className="w-60"
              onChange={(e) => {
                updateConfig((config) => {
                  config.rua.settings.speedUrl = e.target.value;
                });
              }}
            />
          </SettingLine>
        </SettingCard>

        <SettingCard title="Subscription">
          <SettingLine title="Auto update">
            <Select
              className="w-24"
              value={rua.settings.updateSubs || 'off'}
              options={[
                { value: 'Off', label: 'Off' },
                { value: 'Startup', label: 'Startup' },
                { value: 'Time', label: 'Regularly' },
              ]}
              onChange={(value) => {
                updateConfig((config) => {
                  config.rua.settings.updateSubs = value;
                });
              }}
            />
          </SettingLine>
          {rua.settings.updateSubs === 'Time' && (
            <SettingLine title="Update subscription regularly (Unit: hour)">
              <Input
                defaultValue={0}
                value={rua.settings.updateTime}
                className="w-24"
                onChange={(e) => {
                  updateConfig((config) => {
                    config.rua.settings.updateTime = Number(e.target.value);
                  });
                }}
              />
            </SettingLine>
          )}
        </SettingCard>
      </Setting>

      <div>
        <ApplyBtn />
      </div>
    </>
  );
};

export default VenusSetting;
