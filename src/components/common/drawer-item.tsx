import { EditorProps } from '@monaco-editor/react';
import { useBoolean } from 'ahooks';
import { Input, InputProps, Select, SelectProps, theme } from 'antd';
import clsx from 'clsx';
import Monaco from 'components/monaco';
import { ReactNode } from 'react';

const { useToken } = theme;

/**
 * Draw item for input element
 */
const DrawerItem = ({
  label,
  children,
  focused,
  className,
}: {
  label: string;
  children: ReactNode;
  focused?: boolean;
  className?: string;
}) => {
  const token = useToken();

  return (
    <div className={clsx('relative flex items-center', 'mb-4', className)}>
      <div
        className={clsx(
          'outline-gray-300 flex-1',
          'rounded-md outline-0',
          'outline transition-all',
          focused && '!outline-[3px]',
          'border border-gray-300 border-solid',
          'overflow-hidden',
        )}
      >
        {children}
      </div>
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

/**
 * Input
 */
export const DrawerInput = ({
  label,
  value,
  ...rest
}: {
  label: string;
  value?: string;
} & InputProps) => {
  const [focused, setFocused] = useBoolean(false);

  return (
    <DrawerItem label={label} focused={focused}>
      <Input
        bordered={false}
        value={value}
        onFocus={setFocused.setTrue}
        onBlur={setFocused.setFalse}
        {...rest}
      />
    </DrawerItem>
  );
};

export const DrawerInputArea = ({
  label,
  value,
  onChange,
}: {
  label: string;
  value?: string;
  onChange?: () => void;
}) => {
  return (
    <DrawerItem label={label}>
      <Input.TextArea value={value} onChange={onChange} />
    </DrawerItem>
  );
};

export const DrawerMonaco = ({
  label,
  ...rest
}: { label: string } & EditorProps) => {
  const [focused, setFocused] = useBoolean(false);

  return (
    <DrawerItem label={label} focused={focused}>
      <Monaco
        onFocus={setFocused.setTrue}
        onBlur={setFocused.setFalse}
        {...rest}
      />
    </DrawerItem>
  );
};

export const DrawerSelect = ({
  label,
  ...rest
}: { label: string } & SelectProps) => {
  const [focused, setFocused] = useBoolean(false);
  const { onFocus, onBlur, ...props } = rest;

  return (
    <DrawerItem label={label} focused={focused}>
      <Select
        className="block"
        bordered={false}
        onFocus={(e) => {
          setFocused.setTrue();
          onFocus?.(e);
        }}
        onBlur={(e) => {
          setFocused.setFalse();
          onBlur?.(e);
        }}
        {...props}
      />
    </DrawerItem>
  );
};

export default DrawerItem;
