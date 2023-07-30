import { invoke } from '@tauri-apps/api/tauri';
import type { Node } from 'store/config-store';
import { App, Table } from 'antd';
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
    },
    {
      title: 'Speed',
      dataIndex: 'speed',
      key: 'delay',
      width: 80,
      ellipsis: true,
    },
    {
      title: 'Connectivity',
      dataIndex: 'connectivity',
      key: 'connectivity',
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
  const drawerType = useStore((s) => s.menus.node);
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
        pagination={false}
        rowKey={(record) => record.add + record.ps}
        scroll={{
          y: '100%',
        }}
        columns={mergeColumns}
        dataSource={nodes}
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
              ui.menus.clickNode = [record];
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

      {!!drawerType && <NodeDrawer node={currentNode} />}
    </>
  );
};

export default Nodes;
