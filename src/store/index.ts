import { create } from 'zustand';

interface Subscription {
  name: string;
  url: string;
}
interface Node {
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
}
interface VConfig {
  subscription: Subscription[];
  nodes: Node[];
  updateSubscription: (subscription: Subscription[]) => void;
  updateNodes: (nodes: Node[]) => void;
}

const useStore = create<VConfig>()((set) => ({
  subscription: [],
  nodes: [],
  updateSubscription: (subscription) => {
    set(() => ({
      subscription,
    }));
  },
  updateNodes: (nodes) => {
    set(() => ({
      nodes,
    }));
  },
}));

export default useStore;
