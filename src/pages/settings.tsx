import { Tabs, TabsProps } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import useStore from 'store';

const BasicSettings = dynamic(
  () => import('components/settings/basic-settings'),
);
const VenusSetting = dynamic(() => import('components/settings/venus-setting'));

const Settings = () => {
  const tabItems: TabsProps['items'] = [
    {
      key: '1',
      label: 'Basic Setting',
    },
    {
      key: '2',
      label: 'Core Basic',
    },
  ];
  const current = useStore((s) => s.tabs.setting);
  const toggleUI = useStore((s) => s.toggleUI);

  const children = {
    1: <VenusSetting />,
    2: <BasicSettings />,
  };

  return (
    <MainLayout>
      <div className={clsx('mt-1 mb-4')}>
        <Title>Settings</Title>
      </div>

      <Tabs
        activeKey={current}
        items={tabItems}
        onChange={(key) =>
          toggleUI((ui) => {
            ui.tabs.setting = key;
          })
        }
      />
      {children[current]}
    </MainLayout>
  );
};

export default Settings;
