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
  core_status?: 'Started' | 'Restarting' | 'Stopped';
  subscriptions: Subscription[] | null;
}

export interface VConfig {
  rua: RConfig;
  core: CoreConfig;
  updateRconfig: (config: RConfig) => void;
  updateCoreConfig: (config: CoreConfig) => void;
}

export interface Inbound {
  port: number;
}
export interface CoreConfig {
  inbounds: Inbound[];
}

const useStore = create<VConfig>()((set) => ({
  rua: {
    core_status: 'Stopped',
    subscriptions: [],
  },
  core: null,
  updateRconfig: (rua) => {
    set(() => ({
      rua,
    }));
  },
  updateCoreConfig: (core) => {
    set(() => ({
      core,
    }));
  },
}));

export default useStore;
