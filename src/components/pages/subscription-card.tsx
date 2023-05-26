import clsx from 'clsx';
import { ReactNode } from 'react';

const SubscriptionCard = ({ children }: { children: ReactNode }) => {
  return (
    <div
      className={clsx(
        'rounded-lg bg-white shadow-gray-50',
        'p-4 cursor-pointer dark:bg-rua-gray-700',
        'hover:shadow-md transition-all',
        'duration-300 select-none max-w-[10rem]'
      )}
    >
      {children}
    </div>
  );
};

export default SubscriptionCard;
