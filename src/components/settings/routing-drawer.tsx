import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import {
  DrawerInput,
  DrawerMonaco,
  DrawerSelect,
} from 'components/common/drawer-item';
import { useCallback } from 'react';
import useStore from 'store';
import { Rule } from 'store/config-store';

const RoutingDrawer = ({
  drawerType,
  onClose,
  index,
}: {
  drawerType: string;
  onClose: () => void;
  // current edit rule's index
  index: number;
}) => {
  const updateConfig = useStore((s) => s.updateConfig);
  const [open, setOpen] = useBoolean(true);
  const outbounds = useStore((s) => s.core.outbounds);

  const updateField = useCallback(
    (field: keyof Rule, parse?: boolean) => (value) => {
      updateConfig((config) => {
        try {
          /** @ts-expect-error use the key to index rule's field */
          config.core.routing.rules[index][field] = parse
            ? JSON.parse(value)
            : value;
        } catch (err) {
          console.error(err);
        }
      });
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [index],
  );

  return (
    <Drawer
      title={`${drawerType} rules`}
      open={open}
      onClose={() => {
        setOpen.setFalse();
        setTimeout(() => {
          onClose();
        }, 300);
      }}
      keyboard={false}
      maskClosable={false}
    >
      <DrawerMonaco
        label="IP"
        language="json"
        onValidate={(marker) => {
          console.log(marker);
        }}
        onChange={updateField('ip', true)}
      />
      <DrawerMonaco
        label="Domain"
        language="json"
        onChange={updateField('domain', true)}
      />
      <DrawerSelect
        label="Outbounds"
        placeholder="Select a outbound"
        options={outbounds.map((out) => ({ label: out.tag, value: out.tag }))}
        onChange={updateField('network')}
      />
      <DrawerInput
        label="Port"
        onChange={(e) => updateField('port')(e.target.value.trimEnd())}
      />
      <DrawerSelect
        label="Network"
        placeholder="Select a network"
        options={[
          {
            label: 'Tcp',
            value: 'tcp',
          },
          {
            label: 'Udp',
            value: 'udp',
          },
          {
            label: 'Tcp and udp',
            value: 'tcp,udp',
          },
        ]}
        onChange={updateField('network')}
      />
      <DrawerMonaco
        label="Source"
        language="json"
        onChange={updateField('source', true)}
      />
      <DrawerMonaco
        label="Inbound tag"
        language="json"
        onChange={updateField('inboundTag', true)}
      />
      <DrawerSelect
        label="Protocol"
        placeholder="Select a protocol"
        options={[
          {
            label: 'Http',
            value: 'http',
          },
          {
            label: 'Tls',
            value: 'tls',
          },
          {
            label: 'Bittorrent',
            value: 'bittorrent',
          },
        ]}
        onChange={updateField('protocol')}
      />
      <DrawerMonaco
        label="Attrs"
        language="starlark"
        onChange={updateField('attrs')}
      />
      <DrawerInput
        label="Balance tag"
        onChange={(e) => updateField('balancerTag')(e.target.value.trimEnd())}
      />
    </Drawer>
  );
};

export default RoutingDrawer;
