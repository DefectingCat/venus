import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import { App as AntApp, Button, Empty } from 'antd';
import clsx from 'clsx';
import useLoading from 'hooks/use-loading';
import dynamic from 'next/dynamic';
import useStore from 'store';

const SubscriptionAdder = dynamic(
  () => import('components/pages/subscription-adder'),
);
const SubscriptionCard = dynamic(
  () => import('components/pages/subscription-card'),
);

const Subscriptions = () => {
  const { message } = AntApp.useApp();
  const [open, setOpen] = useBoolean(false);
  const subscriptions = useStore((s) => s.rua.subscriptions);

  // Update subscriptions
  const [loading, setLoading] = useLoading('updateAll');
  const handleUpdate = async () => {
    try {
      setLoading.setTrue();
      await invoke('update_all_subs');
      message.success('Update sucess');
    } catch (err) {
      message.error(err.toString());
    } finally {
      setLoading.setFalse();
    }
  };

  return (
    <>
      <div className="flex items-center">
        <Button className="mr-2" onClick={setOpen.setTrue}>
          Add
        </Button>
        <Button
          onClick={handleUpdate}
          loading={loading}
          disabled={!subscriptions.length}
        >
          Update All
        </Button>
      </div>
      <div className={clsx('mt-4 flex flex-wrap', 'items-center ')}>
        {!!subscriptions.length ? (
          subscriptions.map((sub) => (
            <SubscriptionCard key={sub.url} sub={sub} />
          ))
        ) : (
          <Empty />
        )}
      </div>

      {open && <SubscriptionAdder onCancel={setOpen.setFalse} />}
    </>
  );
};

export default Subscriptions;
