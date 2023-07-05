import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import useStore from 'store';

const Logging = () => {
  const logs = useStore((s) => s.logs);

  return (
    <MainLayout>
      <div className={clsx('flex h-full', 'flex-col')}>
        <div className={clsx('mt-1 mb-4')}>
          <Title>Logging</Title>
        </div>

        <div className={clsx('flex-1 rounded-lg bg-gray-200', 'p-6')}>
          {logs.map((log) => (
            <div>{log}</div>
          ))}
        </div>
      </div>
    </MainLayout>
  );
};

export default Logging;
