import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import {
  Button,
  Modal,
  Popconfirm,
  Popover,
  QRCode,
  Tooltip,
  message,
} from 'antd';
import clsx from 'clsx';
import useVaildUrl from 'hooks/use-vaild-url';
import dynamic from 'next/dynamic';
import { ChangeEvent, useState } from 'react';
import { AiOutlineDelete, AiOutlineShareAlt } from 'react-icons/ai';
import { BsPencilSquare } from 'react-icons/bs';
import { RxUpdate } from 'react-icons/rx';
import useStore from 'store';
import { Subscription } from 'store/config-store';
import styles from './subscription-card.module.scss';

const SubsModal = dynamic(() => import('components/common/subs-modal'));

/**
 * Find specific subscription in subscription array.
 *
 * @param subs subscription array
 * @param url target url
 */
const findSub = (subs: Subscription[], url: string) => {
  const target = subs.find((s) => s.url === url);
  if (!target) throw new Error('Cannot find target subscription');
  return target;
};

const SubscriptionCard = ({ sub }: { sub: Subscription }) => {
  const updateSubs = useStore((s) => s.updateSubs);
  const updateConfig = useStore((s) => s.updateConfig);

  // edit subscription state
  const [loading, setLoading] = useBoolean(false);
  // local subscription buffer, when changed will be dispatch to global
  const [buffer, setBuffer] = useState({ name: sub.name, url: sub.url });
  // modal state
  const [open, setOpen] = useBoolean(false);
  const { status, setStatus, vaild } = useVaildUrl();
  const handleSubs = (type: 'name' | 'url') => {
    const map = {
      name: (e: ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value.trim();
        setBuffer((d) => ({ ...d, name: value }));
      },
      url: (e: ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value.trim();
        const valid = vaild(value);
        setStatus(valid ? '' : 'error');
        setBuffer((d) => ({ ...d, url: value }));
      },
    };
    return map[type];
  };
  const handleOk = () => {
    updateSubs((subs) => {
      try {
        setLoading.setTrue();
        const target = findSub(subs, sub.url);
        target.name = buffer.name;
        target.url = buffer.url;
        setOpen.setFalse();
      } catch (err) {
        message.error(err);
        console.error(err);
      } finally {
        setLoading.setFalse();
      }
    });
  };

  // delete state
  const handleDelete = () => {
    updateSubs((subs) => {
      const index = subs.findIndex((s) => s.url === sub.url);
      if (!~index) {
        console.error('Cannot find target subscription');
        message.error('Cannot find target subscription');
      }
      subs.splice(index, 1);
    });
    updateConfig((config) => {
      (async () => {
        try {
          await invoke('update_config', { ruaConfig: config.rua });
        } catch (err) {
          console.error(err);
          message.error(err);
        }
      })();
    });
  };

  // update state
  const handleUpdate = async () => {
    try {
      setLoading.setTrue();
      await invoke('update_sub', { url: sub.url });
      message.success(`Update subscription ${sub.name} success`);
    } catch (err) {
      console.error(err);
      message.error(err);
    } finally {
      setLoading.setFalse();
    }
  };

  return (
    <>
      <div
        className={clsx(
          'rounded-lg bg-white shadow-gray-50',
          'p-4 cursor-pointer dark:bg-rua-gray-700',
          'hover:shadow-lg transition-all',
          'duration-300 select-none w-56',
          'flex flex-col mr-4 mb-4'
        )}
      >
        <div
          className={clsx('mb-2 text-lg text-gray-800', 'dark:text-gray-400')}
        >
          {sub.name}
        </div>
        <div
          className={clsx(
            'text-ellipsis overflow-hidden break-keep',
            'text-sm text-gray-600'
          )}
        >
          {sub.url}
        </div>
        <div className={clsx('flex items-center', 'mt-4')}>
          <Tooltip title="Edit">
            <Button
              shape="circle"
              className={clsx('mr-2', 'flex justify-center items-center')}
              onClick={setOpen.setTrue}
            >
              <BsPencilSquare />
            </Button>
          </Tooltip>
          <Tooltip title="Update" className={clsx(styles['update-btn'])}>
            <Button
              shape="circle"
              className={clsx('mr-2', 'flex justify-center items-center')}
              loading={loading}
              disabled={loading}
              onClick={handleUpdate}
            >
              <RxUpdate className={clsx(loading && 'hidden')} />
            </Button>
          </Tooltip>
          <Tooltip title="Share">
            <Popover
              trigger="click"
              overlayInnerStyle={{ padding: 0 }}
              content={<QRCode value={sub.url} bordered={false} />}
            >
              <Button
                shape="circle"
                className={clsx('mr-2', 'flex justify-center items-center')}
              >
                <AiOutlineShareAlt />
              </Button>
            </Popover>
          </Tooltip>
          <Tooltip title="Delete">
            <Popconfirm
              title="Delete this subscription?"
              description={'will be delete all nodes in this subscription'}
              onConfirm={handleDelete}
            >
              <Button
                shape="circle"
                className={clsx('mr-2', 'flex justify-center items-center')}
                danger
              >
                <AiOutlineDelete />
              </Button>
            </Popconfirm>
          </Tooltip>
        </div>
      </div>

      {/* Edit modal */}
      <Modal
        title="Edit subscription"
        open={open}
        onCancel={setOpen.setFalse}
        onOk={handleOk}
        confirmLoading={loading}
      >
        <SubsModal
          subs={buffer}
          status={status}
          onChange={handleSubs}
          loading={loading}
        />
      </Modal>
    </>
  );
};

export default SubscriptionCard;
