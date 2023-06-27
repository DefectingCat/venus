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

  /**
   * Set inbound settings with specified key
   */
  const changeTarget =
    (
      value: string | number | boolean,
      target: (keyof Inbound | keyof InboundSettings | keyof Sniffing)[]
    ) =>
    (inbound: Inbound) => {
      const len = target.length;
      target.reduce<Inbound | InboundSettings | Sniffing>((prev, key, i) => {
        if (i === len - 1) {
          prev[key] = value;
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
              updateSocksInbound(
                changeTarget(Number(e.target.value.trimEnd()), ['port'])
              );
            }}
          />

          <div>Local http port</div>
          <Input
            className="w-24"
            value={httpInbound?.port}
            onChange={(e) => {
              updateHttpInbound(
                changeTarget(Number(e.target.value.trimEnd()), ['port'])
              );
            }}
          />

          <div>Enable UDP</div>
          <div>
            <Switch
              checked={socksInbound?.settings?.udp}
              onChange={(checked) => {
                updateSocksInbound(changeTarget(checked, ['settings', 'udp']));
                updateHttpInbound(changeTarget(checked, ['settings', 'udp']));
              }}
            />
          </div>

          <div>Enable sniffing</div>
          <div>
            <Switch
              checked={socksInbound?.sniffing?.enabled}
              onChange={(checked) => {
                updateSocksInbound(
                  changeTarget(checked, ['sniffing', 'enabled'])
                );
                updateHttpInbound(
                  changeTarget(checked, ['sniffing', 'enabled'])
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
                  changeTarget(checked, ['sniffing', 'routeOnly'])
                );
                updateHttpInbound(
                  changeTarget(checked, ['sniffing', 'routeOnly'])
                );
              }}
            />
          </div>

          <div>Allow connections from LAN</div>
          <div>
            <Switch
              checked={socksInbound?.listen === '0.0.0.0'}
              onChange={(checked) => {
                if (checked) {
                  updateSocksInbound(changeTarget('0.0.0.0', ['listen']));
                  updateHttpInbound(changeTarget('0.0.0.0', ['listen']));
                } else {
                  updateSocksInbound(changeTarget('127.0.0.1', ['listen']));
                  updateHttpInbound(changeTarget('127.0.0.1', ['listen']));
                }
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
