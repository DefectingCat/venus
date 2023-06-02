import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore, { Subscription } from 'store';

const useBackend = () => {
  const { updateSubscription } = useStore();

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

  return {
    reloadSubs,
  };
};

export default useBackend;