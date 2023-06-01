import { useBoolean } from 'ahooks';
import { Button, Table, Tooltip } from 'antd';
import type { ColumnsType } from 'antd/es/table';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import { useState } from 'react';
import { ResizeCallbackData } from 'react-resizable';
import useStore, { Node } from 'store';

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
  const { nodes, subscription } = useStore();
  console.log(nodes);

  // nodes table
  const [columns, setColumns] = useState<ColumnsType<Node>>([
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
      ellipsis: {
        showTitle: false,
      },
      width: 100,
      render: (id) => (
        <Tooltip placement="topLeft" title={id}>
          <div className={clsx('text-ellipsis', 'break-keep overflow-hidden')}>
            {id}
          </div>
        </Tooltip>
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
            <Button>Update All</Button>
          </div>
          <div className="mt-4">
            {subscription.map((sub) => (
              <SubscriptionCard key={sub.url} sub={sub} />
            ))}
          </div>
        </div>

        <div>
          <Title.h2>Nodes</Title.h2>
          <Table
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
              x: 800,
            }}
          />
        </div>
      </MainLayout>

      {open && <SubscriptionAdder onCancel={setOpen.setFalse} />}
    </>
  );
}

export default App;