import { invoke } from '@tauri-apps/api/tauri';
import type { Node } from 'store/config-store';
import { App, Table, Tooltip } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import { useCallback, useMemo, useState } from 'react';
import { ResizeCallbackData } from 'react-resizable';
import useStore from 'store';
import styles from 'styles/index.module.scss';
import dynamic from 'next/dynamic';

const ResizableTitle = dynamic(
  () => import('components/pages/resizable-title')
);
const NodeDrawer = dynamic(() => import('components/home/node-drawer'));

const Nodes = () => {
  const { message } = App.useApp();
  const subscriptions = useStore((s) => s.rua.subscriptions);
  const toggleUI = useStore((s) => s.toggleUI);

  const nodes = useMemo(
    () => subscriptions.flatMap((sub) => sub.nodes),
    [subscriptions]
  );

  // current outbound in config file
  const outbound = useStore(
    (s) => s.core?.outbounds?.[0]?.settings?.vnext?.[0]
  );

  // nodes table
  const [columns, setColumns] = useState<ColumnsType<Node>>([
    {
      title: 'ID',
      ellipsis: {
        showTitle: false,
      },
      key: 'nodeId',
      width: 80,
      render: (_, _node, i) => (
        <div className={clsx('text-ellipsis', 'break-keep overflow-hidden')}>
          {i + 1}
        </div>
      ),
    },
    {
      title: 'Name',
      dataIndex: 'ps',
      key: 'ps',
      ellipsis: {
        showTitle: false,
      },
      width: 300,
      sorter: (a, b) => a.ps.localeCompare(b.ps),
      render: (addr) => (
        <Tooltip placement="topLeft" title={addr}>
          <div className="text-ellipsis overflow-hidden">{addr}</div>
        </Tooltip>
      ),
    },
    {
      title: 'Address',
      dataIndex: 'add',
      key: 'add',
      ellipsis: {
        showTitle: false,
      },
      width: 100,
      render: (addr) => (
        <Tooltip placement="topLeft" title={addr}>
          <div className="text-ellipsis overflow-hidden">{addr}</div>
        </Tooltip>
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
  ]);
  const handleResize: Function =
    (index: number) =>
    (_: React.SyntheticEvent<Element>, { size }: ResizeCallbackData) => {
      const newColumns = [...columns];
      newColumns[index] = {
        ...newColumns[index],
        width: size.width,
      };
      setColumns(newColumns);
    };
  const mergeColumns: ColumnsType<Node> = columns.map((col, index) => ({
    ...col,
    onHeaderCell: (column: ColumnsType<Node>[number]) => ({
      width: column.width,
      onResize: handleResize(index) as React.ReactEventHandler<any>,
    }),
  }));

  // Select node
  const selected = useMemo(() => {
    const target = nodes.find(
      (n) => `${n.add}${n.port}` === `${outbound?.address}${outbound?.port}`
    );
    return `${target?.add}${target?.port}`;
  }, [nodes, outbound]);
  const updateConfig = useStore((s) => s.updateConfig);
  const handleSelect = useCallback(async (node: Node) => {
    try {
      await invoke('select_node', {
        subName: node.subs,
        nodeId: node.nodeId,
      });
      updateConfig((config) => {
        config.rua.core_status = 'Restarting';
      });
    } catch (err) {
      message.error(err);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // Edit or view node, open by context menu
  const nodeDrawer = useStore((s) => s.menus.nodeDrawer);
  const [currentNode, setCurrentNode] = useState<Node>(null);

  return (
    <>
      <Table
        className={styles.table}
        components={{
          header: {
            cell: ResizableTitle,
          },
        }}
        pagination={{ pageSize: 100 }}
        rowKey={(record) => record.add + record.ps}
        columns={mergeColumns}
        dataSource={nodes}
        scroll={{
          x: '100%',
        }}
        onRow={(record) => ({
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
            });
            setCurrentNode(record);
          },
          className: clsx(
            'cursor-pointer select-none',
            `${record.add}${record.port}` === selected
              ? 'bg-gray-300 dark:bg-gray-900'
              : 'hover:bg-[#fafafa] hover:dark:bg-gray-800',
            'transition-all duration-300'
          ),
        })}
      />

      {nodeDrawer && <NodeDrawer node={currentNode} />}
    </>
  );
};

export default Nodes;
