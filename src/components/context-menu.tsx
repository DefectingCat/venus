import clsx from 'clsx';
import useStore from 'store';

export const ContextID = 'rua-context-menu';

/**
 * Right click context menu
 */
const ContextMenu = () => {
  const pos = useStore((s) => s.mousePos);
  const type = useStore((s) => s.showMenu);

  return (
    <div
      className={clsx(
        'fixed bg-white dark:bg-rua-gray-800',
        'py-2 rounded-lg shadow-lg',
        'transition-all opacity-0'
      )}
      style={{ left: pos.x + 10, top: pos.y + 8, opacity: !!type ? 1 : 0 }}
      id={ContextID}
    >
      <div
        className={clsx(
          'transition-all hover:bg-gray-200',
          'px-4 py-2 select-none',
          'cursor-pointer'
        )}
      >
        {type}
      </div>
      <div
        className={clsx(
          'transition-all hover:bg-gray-200',
          'px-4 py-2 select-none',
          'cursor-pointer'
        )}
      >
        ContextMenu
      </div>
    </div>
  );
};

export default ContextMenu;
