import clsx from 'clsx';
import SideBar from 'components/side-bar';
import { ReactNode } from 'react';

const MainLayout = ({ children }: { children: ReactNode }) => {
  return (
    <main
      className={clsx(
        'flex w-[100vw] h-[100vh]',
        'bg-bluish-gray dark:bg-rua-gray-900'
      )}
    >
      {/* navi */}
      <SideBar />

      {/* body */}
      <div className="p-8">{children}</div>
    </main>
  );
};

export default MainLayout;
