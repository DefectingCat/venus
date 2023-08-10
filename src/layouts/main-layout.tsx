import clsx from 'clsx';
import SideBar from 'components/side-bar';
import dynamic from 'next/dynamic';
import { ReactNode } from 'react';
import useStore from 'store';

const ContextMenu = dynamic(() => import('components/context-menu'));

/**
 * Entier App layout
 */
const MainLayout = ({ children }: { children: ReactNode }) => {
  const showMenu = useStore((s) => s.showMenu);

  return (
    <>
      <main
        className={clsx(
          'flex w-[100vw] h-[100vh]',
          'bg-bluish-gray dark:bg-rua-gray-900',
          'relative',
        )}
      >
        <div
          data-tauri-drag-region
          className={clsx(
            'h-7 bg-transparent fixed',
            'top-0 left-0 right-0 z-[1000]',
          )}
        ></div>

        {/* navi */}
        <SideBar />

        {/* body */}
        <div className="p-8 flex-1 overflow-auto h-full">{children}</div>
      </main>

      {showMenu && <ContextMenu />}
    </>
  );
};

export default MainLayout;
