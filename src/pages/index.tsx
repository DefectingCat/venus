import { useBoolean } from 'ahooks';
import type { ColumnsType } from 'antd/es/table';
import { Button, Table, Tooltip } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import useStore, { Node } from 'store';

const SubscriptionAdder = dynamic(
  () => import('components/pages/subscription-adder')
);
const SubscriptionCard = dynamic(
  () => import('components/pages/subscription-card')
);

function App() {
  const [open, setOpen] = useBoolean(false);
  const { nodes, subscription } = useStore();

  // nodes table
  const colums: ColumnsType<Node> = [
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
          <div className="text-ellipsis overflow-hidden">{id}</div>
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
    },
    {
      title: 'Type',
      dataIndex: 'type',
      key: 'type',
      width: 80,
    },
  ];

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
              <SubscriptionCard key={sub.url}>{sub.name}</SubscriptionCard>
            ))}
          </div>
        </div>

        <div>
          <Title.h2>Nodes</Title.h2>
          <Table className="flex-1" columns={colums} dataSource={nodes} />
        </div>
      </MainLayout>

      {open && <SubscriptionAdder onCancel={setOpen.setFalse} />}
    </>
  );
}

export default App;
