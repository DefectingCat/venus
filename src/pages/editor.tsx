import { Button, message } from 'antd';
import clsx from 'clsx';
import Monaco from 'components/monaco';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';

/**
 * Config file editor
 */
const Editor = () => {
  const [coreConfig, setCoreConfig] = useState();
  useEffect(() => {
    const read = async () => {
      try {
        const config = await invoke('read_config_file', {
          which: 'Core',
        });
        setCoreConfig(config);
      } catch (err) {
        console.error(err);
        message.error(err);
      }
    };
    read();
  }, []);

  return (
    <MainLayout>
      <div className="flex flex-col h-full">
        <div className={clsx('mt-1 ')}>
          <Title>Settings</Title>
        </div>
        <Monaco
          wrapperClass="flex"
          height="100%"
          value={coreConfig}
          language="json"
        />
        <div className="mt-2">
          <Button className="mr-2">Save</Button>
          <Button>Rest</Button>
        </div>
      </div>
    </MainLayout>
  );
};

export default Editor;
