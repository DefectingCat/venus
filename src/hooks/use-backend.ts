import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore, { Node, Subscription } from 'store';

const useBackend = () => {
  const { updateNodes, updateSubscription } = useStore();

  const reloadNodes = useCallback(async () => {
    try {
      const nodesString = await invoke<string>('get_rua_nodes');
      const nodes = JSON.parse(nodesString) as Node[] | null;
      nodes?.length && updateNodes(nodes);
    } catch (err) {
      console.error(err);
      message.error('Get nodes failed', err.toString());
    }
  }, []);

  const reloadSubs = useCallback(async () => {
    try {
      const subsString = await invoke<string>('get_subscriptions');
      const subs = JSON.parse(subsString) as Subscription[] | null;
      subs?.length && updateSubscription(subs);
    } catch (err) {}
  }, []);

  return {
    reloadNodes,
    reloadSubs,
  };
};

export default useBackend;
