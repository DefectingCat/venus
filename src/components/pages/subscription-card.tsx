import { invoke } from '@tauri-apps/api/tauri';
import { useBoolean } from 'ahooks';
import { App, Button, Modal, Popconfirm, Popover, QRCode, Tooltip } from 'antd';
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
import useBackend from 'hooks/use-backend';
import useLoading from 'hooks/use-loading';
import { shallow } from 'zustand/shallow';

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
  const { message } = App.useApp();
  const { updateSubs, updateConfig } = useStore(
    (s) => ({
      updateSubs: s.updateSubs,
      updateConfig: s.updateConfig,
    }),
    shallow,
  );

  const { writeConfig } = useBackend();

  // edit subscription state
  const [loading, setLoading] = useLoading('subCrad', sub.url);
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
        if (!subs) return;
        const target = findSub(subs, sub.url);
        if (target.name !== buffer.name) {
          target.nodes.forEach((node) => {
            node.subs = buffer.name;
          });
        }
        target.name = buffer.name;
        target.url = buffer.url;
        setOpen.setFalse();
      } catch (err) {
        message.error(err);
      } finally {
        setLoading.setFalse();
      }
    });
    writeConfig('rua');
  };

  // delete state
  const handleDelete = () => {
    updateSubs((subs) => {
      if (!subs) return;
      const index = subs?.findIndex((s) => s.url === sub.url);
      if (!~index) {
        message.error('Cannot find target subscription');
      }
      subs.splice(index, 1);
    });
    updateConfig((config) => {
      if (!config.core) return;
      if (config.core.outbounds.length === 3) {
        config.core.outbounds.shift();
      }
    });
    writeConfig(['core', 'rua']);
  };

  // update state
  const handleUpdate = async () => {
    try {
      setLoading.setTrue();
      await invoke('update_sub', { url: sub.url });
      message.success(`Update subscription ${sub.name} success`);
    } catch (err) {
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
          'flex flex-col mr-4 mb-4',
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
            'text-sm text-gray-600',
          )}
        >
          {sub.url}
        </div>
        <div className={clsx('flex items-center', 'mt-4')}>
          <Tooltip title="Edit">
            <div>
              <Button
                shape="circle"
                className={clsx('mr-2', 'flex justify-center items-center')}
                onClick={setOpen.setTrue}
              >
                <BsPencilSquare className="dark:text-gray-500" />
              </Button>
            </div>
          </Tooltip>
          <Tooltip title="Update" className={clsx(styles['update-btn'])}>
            <div>
              <Button
                shape="circle"
                className={clsx('mr-2', 'flex justify-center items-center')}
                loading={loading}
                disabled={loading}
                onClick={handleUpdate}
              >
                <RxUpdate
                  className={clsx(loading ? 'hidden' : 'dark:text-gray-500')}
                />
              </Button>
            </div>
          </Tooltip>
          <Tooltip title="Share">
            <div>
              <Popover
                trigger="click"
                overlayInnerStyle={{ padding: 0 }}
                content={<QRCode value={sub.url} bordered={false} />}
              >
                <Button
                  shape="circle"
                  className={clsx('mr-2', 'flex justify-center items-center')}
                >
                  <AiOutlineShareAlt className="dark:text-gray-500" />
                </Button>
              </Popover>
            </div>
          </Tooltip>
          <Tooltip title="Delete">
            <div>
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
            </div>
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
