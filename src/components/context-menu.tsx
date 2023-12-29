import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import clsx from 'clsx';
import useStore from 'store';
import { MenuType } from 'store/ui-store';

export const ContextID = 'rua-context-menu';
/**
 * System tray and content menu item class
 */
export const MenuItemClass = clsx(
  'transition-all hover:bg-gray-200',
  'px-4 py-1 select-none',
  'cursor-pointer rounded-lg',
  'mb-1 last:mb-0',
  'dark:hover:bg-gray-700',
);

/**
 * Right click context menu
 */
const ContextMenu = () => {
  const pos = useStore((s) => s.mousePos);
  const type = useStore((s) => s.showMenu);
  const clickNode = useStore((s) => s.menus.clickNode);
  const { toggleUI, closeMenus } = useStore((s) => ({
    toggleUI: s.toggleUI,
    closeMenus: s.closeMenus,
  }));
  const updateConfig = useStore((s) => s.updateConfig);
  const menus = useStore((s) => s.menus);

  const menuMap: { [key in MenuType]: JSX.Element } = {
    routing: (
      <>
        <div
          className={MenuItemClass}
          onClick={() => {
            closeMenus();
            toggleUI((ui) => {
              ui.menus.routing = 'editor';
            });
          }}
        >
          Edit
        </div>
        <div
          className={MenuItemClass}
          onClick={() => {
            closeMenus();
            updateConfig((config) => {
              config.core.routing.rules.splice(menus.clickRule, 1);
            });
          }}
        >
          Delete
        </div>
      </>
    ),
    node: (
      <>
        <div
          className={MenuItemClass}
          onClick={() => {
            closeMenus();
            toggleUI((ui) => {
              ui.menus.node = 'editor';
            });
          }}
        >
          Edit
        </div>
        <div
          className={MenuItemClass}
          onClick={() => {
            closeMenus();
            toggleUI((ui) => {
              ui.menus.node = 'share';
            });
          }}
        >
          Share
        </div>
        <div className={MenuItemClass}>Delete</div>
        <div
          className={MenuItemClass}
          onClick={async () => {
            try {
              await invoke('node_speed', {
                nodeId: clickNode.nodeId,
              });
            } catch (err) {
              message.error(err.toString());
            }
          }}
        >
          Test speed
        </div>
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
        'py-1 rounded-lg shadow-lg',
        'transition-all opacity-0',
        'w-52 text-sm px-1',
      )}
      style={{ left: pos.x + 10, top: pos.y + 8, opacity: type ? 1 : 0 }}
      id={ContextID}
    >
      {menuMap[type]}
    </div>
  );
};

export default ContextMenu;
