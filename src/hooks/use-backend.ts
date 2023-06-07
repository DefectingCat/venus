import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore, { RConfig, Subscription } from 'store';

const useBackend = () => {
  const { updateSubscription, updateRconfig } = useStore();

  const reloadSubs = useCallback(async () => {
    try {
      const subsString = await invoke<string>('get_subscriptions');
      const subs = JSON.parse(subsString) as Subscription[] | null;
      subs?.length && updateSubscription(subs);
    } catch (err) {
      console.error(err);
      message.error('Get subscription failed', err.toString());
    }
  }, []);

  const reloadRconfig = useCallback(async () => {
    try {
      const status = await invoke<string>('get_rua_config');
      const rua = JSON.parse(status) as RConfig | null;
      updateRconfig(rua);
    } catch (err) {
      console.error(err);
      message.error('Get rau config failed', err.toString());
    }
  }, []);

  return {
    reloadSubs,
    reloadRconfig,
  };
};

export default useBackend;
