import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import {
  DrawerInput,
  DrawerMonaco,
  DrawerSelect,
} from 'components/common/drawer-item';
import useStore from 'store';

const RoutingDrawer = ({
  drawerType,
  onClose,
}: {
  drawerType: string;
  onClose: () => void;
}) => {
  const [open, setOpen] = useBoolean(true);
  const outbounds = useStore((s) => s.core.outbounds);
  const inbounds = useStore((s) => s.core.inbounds);

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
      <DrawerMonaco label="IP" language="json" />
      <DrawerMonaco label="Domain" language="json" />
      <DrawerSelect
        label="Outbounds"
        placeholder="Select a outbound"
        options={outbounds.map((out) => ({ label: out.tag, value: out.tag }))}
      />
      <DrawerInput label="Port" />
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
      />
      <DrawerMonaco label="Source" language="json" />
      <DrawerSelect
        label="Inbound tag"
        placeholder="Select a inbound"
        options={inbounds.map((inbound) => ({
          label: inbound.tag,
          value: inbound.tag,
        }))}
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
      />
      <DrawerMonaco label="Attr" language="starlark" />
      <DrawerInput label="Balance tag" />
    </Drawer>
  );
};

export default RoutingDrawer;
