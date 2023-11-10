import { LoadingOutlined } from '@ant-design/icons';
import clsx from 'clsx';
import useStore from 'store';

const StatusMap = {
  Started: <div className={clsx('bg-green-500 rounded-full', 'w-4 h-4')}></div>,
  Restarting: <LoadingOutlined />,
  Stopped: <div className={clsx('bg-red-500 rounded-full', 'w-4 h-4')}></div>,
};

const CoreStatus = () => {
  const coreStatus = useStore((s) => s.venus.coreStatus);
  const version = useStore((s) => s.venus.coreVersion);

  return (
    <>
      <div className="flex items-center">
        <div className="mr-2">{StatusMap[coreStatus]}</div>
        <div className="">Core {version}</div>
      </div>
    </>
  );
};

export default CoreStatus;
