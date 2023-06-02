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
}
interface VConfig {
  subscription: Subscription[];
  updateSubscription: (subscription: Subscription[]) => void;
}

const useStore = create<VConfig>()((set) => ({
  subscription: [],
  updateSubscription: (subscription) => {
    set(() => ({
      subscription,
    }));
  },
}));

export default useStore;