import { invoke } from '@tauri-apps/api/tauri';
import { Button, Input, Switch, Tooltip, message } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import { useMemo } from 'react';
import useStore, { Inbound, InboundSettings, Sniffing } from 'store';

const basicSettings = () => {
  const core = useStore((s) => s.core);
  const updateSocksInbound = useStore((s) => s.updateSocksInbound);
  const socksInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'socks'),
    [core?.inbounds]
  );

  const updateHttpInbound = useStore((s) => s.updateHttpInbound);
  const httpInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'http'),
    [core?.inbounds]
  );

  // Change port
  const changePort = (value: string) => (inbound: Inbound) => {
    inbound.port = Number(value);
  };
  /**
   * Toggle inbound settings with specified key
   */
  const toggleTarget =
    (
      checked: boolean,
      target: (keyof Inbound | keyof InboundSettings | keyof Sniffing)[]
    ) =>
    (inbound: Inbound) => {
      const len = target.length;
      target.reduce<Inbound | InboundSettings | Sniffing>((prev, key, i) => {
        if (i === len - 1) {
          prev[key] = checked;
        }
        return prev[key];
      }, inbound);
    };

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
              updateSocksInbound(changePort(e.target.value.trimEnd()));
            }}
          />

          <div>Local http port</div>
          <Input
            className="w-24"
            value={httpInbound?.port}
            onChange={(e) => {
              updateHttpInbound(changePort(e.target.value.trimEnd()));
            }}
          />

          <div>Enable UDP</div>
          <div>
            <Switch
              checked={socksInbound?.settings?.udp}
              onChange={(checked) => {
                updateSocksInbound(toggleTarget(checked, ['settings', 'udp']));
                updateHttpInbound(toggleTarget(checked, ['settings', 'udp']));
              }}
            />
          </div>

          <div>Enable sniffing</div>
          <div>
            <Switch
              checked={socksInbound?.sniffing?.enabled}
              onChange={(checked) => {
                updateSocksInbound(
                  toggleTarget(checked, ['sniffing', 'enabled'])
                );
                updateHttpInbound(
                  toggleTarget(checked, ['sniffing', 'enabled'])
                );
              }}
            />
          </div>

          <div>RouteOnly</div>
          <div>
            <Switch
              checked={socksInbound?.sniffing?.routeOnly}
              onChange={(checked) => {
                updateSocksInbound(
                  toggleTarget(checked, ['sniffing', 'routeOnly'])
                );
                updateHttpInbound(
                  toggleTarget(checked, ['sniffing', 'routeOnly'])
                );
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
