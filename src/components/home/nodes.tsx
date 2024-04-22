import { invoke } from '@tauri-apps/api/tauri';
import { App } from 'antd';
import { AnyObject } from 'antd/es/_util/type';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import dynamic from 'next/dynamic';
import { useCallback, useMemo, useState } from 'react';
import { BsCheckCircleFill, BsFillDashCircleFill } from 'react-icons/bs';
import useStore from 'store';
import type { Node } from 'store/config-store';

const ResizableTable = dynamic(
  () => import('components/common/resizeable-table'),
);
const NodeDrawer = dynamic(() => import('components/home/node-drawer'));
const LoadingIcon = dynamic(() => import('components/common/loading-icon'));

const Nodes = () => {
  const { message } = App.useApp();
  const subscriptions = useStore((s) => s.rua.subscriptions);
  const nodeLoading = useStore((s) => s.loading.node.speedTest);

  const nodes = useMemo(
    () => subscriptions?.flatMap((sub) => sub.nodes),
    [subscriptions],
  );

  // nodes table
  const columns: ColumnsType<AnyObject> = useMemo(
    () => [
      {
        title: 'ID',
        ellipsis: true,
        key: 'nodeId',
        width: 50,
        render: (_, _node, i) => (
          <div className={clsx('text-ellipsis', 'break-keep overflow-hidden')}>
            {i + 1}
          </div>
        ),
      },
      {
        title: 'Name',
        dataIndex: 'ps',
        ellipsis: true,
        key: 'ps',
        width: 300,
        sorter: (a, b) => a.ps.localeCompare(b.ps),
        render: (addr) => (
          <div className="overflow-hidden text-ellipsis">{addr}</div>
        ),
      },
      {
        title: 'Address',
        dataIndex: 'add',
        key: 'add',
        ellipsis: true,
        width: 100,
        render: (addr) => (
          <div className="overflow-hidden text-ellipsis">{addr}</div>
        ),
      },
      {
        title: 'Port',
        dataIndex: 'port',
        key: 'port',
        width: 80,
        ellipsis: true,
      },
      {
        title: 'Delay',
        dataIndex: 'delay',
        key: 'delay',
        width: 80,
        ellipsis: true,
        render: (delay) => (
          <div className="overflow-hidden text-ellipsis">
            {delay != null && `${delay}ms`}
          </div>
        ),
      },
      {
        title: 'Speed',
        dataIndex: 'speed',
        key: 'speed',
        width: 80,
        ellipsis: true,
        render: (speed) => (
          <div className="overflow-hidden text-ellipsis">
            {speed != null && `${speed}MB/s`}
          </div>
        ),
      },
      {
        title: 'Connectivity',
        dataIndex: 'connectivity',
        key: 'connectivity',
        width: 80,
        ellipsis: true,
        render: (connectivity: boolean, record: Node) => {
          const current = nodeLoading.find(
            (n) => n.id === record.nodeId,
          )?.loading;
          return (
            <div
              className={clsx(
                'overflow-hidden text-ellipsis flex',
                'items-center',
              )}
            >
              {current ? (
                <LoadingIcon />
              ) : connectivity == null ? (
                ''
              ) : connectivity ? (
                <BsCheckCircleFill />
              ) : (
                <BsFillDashCircleFill />
              )}
            </div>
          );
        },
      },
      {
        title: 'Net Type',
        dataIndex: 'net',
        key: 'net',
        width: 80,
        ellipsis: true,
      },
      {
        title: 'TLS',
        dataIndex: 'tls',
        key: 'tls',
        width: 80,
        ellipsis: true,
      },
      {
        title: 'Subscription',
        dataIndex: 'subs',
        key: 'subs',
        width: 100,
        ellipsis: true,
      },
    ],
    [nodeLoading],
  );

  // Select node
  const current = useStore((s) => s.rua.currentId);
  const toggleUI = useStore((s) => s.toggleUI);
  const handleSelect = useCallback(async (node: Node) => {
    try {
      await invoke('select_node', {
        nodeId: node.nodeId,
      });
    } catch (err) {
      message.error(err);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Edit or view node, open by context menu
  const drawerType = useStore((s) => s.menus.node);
  const [currentNode, setCurrentNode] = useState<Node | null>(null);

  return (
    <div className="overflow-auto flex-1">
      <ResizableTable
        pagination={false}
        rowKey={(record: Node) => record.add + record.ps}
        scroll={{
          y: '100%',
        }}
        columns={columns}
        dataSource={nodes}
        onRow={(record: Node) => ({
          onDoubleClick: () => {
            handleSelect(record);
          },
          onContextMenu: (e) => {
            e.stopPropagation();
            e.preventDefault();
            toggleUI((ui) => {
              ui.showMenu = 'node';
              ui.mousePos = {
                x: e.clientX,
                y: e.clientY,
              };
              ui.menus.clickNode = record;
            });
            setCurrentNode(record);
          },
          className: clsx(
            'cursor-pointer select-none',
            record.nodeId === current
              ? 'bg-gray-300 dark:bg-gray-900'
              : 'hover:bg-[#fafafa] hover:dark:bg-gray-800',
            'transition-all duration-300',
          ),
        })}
      />

      {!!drawerType && currentNode && <NodeDrawer node={currentNode} />}
    </div>
  );
};

export default Nodes;
