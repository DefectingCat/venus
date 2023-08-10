import { useBoolean } from 'ahooks';
import { Drawer, Input, QRCode, theme } from 'antd';
import clsx from 'clsx';
import useStore from 'store';
import { Node } from 'store/config-store';
import { NodeDrawerType } from 'store/ui-store';

const { useToken } = theme;

const DrawerItem = ({
  label,
  value,
  onChange,
}: {
  label: string;
  value?: string;
  onChange?: () => void;
}) => {
  const token = useToken();

  return (
    <div className={clsx('relative flex items-center', 'mb-4')}>
      <Input value={value} />
      <div
        className={clsx(
          'absolute left-2 top-[-8px]',
          'text-gray-600 dark:text-gray-400',
          'text-xs',
        )}
        style={{
          background: token.token.colorBgElevated,
        }}
      >
        {label}
      </div>
    </div>
  );
};

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
