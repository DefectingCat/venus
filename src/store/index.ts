import { create } from 'zustand';

export interface Subscription {
  name: string;
  url: string;
  nodes: Node[];
}
export interface Node {
  v: string;
  // Node name
  ps: string;
  // Address
  add: string;
  port: string;
  id: string;
  // AlertID
  aid: string;
  net: string;
  // Protocol type
  type: string;
  host: string;
  path: string;
  tls: string;
  sni: string;
  alpn: string;
  // Subscription group
  subs: string;
  delay: string;
  nodeId: string;
}

export interface RConfig {
  coreStatus: 'Started' | 'Restarting' | 'Stopped';
  subscription: Subscription[];
}
export interface VConfig extends RConfig {
  updateSubscription: (subscription: Subscription[]) => void;
  updateRconfig: (config: RConfig) => void;
}

const useStore = create<VConfig>()((set) => ({
  coreStatus: 'Stopped',
  subscription: [],
  updateSubscription: (subscription) => {
    set(() => ({
      subscription,
    }));
  },
  updateRconfig: (config) => {
    set(() => ({
      ...config,
    }));
  },
}));

export default useStore;