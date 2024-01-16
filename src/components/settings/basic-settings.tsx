import { Input, Select, Switch } from 'antd';
import SettingCard, {
  Setting,
  SettingLine,
} from 'components/common/setting-line';
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
      <Setting>
        <SettingCard title="Common">
          <SettingLine title="Log level">
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
          </SettingLine>

          <SettingLine title="Local socks port">
            <Input
              className="w-24"
              value={socksInbound?.port}
              onChange={(e) => {
                updateSocksInbound(
                  changeTarget(Number(e.target.value.trimEnd()), ['port']),
                );
              }}
            />
          </SettingLine>

          <SettingLine title="Local http port">
            <Input
              className="w-24"
              value={httpInbound?.port}
              onChange={(e) => {
                updateHttpInbound(
                  changeTarget(Number(e.target.value.trimEnd()), ['port']),
                );
              }}
            />
          </SettingLine>
        </SettingCard>

        <SettingCard title="Features">
          <SettingLine title="Enable UDP">
            <div>
              <Switch
                checked={socksInbound?.settings?.udp}
                onChange={(checked) => {
                  updateSocksInbound(
                    changeTarget(checked, ['settings', 'udp']),
                  );
                  updateHttpInbound(changeTarget(checked, ['settings', 'udp']));
                }}
              />
            </div>
          </SettingLine>

          <SettingLine title="Enable sniffing">
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
          </SettingLine>

          <SettingLine title="RouteOnly">
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
          </SettingLine>

          <SettingLine title="Allow LAN">
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
          </SettingLine>
        </SettingCard>
      </Setting>

      <div className="mt-4">
        <ApplyBtn />
      </div>
    </>
  );
};

export default BasicSettings;
