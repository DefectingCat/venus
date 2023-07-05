import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';

const Logging = () => {
  return (
    <MainLayout>
      <div className={clsx('flex h-full', 'flex-col')}>
        <div className={clsx('mt-1 mb-4')}>
          <Title>Logging</Title>
        </div>

        <div className={clsx('flex-1 rounded-lg bg-gray-200')}></div>
      </div>
    </MainLayout>
  );
};

export default Logging;
