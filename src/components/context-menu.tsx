import clsx from 'clsx';
import useStore from 'store';
import { MenuType } from 'store/ui-store';

export const ContextID = 'rua-context-menu';
const MenuItemClass = clsx(
  'transition-all hover:bg-gray-200',
  'px-4 py-2 select-none',
  'cursor-pointer'
);

/**
 * Right click context menu
 */
const ContextMenu = () => {
  const pos = useStore((s) => s.mousePos);
  const type = useStore((s) => s.showMenu);
  const toggleUI = useStore((s) => s.toggleUI);

  const menuMap: { [key in MenuType]: JSX.Element } = {
    node: (
      <>
        <div
          className={MenuItemClass}
          onClick={() => {
            toggleUI((ui) => {
              ui.menus.nodeDrawer = true;
            });
          }}
        >
          Edit
        </div>
        <div className={MenuItemClass}>Share</div>
        <div className={MenuItemClass}>Delete</div>
      </>
    ),
    global: (
      <>
        <div className={MenuItemClass}>{type}</div>
        <div className={MenuItemClass}>ContextMenu</div>
      </>
    ),
  };

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
      {menuMap[type]}
    </div>
  );
};

export default ContextMenu;
