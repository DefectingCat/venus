import clsx from 'clsx';
import { DetailedHTMLProps, HTMLAttributes, ReactNode } from 'react';

interface NaviItemProps
  extends DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement> {
  active: boolean;
  children: ReactNode;
}

const NaviItem = ({ active, children, ...rest }: NaviItemProps) => {
  const { className, ...props } = rest;

  return (
    <div
      className={clsx(
        'rounded-md flex',
        'px-4 py-2 cursor-pointer',
        'transition-all',
        'select-none',
        active && 'bg-white',
        active || 'hover:bg-gray-200',
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};

export default NaviItem;
