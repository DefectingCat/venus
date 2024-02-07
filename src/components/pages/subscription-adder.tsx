import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import { App, Modal } from 'antd';
import useVaildUrl from 'hooks/use-vaild-url';
import dynamic from 'next/dynamic';
import { ChangeEvent, useState } from 'react';
import useStore from 'store';

const SubsModal = dynamic(() => import('components/common/subs-modal'));

const SubscriptionAdder = ({ onCancel }: { onCancel: () => void }) => {
  const { message } = App.useApp();
  const subscriptions = useStore((s) => s.rua.subscriptions);
  const subs = subscriptions;

  const [open, setOpen] = useBoolean(true);
  // Add subscripition
  const [subscripition, setSubscripiton] = useState({
    name: '',
    url: '',
  });
  const { status, setStatus, vaild } = useVaildUrl();
  const handleSubs = (type: 'name' | 'url') => {
    const map = {
      name: (e: ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value.trim();
        setSubscripiton((d) => ({ ...d, name: value }));
      },
      url: (e: ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value.trim();
        const valid = vaild(value);
        setStatus(!subscripition.url ? '' : valid ? '' : 'error');
        setSubscripiton((d) => ({ ...d, url: value }));
      },
    };
    return map[type];
  };
  // Send request
  const [loading, setLoading] = useBoolean(false);
  const handlAdd = async () => {
    try {
      setLoading.setTrue();
      const index = subs?.findIndex((sub) => sub.url === subscripition.url);
      if (index && ~index) return message.warning('Subscription already added');
      await invoke('add_subscription', {
        ...subscripition,
        name: subscripition.name || 'Unnamed',
      });
      message.success('Add subscripition success');
      setOpen.setFalse();
    } catch (err) {
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
      maskClosable={!loading}
    >
      <SubsModal
        subs={subscripition}
        status={status}
        loading={loading}
        onChange={handleSubs}
      />
    </Modal>
  );
};

export default SubscriptionAdder;
