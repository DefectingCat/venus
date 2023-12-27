import { useBoolean } from 'ahooks';
import { Drawer, QRCode } from 'antd';
import { DrawerInput } from 'components/common/drawer-item';
import useStore from 'store';
import { Node } from 'store/config-store';
import { NodeDrawerType } from 'store/ui-store';

const NodeDrawer = ({ node }: { node: Node }) => {
  const [open, setOpen] = useBoolean(true);
  const toggleUI = useStore((s) => s.toggleUI);

  const type = useStore((s) => s.menus.node);
  const typeMap: { [key in NodeDrawerType]: JSX.Element } = {
    editor: (
      <>
        <DrawerInput label="Protocol" value={node.nodeType} />
        <DrawerInput label="Name" value={node.ps} />
        <DrawerInput label="Address" value={node.add} />
        <DrawerInput label="Port" value={node.port} />
        <DrawerInput label="Net Type" value={node.net} />
        <DrawerInput label="AlertID" value={node.aid} />
        <DrawerInput label="Host" value={node.host} />
        <DrawerInput label="Path" value={node.path} />
        <DrawerInput label="TLS" value={node.tls} />
        <DrawerInput label="Alpn" value={node.alpn} />
        <DrawerInput label="Link" value={node.rawLink} />
      </>
    ),
    share: (
      <>
        <DrawerInput label="Name" value={node.ps} />
        <DrawerInput label="Link" value={node.rawLink} />
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
