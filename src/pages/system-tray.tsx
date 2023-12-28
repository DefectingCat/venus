import { App } from 'antd';
import clsx from 'clsx';
import { invoke } from '@tauri-apps/api/tauri';
import useStore from 'store';
import { MenuItemClass } from 'components/context-menu';
import { GrPowerShutdown } from 'react-icons/gr';
import { LuRefreshCcw } from 'react-icons/lu';
import { BsWindowDesktop } from 'react-icons/bs';

const TrayMenu = clsx(MenuItemClass, 'flex items-center');

/**
 * The system tray menu.
 *
 * In tauri window called `menu`
 * @returns
 */
const SystemTray = () => {
  const { message } = App.useApp();
  const mainVisible = useStore((s) => s.venus.mainVisible);
  const handleShow = async () => {
    try {
      await Promise.all([
        invoke('toggle_window', { label: 'main', show: true }),
        invoke('toggle_window', { label: 'menu', show: false }),
      ]);
    } catch (err) {
      message.error(err.toString());
    }
  };

  const handleCore = async () => {
    try {
      await Promise.all([
        invoke('restart_core'),
        invoke('toggle_window', { label: 'menu', show: false }),
      ]);
    } catch (err) {
      message.error(err.toString());
    }
  };

  const handleExit = async () => {
    try {
      await invoke('exit_app');
    } catch (err) {
      message.error(err.toString());
    }
  };

  return (
    <>
      <div
        className={clsx(
          'rounded-lg overflow-hidden bg-white',
          'h-[600px] flex p-2 dark:bg-rua-gray-900',
          'flex-col',
        )}
      >
        <div className={TrayMenu} onClick={handleShow}>
          <BsWindowDesktop className="mr-2" />
          <div>{mainVisible ? 'Hide all windows' : 'Show windows'}</div>
        </div>
        <div className={TrayMenu} onClick={handleCore}>
          <LuRefreshCcw className="mr-2" />
          <div>Restart Core</div>
        </div>
        <div className={TrayMenu} onClick={handleExit}>
          <GrPowerShutdown className="mr-2" />
          <div>Quit</div>
        </div>
      </div>
    </>
  );
};

export default SystemTray;
