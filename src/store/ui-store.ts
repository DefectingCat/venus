import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';
import { LogSlice } from './log-store';

export type MenuType = 'global' | 'node';
export type NodeDrawerType = 'editor' | 'share';
export interface UI {
  // content menu on right click
  showMenu: MenuType | null;
  // mouse position when right click
  mousePos: {
    x: number;
    y: number;
  };
  // control by context menu
  menus: {
    // node editor
    nodeDrawer: boolean;
    // node qrcode
    nodeShare: boolean;
    // node menus
    node: NodeDrawerType | false;
  };

  // loadings
  loading: {
    // update all loading
    updateAll: boolean;
    // subs card loading
    subCrad: {
      url: string;
      loading: boolean;
    }[];
  };
  // current selected tabs
  tabs: {
    index: string;
    setting: string;
  };
}
export interface UIAction {
  toggleUI: (callback: (ui: UI) => void) => void;
  closeMenus: () => void;
}

export type UISlice = UI & UIAction;

const createUISlice: StateCreator<
  UISlice & ConfigSlice & LogSlice,
  [],
  [['zustand/immer', never]],
  UISlice
> = immer<UISlice>((set) => ({
  showMenu: null,
  mousePos: {
    x: 0,
    y: 0,
  },
  menus: {
    nodeDrawer: false,
    nodeShare: false,
    node: false,
  },
  loading: {
    updateAll: false,
    subCrad: [],
  },
  tabs: {
    index: '1',
    setting: '1',
  },
  toggleUI(callback) {
    set(callback);
  },
  closeMenus() {
    set((ui) => {
      Object.keys(ui.menus).forEach((key) => {
        ui.menus[key] = false;
      });
    });
  },
}));

export default createUISlice;
