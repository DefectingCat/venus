import { Input, theme } from 'antd';
import clsx from 'clsx';

const { useToken } = theme;

const DrawerItem = ({
  label,
  value,
  onChange,
}: {
  label: string;
  value?: string;
  onChange?: () => void;
}) => {
  const token = useToken();

  return (
    <div className={clsx('relative flex items-center', 'mb-4')}>
      <Input value={value} />
      <div
        className={clsx(
          'absolute left-2 top-[-8px]',
          'text-gray-600 dark:text-gray-400',
          'text-xs',
        )}
        style={{
          background: token.token.colorBgElevated,
        }}
      >
        {label}
      </div>
    </div>
  );
};

export default DrawerItem;
