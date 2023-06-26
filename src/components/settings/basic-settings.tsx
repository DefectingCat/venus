import { invoke } from '@tauri-apps/api/tauri';
import { Button, Input, Switch, Tooltip, message } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import { useMemo } from 'react';
import useStore from 'store';

const basicSettings = () => {
  const core = useStore((s) => s.core);
  const updateSocksInbound = useStore((s) => s.updateSocksInbound);
  const socksInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'socks'),
    [core?.inbounds]
  );

  // Apply settings
  const coreStatus = useStore((s) => s.rua.core_status);
  const updateConfig = useStore((s) => s.updateConfig);
  const handleApply = async () => {
    try {
      updateConfig((config) => {
        config.rua.core_status = 'Restarting';
      });
      await invoke('update_core', { coreConfig: core });
      message.success('Update config success');
    } catch (err) {
      console.error(err);
    }
  };

  return (
    <>
      <div className="mt-1 mb-4">
        <Title.h2>Core basic</Title.h2>
      </div>

      <div className="flex">
        <div className={clsx('grid grid-cols-2', 'items-center gap-4')}>
          <div>Local socks port</div>
          <Input
            className="w-24"
            value={socksInbound?.port}
            onChange={(e) => {
              updateSocksInbound((socks) => {
                socks.port = Number(e.target.value.trimEnd());
              });
            }}
          />

          <div>Enable UDP</div>
          <div>
            <Switch
              checked={socksInbound?.settings?.udp}
              onChange={(checked) => {
                updateSocksInbound((socks) => {
                  socks.settings.udp = checked;
                });
              }}
            />
          </div>

          <div>Enable sniffing</div>
          <div>
            <Switch
              checked={socksInbound?.sniffing?.enabled}
              onChange={(checked) => {
                updateSocksInbound((socks) => {
                  socks.sniffing.enabled = checked;
                });
              }}
            />
          </div>
        </div>
      </div>

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
    </>
  );
};

export default basicSettings;
