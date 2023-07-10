import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';
import { UISlice } from './ui-store';

export interface Logging {
  logs: string[];
}
export interface LoggingAction {
  updateLogging: (callback: (log: Logging) => void) => void;
}

export type LogSlice = Logging & LoggingAction;

const createLogSlice: StateCreator<
  LogSlice & ConfigSlice & UISlice,
  [],
  [['zustand/immer', never]],
  LogSlice
> = immer<LogSlice>((set) => ({
  logs: [],
  updateLogging(callback) {
    set(callback);
  },
}));

export default createLogSlice;
