import { StateCreator } from 'zustand';
import { ConfigSlice } from './config-store';
import { immer } from 'zustand/middleware/immer';

export interface Logging {
  enable: boolean;
  logs: string[];
}
export interface LoggingAction {
  updateLogging: (callback: (log: Logging) => void) => void;
}

export type LogSlice = Logging & LoggingAction;

const createLogSlice: StateCreator<
  LogSlice & ConfigSlice,
  [],
  [['zustand/immer', never]],
  LogSlice
> = immer<LogSlice>((set) => ({
  enable: false,
  logs: [],
  updateLogging(callback) {
    set(callback);
  },
}));

export default createLogSlice;
