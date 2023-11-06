import { Button } from 'antd';
import clsx from 'clsx';

const SystemTray = () => {
  return (
    <>
      <div
        className={clsx(
          'rounded-lg overflow-hidden bg-white',
          'h-full flex p-4',
        )}
      >
        <div>
          <Button>Show</Button>
          <Button>Hide</Button>
        </div>
        123
      </div>
    </>
  );
};

export default SystemTray;
