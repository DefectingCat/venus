import { Input, Switch } from 'antd';
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
    </>
  );
};

export default basicSettings;
