import clsx from 'clsx';
import { DetailedHTMLProps, HTMLAttributes, ReactNode } from 'react';

export interface ButtonProps
  extends DetailedHTMLProps<
    HTMLAttributes<HTMLButtonElement>,
    HTMLButtonElement
  > {
  children: ReactNode;
}

const Button = ({ children, ...rest }: ButtonProps) => {
  const { className, ...props } = rest;

  return (
    <button
      type="button"
      className={clsx(
        'text-gray-900 bg-white border border-gray-200',
        'dark:hover:bg-gray-700 dark:hover:border-gray-600',
        'mr-2 mb-2 dark:bg-gray-800 dark:text-white',
        'focus:ring-gray-200 font-medium rounded-lg ',
        'focus:outline-none hover:bg-gray-100',
        'dark:focus:ring-gray-700 text-sm px-5 py-2.5',
        'dark:border-gray-600 focus:ring-4',
        'transition-all active:bg-gray-200',
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};

export default Button;
