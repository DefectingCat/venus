import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import { Input, Modal, message } from 'antd';
import { ChangeEventHandler, useState } from 'react';
import { URL_VALID } from 'utils/consts';
import useBackend from 'hooks/use-backend';
import useStore from 'store';

const SubscriptionAdder = ({ onCancel }: { onCancel: () => void }) => {
  const { subscription: subs } = useStore();
  const { reloadSubs } = useBackend();

  const [open, setOpen] = useBoolean(true);
  // Add subscripition
  const [subscripition, setSubscripiton] = useState({
    name: '',
    url: '',
  });
  const [status, setStatus] = useState<'' | 'error'>('');
  const handleName: ChangeEventHandler<HTMLInputElement> = (e) => {
    const value = e.target.value.trim();
    setSubscripiton((d) => ({ ...d, name: value }));
  };
  const handleSub: ChangeEventHandler<HTMLInputElement> = (e) => {
    const value = e.target.value.trim();
    const valid = URL_VALID.test(value);
    setStatus(!subscripition ? '' : valid ? '' : 'error');
    setSubscripiton((d) => ({ ...d, url: value }));
  };
  // Send request
  const [loading, setLoading] = useBoolean(false);
  const handlAdd = async () => {
    try {
      setLoading.setTrue();
      const index = subs.findIndex((sub) => sub.url === subscripition.url);
      if (~index) return message.warning('Subscription already added');
      await invoke('add_subscription', {
        ...subscripition,
        name: subscripition.name || 'Unnamed',
      });
      message.success('Add subscripition success');
      await reloadSubs();
      setOpen.setFalse();
    } catch (err) {
      console.error(err);
      message.error(`Failed to add subscripition ${err?.toString()}`);
    } finally {
      setLoading.setFalse();
      setStatus('');
      setSubscripiton({
        name: '',
        url: '',
      });
    }
  };

  return (
    <Modal
      title="Add subscription"
      open={open}
      onOk={handlAdd}
      onCancel={setOpen.setFalse}
      afterClose={onCancel}
      okButtonProps={{
        loading,
        disabled: !subscripition.url || status === 'error',
      }}
    >
      <div className="flex items-center mb-2 mr-2">
        <div className="w-14">Name: </div>
        <div className="relative">
          <Input
            value={subscripition.name}
            onChange={handleName}
            allowClear
            placeholder="Unnamed"
            disabled={loading}
          />
        </div>
      </div>
      <div className="flex items-center mr-2">
        <div className="w-14">URL: </div>
        <div className="relative">
          <Input
            value={subscripition.url}
            onChange={handleSub}
            allowClear
            placeholder="Subscription url"
            status={status}
            disabled={loading}
          />
        </div>
      </div>
    </Modal>
  );
};

export default SubscriptionAdder;
