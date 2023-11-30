import { App, Button, Tooltip } from 'antd';
import useBackend from 'hooks/use-backend';
import React from 'react';
import useStore from 'store';

const ApplyBtn = () => {
  const { message } = App.useApp();
  const coreStatus = useStore((s) => s.venus.coreStatus);
  const toggleUI = useStore((s) => s.toggleUI);
  const { writeConfig } = useBackend();

  const handleApply = async () => {
    try {
      toggleUI((ui) => {
        ui.venus.coreStatus = 'Restarting';
      });
      writeConfig(['rua', 'core']);
    } catch (err) {
      message.error(err);
    }
  };

  return (
    <Tooltip placement="top" title="Apply and restart core">
      <Button
        loading={coreStatus === 'Restarting'}
        disabled={coreStatus === 'Stopped'}
        onClick={handleApply}
      >
        Apply
      </Button>
    </Tooltip>
  );
};

export default ApplyBtn;
