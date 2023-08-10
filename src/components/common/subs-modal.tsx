import { Input } from 'antd';
import clsx from 'clsx';
import { ChangeEvent } from 'react';

/**
 * Subscription form
 *
 * used for subscription adder and changer
 *
 * when use onChange to change fields
 * onChange method need to return two methods,
 * to change name and url.
 *
 * ```ts
 * const handleSubs = (type: 'name' | 'url') => {
 *   const map = {
 *     name: (e: ChangeEvent<HTMLInputElement>) => {
 *       const value = e.target.value.trim();
 *       setSubscripiton((d) => ({ ...d, name: value }));
 *     },
 *     url: (e: ChangeEvent<HTMLInputElement>) => {
 *       const value = e.target.value.trim();
 *       const valid = URL_VALID.test(value);
 *       setStatus(!subscripition ? '' : valid ? '' : 'error');
 *       setSubscripiton((d) => ({ ...d, url: value }));
 *     },
 *   };
 *   return map[type];
 * };
 * ```
 */
const SubsModal = ({
  subs,
  status,
  loading,
  onChange,
}: {
  subs: {
    name: string;
    url: string;
  };
  status?: '' | 'error';
  loading?: boolean;
  onChange?: (
    type: 'name' | 'url',
  ) => (e: ChangeEvent<HTMLInputElement>) => void;
}) => {
  return (
    <div className="flex">
      <div
        className={clsx(
          'grid grid-cols-[3em_1fr]',
          'items-center gap-4',
          'py-4',
        )}
      >
        <div>Name: </div>
        <Input
          value={subs.name}
          onChange={onChange?.('name')}
          allowClear
          placeholder="Unnamed"
          disabled={loading}
        />
        <div>URL: </div>
        <Input
          value={subs.url}
          onChange={onChange?.('url')}
          allowClear
          placeholder="Subscription url"
          status={status}
          disabled={loading}
        />
      </div>
    </div>
  );
};

export default SubsModal;
