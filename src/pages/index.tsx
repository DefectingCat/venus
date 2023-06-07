import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import { Button, Table, Tooltip, message } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import { useCallback, useState } from 'react';
import { ResizeCallbackData } from 'react-resizable';
import useStore, { Node } from 'store';
import styles from 'styles/index.module.scss';

const SubscriptionAdder = dynamic(
  () => import('components/pages/subscription-adder')
);
const SubscriptionCard = dynamic(
  () => import('components/pages/subscription-card')
);
const ResizableTitle = dynamic(
  () => import('components/pages/resizable-title')
);

function App() {
  const [open, setOpen] = useBoolean(false);
  const subscriptions = useStore((s) => s.subscriptions);

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
  const [selected, setSelected] = useState('');
  const handleSelect = useCallback(async (node: Node) => {
    try {
      await invoke('select_node', {
        subName: node.subs,
        nodeId: node.nodeId,
      });
      setSelected(node.nodeId);
    } catch (err) {
      console.error(err);
    }
  }, []);

  // Update subscriptions
  const [loading, setLoading] = useBoolean(false);
  const handleUpdate = async () => {
    try {
      setLoading.setTrue();
      await invoke('update_all_subs');
      message.success('Update sucess');
    } catch (err) {
      message.error(err.toString());
    } finally {
      setLoading.setFalse();
    }
  };

  return (
    <>
      <MainLayout>
        <div className="mt-1 mb-4">
          <Title>Proxies</Title>
        </div>

        <div>
          <Title.h2>Subscription</Title.h2>
          <div className="flex items-center">
            <Button className="mr-2" onClick={setOpen.setTrue}>
              Add
            </Button>
            <Button
              onClick={handleUpdate}
              loading={loading}
              disabled={!subscriptions.length}
            >
              Update All
            </Button>
          </div>
          <div className="mt-4">
            {subscriptions.map((sub) => (
              <SubscriptionCard key={sub.url} sub={sub} />
            ))}
          </div>
        </div>

        <div>
          <Title.h2>Nodes</Title.h2>
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
            dataSource={subscriptions.flatMap((sub) => sub.nodes)}
            scroll={{
              x: '100%',
            }}
            onRow={(record) => ({
              onDoubleClick: () => {
                handleSelect(record);
              },
              className: clsx(
                'cursor-pointer select-none',
                record.nodeId === selected
                  ? 'bg-gray-300'
                  : 'hover:bg-[#fafafa]',
                'transition-all duration-300'
              ),
            })}
          />
        </div>
      </MainLayout>

      {open && <SubscriptionAdder onCancel={setOpen.setFalse} />}
    </>
  );
}

export default App;
