import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import React from 'react';

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
    >
      test
    </Drawer>
  );
};

export default RoutingDrawer;
