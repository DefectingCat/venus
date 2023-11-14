import { App, Button } from 'antd';
import clsx from 'clsx';
import { invoke } from '@tauri-apps/api/tauri';
import useStore from 'store';

const SystemTray = () => {
  const { message } = App.useApp();
  const mainVisible = useStore((s) => s.venus.mainVisible);
  const handleShow = async () => {
    try {
      await invoke('toggle_main', { show: !mainVisible });
    } catch (err) {
      message.error(err.toString());
    }
  };

  return (
    <>
      <div
        className={clsx(
          'rounded-lg overflow-hidden bg-white',
          'h-[600px] flex p-4 dark:bg-rua-gray-900',
        )}
      >
        <div>
          <Button onClick={handleShow}>{mainVisible ? 'Hide' : 'Show'}</Button>
        </div>
      </div>
    </>
  );
};

export default SystemTray;
