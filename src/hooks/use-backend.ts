import { invoke } from '@tauri-apps/api/tauri';
import { message } from 'antd';
import { useCallback } from 'react';
import useStore from 'store';
import { CoreConfig, RConfig } from 'store/config-store';

const useBackend = () => {
  const { updateRconfig, updateCoreConfig } = useStore();

  const reloadRconfig = useCallback(async () => {
    try {
      const rua = await invoke<RConfig>('get_rua_config');
      updateRconfig(rua);
    } catch (err) {
      console.error(err);
      message.error('Get rua config failed', err.toString());
    }
  }, []);

  const reloadCoreCOnfig = useCallback(async () => {
    try {
      const core = await invoke<CoreConfig>('get_core_config');
      updateCoreConfig(core);
    } catch (err) {
      console.error(err);
      message.error('Get core config failed', err.toString());
    }
  }, []);

  return {
    reloadRconfig,
    reloadCoreCOnfig,
  };
};

export default useBackend;
