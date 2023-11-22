import { Input, Select, Switch } from 'antd';
import clsx from 'clsx';
import { useMemo } from 'react';
import useStore from 'store';
import { Inbound, InboundSettings, Sniffing } from 'store/config-store';
import ApplyBtn from './apply-btn';

const BasicSettings = () => {
  const core = useStore((s) => s.core);
  const updateSocksInbound = useStore((s) => s.updateSocksInbound);
  const socksInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'socks'),
    [core?.inbounds],
  );

  const updateHttpInbound = useStore((s) => s.updateHttpInbound);
  const httpInbound = useMemo(
    () => core?.inbounds.find((i) => i.tag === 'http'),
    [core?.inbounds],
  );

  /**
   * Set inbound settings with specified key
   */
  const changeTarget =
    (
      value: string | number | boolean,
      target: (keyof Inbound | keyof InboundSettings | keyof Sniffing)[],
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
  const updateConfig = useStore((s) => s.updateConfig);

  return (
    <>
      <div className="flex">
        <div className={clsx('grid grid-cols-2', 'items-center gap-4')}>
          <div>Log Level</div>
          <Select
            value={core.log.loglevel}
            options={[
              { value: 'debug', label: 'Debug' },
              { value: 'info', label: 'Info' },
              { value: 'warning', label: 'Warning' },
              { value: 'error', label: 'Error' },
              { value: 'none', label: 'None' },
            ]}
            className="w-24"
            onChange={(value) => {
              updateConfig((config) => {
                config.core.log.loglevel = value;
              });
            }}
          />

          <div>Local socks port</div>
          <Input
            className="w-24"
            value={socksInbound?.port}
            onChange={(e) => {
              updateSocksInbound(
                changeTarget(Number(e.target.value.trimEnd()), ['port']),
              );
            }}
          />

          <div>Local http port</div>
          <Input
            className="w-24"
            value={httpInbound?.port}
            onChange={(e) => {
              updateHttpInbound(
                changeTarget(Number(e.target.value.trimEnd()), ['port']),
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
                  changeTarget(checked, ['sniffing', 'enabled']),
                );
                updateHttpInbound(
                  changeTarget(checked, ['sniffing', 'enabled']),
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
                  changeTarget(checked, ['sniffing', 'routeOnly']),
                );
                updateHttpInbound(
                  changeTarget(checked, ['sniffing', 'routeOnly']),
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
        <ApplyBtn />
      </div>
    </>
  );
};

export default BasicSettings;
