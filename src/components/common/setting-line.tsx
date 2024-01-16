import clsx from 'clsx';
import { ReactNode } from 'react';

/**
 * The setting page wrapper
 */
export const Setting = ({ children }: { children: ReactNode }) => {
  return (
    <div className="flex justify-center mb-4">
      <div className="max-w-4xl flex-1">{children}</div>
    </div>
  );
};

/**
 * The setting lines set
 */
const SettingCard = ({
  children,
  title,
}: {
  children: ReactNode;
  title?: string;
}) => {
  return (
    <>
      {title && <div className="font-semibold pl-4 mb-1">{title}</div>}
      <div
        className={clsx(
          'rounded-lg bg-white p-5',
          'overflow-hidden flex-1',
          'mb-4 last:mb-0 dark:bg-rua-gray-800',
        )}
      >
        {children}
      </div>
    </>
  );
};

/**
 * The setting item single line
 */
export const SettingLine = ({
  title,
  children,
}: {
  title: ReactNode;
  children: ReactNode;
}) => {
  return (
    <>
      <div
        className={clsx(
          'flex items-center justify-between',
          'py-4 border-b border-solid',
          'border-t-0 border-l-0 border-r-0',
          'border-gray-100 last:border-none',
          'first:pt-0 last:pb-0',
          'dark:border-rua-gray-700',
        )}
      >
        <div className="flex">{title}</div>
        {children}
      </div>
    </>
  );
};

export default SettingCard;
