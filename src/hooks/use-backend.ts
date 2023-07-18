import { invoke } from '@tauri-apps/api/tauri';
import { App } from 'antd';
import { useCallback } from 'react';
import useStore from 'store';
import { CoreConfig, RConfig } from 'store/config-store';

const writeConfigMap = {
  core: 'coreConfig',
  rua: 'ruaConfig',
};

const useBackend = () => {
  const { message } = App.useApp();
  const { updateRConfig, updateCoreConfig, updateConfig } = useStore();

  /**
   * Get newest config from backend
   */
  const reloadConfig = useCallback(async (type: 'core' | 'rua') => {
    const map = {
      rua: async () => {
        const rua = await invoke<RConfig>('get_config', { configType: 'rua' });
        updateRConfig(rua);
      },
      core: async () => {
        const core = await invoke<CoreConfig>('get_config', {
          configType: 'core',
        });
        updateCoreConfig(core);
      },
    };
    try {
      map[type]();
    } catch (err) {
      message.error('Get rua config failed', err.toString());
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  /**
   * Send current config in global state to backend
   */
  const writeConfig = useCallback(
    (type: 'core' | 'rua' | ('core' | 'rua')[]) => {
      updateConfig((config) => {
        (async () => {
          try {
            if (Array.isArray(type)) {
              type.forEach(async (t) => {
                await invoke('update_config', {
                  [`${writeConfigMap[t]}`]: config[t],
                });
              });
            } else {
              await invoke('update_config', {
                [`${writeConfigMap[type]}`]: config[type],
              });
            }
            message.success('Update config success');
          } catch (err) {
            message.error(err);
          }
        })();
      });
    },
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  return {
    reloadConfig,
    writeConfig,
  };
};

export default useBackend;
