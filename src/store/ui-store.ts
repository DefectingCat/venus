import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';
import { Node } from './config-store';
import { LogSlice } from './log-store';

export type MenuType = 'global' | 'node' | 'routing';
export type NodeDrawerType = 'editor' | 'share';
export type RoutingDrawerType = 'editor' | 'add';

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
    // node menus drawer
    node: NodeDrawerType | false;
    // right click node
    clickNode: Node | null;
    // routing menus drawer
    routing: RoutingDrawerType | false;
    // right clikc custom routing index
    clickRule: number;
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
    // node list loading
    node: {
      speedTest: {
        // speed testing node id
        id: string;
        loading: boolean;
      }[];
    };
  };
  // current selected tabs
  tabs: {
    index: string;
    setting: string;
  };
  // from backend
  venus: VenusUI;
}
export interface VenusUI {
  coreStatus?: 'Started' | 'Restarting' | 'Stopped';
  coreVersion: string;
  mainVisible: boolean;
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
    node: false,
    clickNode: null,
    routing: false,
    clickRule: -1,
  },
  loading: {
    updateAll: false,
    subCrad: [],
    node: {
      speedTest: [],
    },
  },
  tabs: {
    index: '1',
    setting: '1',
  },
  venus: {
    coreStatus: 'Stopped',
    coreVersion: '',
    mainVisible: true,
  },
  toggleUI(callback) {
    set(callback);
  },
  closeMenus() {
    set((ui) => {
      ['node', 'routing'].forEach((key) => {
        ui.menus[key] = false;
      });
    });
  },
}));

export default createUISlice;
