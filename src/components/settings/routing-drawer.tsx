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
    </Drawer>
  );
};

export default RoutingDrawer;
