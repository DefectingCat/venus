import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore, { Node } from 'store';

const useBackend = () => {
  const { updateNodes, updateSubscription } = useStore();

  const reloadNodes = useCallback(async () => {
    try {
      const nodesString = await invoke<string>('get_rua_nodes');
      const nodes = JSON.parse(nodesString) as Node[];
      updateNodes(nodes);
    } catch (err) {
      message.error('Get nodes failed');
    }
  }, []);

  return {
    reloadNodes,
  };
};

export default useBackend;
