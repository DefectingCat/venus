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

export interface CoreConfig {}

export interface RConfig {
  coreStatus: 'Started' | 'Restarting' | 'Stopped';
  core_status?: 'Started' | 'Restarting' | 'Stopped';
  subscriptions: Subscription[] | null;
  // coreConfig:
}
export interface VConfig extends RConfig {
  updateSubscription: (subscription: Subscription[] | null) => void;
  updateRconfig: (config: RConfig) => void;
}

export interface Inbound {
  port: number;
}
export interface CoreConfig {
  inbounds: Inbound[];
}

const useStore = create<VConfig>()((set) => ({
  coreStatus: 'Stopped',
  coreConfig: {},
  subscriptions: [],
  updateSubscription: (subscriptions) => {
    set(() => ({
      subscriptions,
    }));
  },
  updateRconfig: (config) => {
    const { core_status: coreStatus, ...rest } = config;
    set(() => ({
      coreStatus,
      ...rest,
    }));
  },
}));

export default useStore;
