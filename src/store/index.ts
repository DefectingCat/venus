import { create } from 'zustand';
import createConfigSlice, { ConfigSlice } from './config-store';
import createLogSlice, { LogSlice } from './log-store';

const useStore = create<ConfigSlice & LogSlice>()((...a) => ({
  ...createConfigSlice(...a),
  ...createLogSlice(...a),
}));

export default useStore;
