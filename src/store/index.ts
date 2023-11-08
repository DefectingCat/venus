import { StoreApi, UseBoundStore, create } from 'zustand';
import createConfigSlice, { ConfigSlice } from './config-store';
import createLogSlice, { LogSlice } from './log-store';
import createUISlice, { UISlice } from './ui-store';

const useStore = create<ConfigSlice & LogSlice & UISlice>()((...a) => ({
  ...createConfigSlice(...a),
  ...createLogSlice(...a),
  ...createUISlice(...a),
}));

type WithSelectors<S> = S extends { getState: () => infer T }
  ? S & { use: { [K in keyof T]: () => T[K] } }
  : never;

const createSelectors = <S extends UseBoundStore<StoreApi<object>>>(
  _store: S,
) => {
  let store = _store as WithSelectors<typeof _store>;
  store.use = {};
  for (let k of Object.keys(store.getState())) {
    (store.use as any)[k] = () => store((s) => s[k as keyof typeof s]);
  }
  return store;
};
export const store = createSelectors(useStore);

export default useStore;
