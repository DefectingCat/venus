import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import DrawerItem, { DrawerInput } from 'components/common/drawer-item';
import Monaco from 'components/monaco';

const RoutingDrawer = ({
  drawerType,
  onClose,
}: {
  drawerType: string;
  onClose: () => void;
}) => {
  const [open, setOpen] = useBoolean(true);

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
      <DrawerItem label="IP">
        <Monaco />
      </DrawerItem>
      <DrawerInput />
    </Drawer>
  );
};

export default RoutingDrawer;
