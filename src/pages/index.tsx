import { Tabs, TabsProps } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import { useState } from 'react';

const Subscriptions = dynamic(() => import('components/home/subscriptions'));
const Nodes = dynamic(() => import('components/home/nodes'));

function App() {
  const tabItems: TabsProps['items'] = [
    {
      key: '1',
      label: 'Subscription',
    },
    {
      key: '2',
      label: 'Nodes',
    },
  ];
  const [current, setCurrent] = useState('1');
  const childrenMap = {
    1: <Subscriptions />,
    2: <Nodes />,
  };

  return (
    <>
      <MainLayout>
        <div className="mt-1 mb-4">
          <Title>Proxies</Title>
        </div>

        <Tabs
          accessKey={current}
          items={tabItems}
          onChange={(key) => setCurrent(key)}
        />
        {childrenMap[current]}
      </MainLayout>
    </>
  );
}

export default App;
