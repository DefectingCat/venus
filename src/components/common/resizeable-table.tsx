import { TableProps } from 'antd';
import Table, { ColumnsType } from 'antd/es/table';
import dynamic from 'next/dynamic';
import {
  memo,
  ReactEventHandler,
  SyntheticEvent,
  useMemo,
  useState,
} from 'react';
import { ResizeCallbackData } from 'react-resizable';
import styles from 'styles/index.module.scss';
import { AnyObject } from 'antd/es/_util/type';

const ResizableTitle = dynamic(
  () => import('components/pages/resizable-title'),
);

type ResizableTable<T extends AnyObject> = TableProps<T>;

const ResizableTable = <T extends AnyObject>({
  columns,
  ...rest
}: ResizableTable<T>) => {
  const [colWidth, setColWidth] = useState(
    columns?.map((d) => ({
      key: d.key,
      width: d.width,
    })),
  );
  const cols = useMemo(
    () =>
      columns?.map((d) => ({
        ...d,
        width: colWidth?.find((c) => c.key === d.key)?.width,
      })),
    [colWidth, columns],
  );
  const handleResize =
    (index: number) =>
    (_: SyntheticEvent, { size }: ResizeCallbackData) => {
      if (!colWidth) return;
      const newWidth = [...colWidth];
      newWidth[index] = {
        ...newWidth[index],
        width: size.width,
      };
      setColWidth(newWidth);
    };

  if (!cols) return null;
  const mergeColumns: ColumnsType<T> = cols.map((col, index) => ({
    ...col,
    onHeaderCell: (column: ColumnsType<T>[number]) => ({
      width: column.width,
      onResize: handleResize(index) as ReactEventHandler<unknown>,
    }),
  }));

  return (
    <>
      <Table
        className={styles.table}
        components={{
          header: {
            cell: ResizableTitle,
          },
        }}
        columns={mergeColumns}
        {...rest}
      />
    </>
  );
};

export default memo(ResizableTable);
