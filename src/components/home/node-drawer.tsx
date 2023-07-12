import { useBoolean } from 'ahooks';
import { Drawer } from 'antd';
import useStore from 'store';
import { Node } from 'store/config-store';

const NodeDrawer = ({ node }: { node: Node }) => {
  const [open, setOpen] = useBoolean(true);
  const toggleUI = useStore((s) => s.toggleUI);

  return (
    <Drawer
      title="Node"
      open={open}
      onClose={() => {
        toggleUI((ui) => {
          ui.showMenu = null;
          ui.menus.nodeDrawer = false;
        });
        setOpen.setFalse();
      }}
    >
      123
    </Drawer>
  );
};

export default NodeDrawer;
