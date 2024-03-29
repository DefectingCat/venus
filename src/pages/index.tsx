import { Tabs, TabsProps } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import useStore from 'store';

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
  const current = useStore((s) => s.tabs.index);
  const toggleUI = useStore((s) => s.toggleUI);

  const childrenMap = {
    1: <Subscriptions />,
    2: <Nodes />,
  };

  return (
    <>
      <MainLayout>
        <div className={clsx('flex flex-col h-full')}>
          <div className="">
            <div className="mt-1 mb-4">
              <Title>Proxies</Title>
            </div>

            <Tabs
              activeKey={current}
              items={tabItems}
              onChange={(key) =>
                toggleUI((ui) => {
                  ui.tabs.index = key;
                })
              }
            />
          </div>

          {childrenMap[current]}
        </div>
      </MainLayout>
    </>
  );
}

export default App;
