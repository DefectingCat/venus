import clsx from 'clsx';
import { ReactNode } from 'react';

const MainLayout = ({ children }: { children: ReactNode }) => {
  return (
    <main
      className={clsx(
        'flex w-[100vw] h-[100vh]',
        'bg-bluish-gray dark:bg-rua-gray-900'
      )}
    >
      {children}
    </main>
  );
};

export default MainLayout;
