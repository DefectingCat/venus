import { Switch } from 'antd';
import clsx from 'clsx';
import Title from 'components/pages/page-title';
import useBackend from 'hooks/use-backend';
import MainLayout from 'layouts/main-layout';
import { useEffect, useRef, useState } from 'react';
import useStore from 'store';

const Logging = () => {
  const logs = useStore((s) => s.logs);
  const total = useStore((s) => s.total);
  const enable = useStore((s) => s.rua.logging);

  const updateConfig = useStore((s) => s.updateConfig);
  const { writeConfig } = useBackend();

  // auto scroll to bottom
  const [autoScroll, setAutoScroll] = useState(true);
  const ref = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (!autoScroll || !ref.current) return;
    ref.current.scrollTop = ref.current.scrollHeight;
  }, [autoScroll, total]);

  return (
    <MainLayout>
      <div className={clsx('flex h-full', 'flex-col')}>
        <div className={clsx('mt-1 mb-4')}>
          <Title>Logging</Title>
        </div>

        <div className="flex items-center mb-2">
          <div className="flex items-center mr-4">
            <div className="mr-2">Logging</div>
            <Switch
              checked={enable}
              onChange={async (checked) => {
                updateConfig((config) => {
                  config.rua.logging = checked;
                });
                writeConfig('rua');
              }}
            />
          </div>

          <div className="flex items-center">
            <div className="mr-2">Auto scroll</div>
            <Switch
              checked={autoScroll}
              onChange={(checked) => setAutoScroll(checked)}
            />
          </div>
        </div>

        <div
          className={clsx(
            'flex-1 rounded-lg bg-gray-200',
            'p-6 overflow-x-auto',
            'dark:bg-gray-800',
            'select-auto',
          )}
          ref={ref}
        >
          <pre className="m-0">
            {logs.map((log) => (
              <div key={log.id} className={clsx('px-2', 'pb-2 last:pb-0')}>
                <code>{log.content}</code>
              </div>
            ))}
          </pre>
        </div>
      </div>
    </MainLayout>
  );
};

export default Logging;
