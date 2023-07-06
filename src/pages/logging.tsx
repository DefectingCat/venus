import { invoke } from '@tauri-apps/api/tauri';
import { Switch, message } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import { useEffect, useRef } from 'react';
import useStore from 'store';

const Logging = () => {
  const logs = useStore((s) => s.logs);
  const enable = useStore((s) => s.enable);
  const updateLogging = useStore((s) => s.updateLogging);

  // scroll to bottom
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    ref.current.scrollTop = ref.current.scrollHeight;
  }, [logs]);

  return (
    <MainLayout>
      <div className={clsx('flex h-full', 'flex-col')}>
        <div className={clsx('mt-1 mb-4')}>
          <Title>Logging</Title>
        </div>

        <div className="flex items-center mb-2">
          <div className="mr-2">Logging</div>
          <div>
            <Switch
              checked={enable}
              onChange={async (checked) => {
                updateLogging((log) => {
                  log.enable = checked;
                });
                try {
                  await invoke('toggle_logging', { enable: checked });
                } catch (err) {
                  console.error(err);
                  message.error(err);
                }
              }}
            />
          </div>
        </div>
        <div
          className={clsx(
            'flex-1 rounded-lg bg-gray-200',
            'p-6 overflow-x-auto',
            'dark:bg-gray-800'
          )}
          ref={ref}
        >
          <pre className="m-0">
            {logs.map((log) => (
              <div className={clsx('px-2', 'pb-2 last:pb-0')}>
                <code>{log}</code>
              </div>
            ))}
          </pre>
        </div>
      </div>
    </MainLayout>
  );
};

export default Logging;
