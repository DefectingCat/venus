import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import { DrawerInput, DrawerMonaco } from 'components/common/drawer-item';

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
      <DrawerMonaco label="IP" defaultLanguage="json" />
      <DrawerInput />
    </Drawer>
  );
};

export default RoutingDrawer;
