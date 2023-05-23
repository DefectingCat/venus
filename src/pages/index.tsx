import { invoke } from '@tauri-apps/api/tauri';
import { Button, Input } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { ChangeEventHandler, useState } from 'react';
import { URL_VALID } from 'utils/consts';

function App() {
  // Add subscripition
  const [subscripition, setSubscripiton] = useState('');
  const [status, setStatus] = useState<'' | 'error'>('');
  const handleSub: ChangeEventHandler<HTMLInputElement> = (e) => {
    const value = e.target.value.trim();
    const valid = URL_VALID.test(value);
    setStatus(!subscripition ? '' : valid ? '' : 'error');
    setSubscripiton(value);
  };
  const handlAdd = async () => {
    const res = await invoke('add_subscription', { url: subscripition });
    console.log(res);
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
            <div className="flex items-center  mr-2">
              <div className="mr-2">URL</div>
              <div className="relative">
                <Input
                  value={subscripition}
                  onChange={handleSub}
                  allowClear
                  placeholder="Subscription url"
                  status={status}
                />
              </div>
            </div>
            <Button
              disabled={!subscripition || status === 'error'}
              onClick={handlAdd}
              className="mr-2"
            >
              Add
            </Button>
            <Button>Update All</Button>
          </div>
        </div>
      </MainLayout>
    </>
  );
}

export default App;
