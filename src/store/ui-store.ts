import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';
import { LogSlice } from './log-store';

export interface UI {
  // content menu on right click
  showMenu: boolean;
  // mouse position when right click
  mousePos: {
    x: number;
    y: number;
  };
}
export interface UIAction {
  toggleUI: (callback: (ui: UI) => void) => void;
}

export type UISlice = UI & UIAction;

const createUISlice: StateCreator<
  UISlice & ConfigSlice & LogSlice,
  [],
  [['zustand/immer', never]],
  UISlice
> = immer<UISlice>((set) => ({
  showMenu: false,
  mousePos: {
    x: 0,
    y: 0,
  },
  toggleUI(callback) {
    set(callback);
  },
}));

export default createUISlice;
