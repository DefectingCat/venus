import { Rule } from 'store/config-store';

export const URL_VALID =
  /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_+.~#?&//=]*)/;

export const DEFAULT_ROUTING_RULE: Rule = {
  type: 'field',
  ip: [],
  domain: [],
  outboundTag: '',
  port: '',
  network: '',
  source: [],
  inboundTag: [],
  protocol: [],
  attrs: '',
  balancerTag: '',
};
