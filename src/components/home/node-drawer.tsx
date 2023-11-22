import { useBoolean } from 'ahooks';
import { Drawer, QRCode } from 'antd';
import dynamic from 'next/dynamic';
import useStore from 'store';
import { Node } from 'store/config-store';
import { NodeDrawerType } from 'store/ui-store';

const DrawerItem = dynamic(() => import('components/home/drawer-item'));

const NodeDrawer = ({ node }: { node: Node }) => {
  const [open, setOpen] = useBoolean(true);
  const toggleUI = useStore((s) => s.toggleUI);

  const type = useStore((s) => s.menus.node);
  const typeMap: { [key in NodeDrawerType]: JSX.Element } = {
    editor: (
      <>
        <DrawerItem label="Protocol" value={node.nodeType} />
        <DrawerItem label="Name" value={node.ps} />
        <DrawerItem label="Address" value={node.add} />
        <DrawerItem label="Port" value={node.port} />
        <DrawerItem label="Net Type" value={node.net} />
        <DrawerItem label="AlertID" value={node.aid} />
        <DrawerItem label="Host" value={node.host} />
        <DrawerItem label="Path" value={node.path} />
        <DrawerItem label="TLS" value={node.tls} />
        <DrawerItem label="Alpn" value={node.alpn} />
        <DrawerItem label="Link" value={node.rawLink} />
      </>
    ),
    share: (
      <>
        <DrawerItem label="Name" value={node.ps} />
        <DrawerItem label="Link" value={node.rawLink} />
        <div className="flex items-center justify-center w-full">
          <QRCode size={330} value={node.rawLink} />
        </div>
      </>
    ),
  };

  return (
    <Drawer
      title="Node"
      open={open}
      onClose={() => {
        setOpen.setFalse();
        setTimeout(() => {
          toggleUI((ui) => {
            ui.menus.node = false;
          });
        }, 300);
      }}
    >
      {type && typeMap?.[type]}
    </Drawer>
  );
};

export default NodeDrawer;
