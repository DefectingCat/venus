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

        <div
          className={clsx(
            'flex-1 rounded-lg bg-gray-200',
            'p-6 overflow-x-auto'
          )}
        >
          {/* <pre className="m-0"> */}
          {/*   <List data={logs} height={300} itemHeight={30} itemKey="id"> */}
          {/*     {(log) => ( */}
          {/*       <div className={clsx('h-[30px]')}> */}
          {/*         <code>{log}</code> */}
          {/*       </div> */}
          {/*     )} */}
          {/*   </List> */}
          {/* </pre> */}
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
