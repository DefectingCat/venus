import { StateCreator } from 'zustand';
import { immer } from 'zustand/middleware/immer';
import { LogSlice } from './log-store';
import { UISlice } from './ui-store';

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
  rawLink: string;
}

export interface CoreConfig {
  log: Log;
  inbounds: Inbound[];
  outbounds: Outbound[];
  routing: Routing;
  dns: DNS;
  policy: Policy;
  other: Other;
}

export interface DNS {
  hosts: Hosts;
  servers: ServerElement[];
}

export interface Hosts {
  'domain:v2fly.org': string;
  'domain:github.io': string;
  'domain:wikipedia.org': string;
  'domain:shadowsocks.org': string;
}

export type ServerElement = ServerClass | string;

export interface ServerClass {
  address: string;
  port: number;
  domains: string[];
}

export interface Inbound {
  port: number;
  listen: string;
  tag: string;
  protocol: string;
  settings: InboundSettings;
  sniffing: Sniffing;
}

export interface InboundSettings {
  auth: string;
  udp: boolean;
  ip: string;
}

export interface Sniffing {
  enabled: boolean;
  destOverride: string[];
  routeOnly?: boolean;
}

export interface Log {
  loglevel: string;
  access: null;
  error: null;
}

export interface Other {}

export interface Outbound {
  protocol: string;
  settings: OutboundSettings;
  tag: string;
  proxySetting: null;
  mux: null;
}

export interface OutboundSettings {
  vnext: Vnext[] | null;
}

export interface Vnext {
  address: string;
  port: number;
  users: User[];
}

export interface User {
  id: string;
  alterId: number;
  email: string;
  security: string;
}

export interface Policy {
  levels: Levels;
  system: System;
}

export interface Levels {
  '0': The0;
}

export interface The0 {
  uplinkOnly: number;
  downlinkOnly: number;
}

export interface System {
  statsInboundUplink: boolean;
  statsInboundDownlink: boolean;
  statsOutboundUplink: boolean;
  statsOutboundDownlink: boolean;
}

export interface Routing {
  domainStrategy: string;
  rules: Rule[];
}

export interface Rule {
  type: string;
  ip: string[];
  outboundTag: string;
  domain: string[];
}

export interface RConfig {
  logging: boolean;
  version: string;
  save_windows: boolean;
  core_status?: 'Started' | 'Restarting' | 'Stopped';
  subscriptions: Subscription[] | null;
}

export interface VConfig {
  rua: RConfig;
  core: CoreConfig | null;
}
export interface Actions {
  /**
   * Set rua config from backend to global state
   */
  updateRConfig: (config: RConfig) => void;
  /**
   * Set core config from backend to global state
   */
  updateCoreConfig: (config: CoreConfig) => void;
  updateSubs: (callback: (subs: Subscription[]) => void) => void;

  /**
   * Update whole config with immer
   */
  updateConfig: (callback: (config: VConfig) => void) => void;
  /**
   * Update the socks inbound settings
   */
  updateSocksInbound: (callback: (socksInbound: Inbound) => void) => void;
  updateHttpInbound: (callback: (socksInbound: Inbound) => void) => void;
}

export type ConfigSlice = VConfig & Actions;
const createConfigSlice: StateCreator<
  ConfigSlice & LogSlice & UISlice,
  [],
  [['zustand/immer', never]],
  ConfigSlice
> = immer<ConfigSlice>((set) => ({
  rua: {
    logging: false,
    version: '',
    save_windows: true,
    core_status: 'Stopped',
    subscriptions: [],
  },
  core: null,
  updateRConfig: (rua) => {
    set(() => ({
      rua,
    }));
  },
  updateCoreConfig: (core) => {
    set(() => ({
      core,
    }));
  },
  updateConfig: (callback) => {
    set(callback);
  },
  updateSubs: (callback) => {
    set((config) => {
      const subs = config.rua.subscriptions;
      callback(subs);
    });
  },
  /**
   * Only update socks inbound with immer.
   */
  updateSocksInbound: (callback) => {
    set((config) => {
      const socks = config.core.inbounds.find((i) => i.tag === 'socks');
      if (!socks) throw new Error('Cannot find socks inbound');
      callback(socks);
    });
  },
  updateHttpInbound: (callback) => {
    set((config) => {
      const http = config.core.inbounds.find((i) => i.tag === 'http');
      if (!http) throw new Error('Cannot find http inbound');
      callback(http);
    });
  },
}));
export default createConfigSlice;
