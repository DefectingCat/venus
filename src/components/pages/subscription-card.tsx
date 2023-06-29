import { Button } from 'antd';
import clsx from 'clsx';
import { Subscription } from 'store';
import { BsPencilSquare } from 'react-icons/bs';
import { AiOutlineShareAlt, AiOutlineDelete } from 'react-icons/ai';

const SubscriptionCard = ({ sub }: { sub: Subscription }) => {
  return (
    <div
      className={clsx(
        'rounded-lg bg-white shadow-gray-50',
        'p-4 cursor-pointer dark:bg-rua-gray-700',
        'hover:shadow-md transition-all',
        'duration-300 select-none w-52',
        'flex flex-col'
      )}
    >
      <div className={clsx('mb-2 text-lg text-gray-800', 'dark:text-gray-400')}>
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
        <Button shape="circle" className="mr-2">
          <BsPencilSquare />
        </Button>
        <Button shape="circle" className="mr-2">
          <AiOutlineShareAlt />
        </Button>
        <Button shape="circle" className="mr-2">
          <AiOutlineDelete />
        </Button>
      </div>
    </div>
  );
};

export default SubscriptionCard;
