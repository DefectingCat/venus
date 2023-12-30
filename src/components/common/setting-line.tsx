import clsx from 'clsx';
import { ReactNode } from 'react';

const Setting = ({ children }: { children: ReactNode }) => {
  return (
    <div className={clsx('grid grid-cols-[auto_1fr]', 'items-center gap-5')}>
      {children}
    </div>
  );
};

export const SettingLine = ({
  title,
  children,
}: {
  title: ReactNode;
  children: ReactNode;
}) => {
  return (
    <>
      <div className="flex justify-end">{title}</div>
      {children}
    </>
  );
};

export default Setting;
