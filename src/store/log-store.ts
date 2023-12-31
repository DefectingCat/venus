import { StateCreator } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { ConfigSlice } from './config-store';
import { UISlice } from './ui-store';

type VenusLog = {
  id: number;
  content: string;
};
export interface Logging {
  total: number;
  logs: VenusLog[];
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
  total: 0,
  logs: [],
  updateLogging(callback) {
    set(callback);
  },
}));

export default createLogSlice;
