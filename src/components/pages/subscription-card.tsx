import { Button, Modal } from 'antd';
import clsx from 'clsx';
import { Subscription } from 'store';
import { BsPencilSquare } from 'react-icons/bs';
import { AiOutlineShareAlt, AiOutlineDelete } from 'react-icons/ai';
import { useBoolean } from 'ahooks';
import dynamic from 'next/dynamic';

const SubsModal = dynamic(() => import('components/common/subs-modal'));

const SubscriptionCard = ({ sub }: { sub: Subscription }) => {
  // edit subscription state
  const [open, setOpen] = useBoolean(false);

  return (
    <>
      <div
        className={clsx(
          'rounded-lg bg-white shadow-gray-50',
          'p-4 cursor-pointer dark:bg-rua-gray-700',
          'hover:shadow-lg transition-all',
          'duration-300 select-none w-56',
          'flex flex-col'
        )}
      >
        <div
          className={clsx('mb-2 text-lg text-gray-800', 'dark:text-gray-400')}
        >
          {sub.name}
        </div>
        <div
          className={clsx(
            'text-ellipsis overflow-hidden break-keep',
            'text-sm text-gray-600'
          )}
        >
          {sub.url}
        </div>
        <div className={clsx('flex items-center', 'mt-4')}>
          <Button
            shape="circle"
            className={clsx('mr-2', 'flex justify-center items-center')}
            onClick={setOpen.setTrue}
          >
            <BsPencilSquare />
          </Button>
          <Button
            shape="circle"
            className={clsx('mr-2', 'flex justify-center items-center')}
          >
            <AiOutlineShareAlt />
          </Button>
          <Button
            shape="circle"
            className={clsx('mr-2', 'flex justify-center items-center')}
            danger
          >
            <AiOutlineDelete />
          </Button>
        </div>
      </div>

      <Modal title="Edit subscription" open={open} onCancel={setOpen.setFalse}>
        <SubsModal subs={{ name: sub.name, url: sub.url }} />
      </Modal>
    </>
  );
};

export default SubscriptionCard;
