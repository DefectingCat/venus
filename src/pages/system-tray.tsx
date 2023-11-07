import { App, Button } from 'antd';
import clsx from 'clsx';
import { invoke } from '@tauri-apps/api/tauri';

const SystemTray = () => {
  const { message } = App.useApp();
  const handleShow = async (show: boolean) => {
    try {
      await invoke('toggle_main', { show });
    } catch (err) {
      message.error(err.toString());
    }
  };

  return (
    <>
      <div
        className={clsx(
          'rounded-lg overflow-hidden bg-white',
          'h-full flex p-4',
        )}
      >
        <div>
          <Button onClick={() => handleShow(true)}>Show</Button>
          <Button onClick={() => handleShow(false)}>Hide</Button>
        </div>
      </div>
    </>
  );
};

export default SystemTray;
