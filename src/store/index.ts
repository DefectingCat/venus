import { create } from 'zustand';
import createConfigSlice, { ConfigSlice } from './config-store';
import createLogSlice, { LogSlice } from './log-store';
import createUISlice, { UISlice } from './ui-store';

const useStore = create<ConfigSlice & LogSlice & UISlice>()((...a) => ({
  ...createConfigSlice(...a),
  ...createLogSlice(...a),
  ...createUISlice(...a),
}));

export default useStore;
