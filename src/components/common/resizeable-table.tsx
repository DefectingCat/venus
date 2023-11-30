import { TableProps } from 'antd';
import Table, { ColumnsType } from 'antd/es/table';
import dynamic from 'next/dynamic';
import { memo, useMemo, useState } from 'react';
import { ResizeCallbackData } from 'react-resizable';
import styles from 'styles/index.module.scss';

const ResizableTitle = dynamic(
  () => import('components/pages/resizable-title'),
);

type ResizableTable<T> = TableProps<T>;

const ResizableTable = <T,>({ columns, ...rest }: ResizableTable<T>) => {
  const [colWidth, setColWidth] = useState(
    columns.map((d) => ({
      key: d.key,
      width: d.width,
    })),
  );
  const cols = useMemo(
    () =>
      columns.map((d) => ({
        ...d,
        width: colWidth.find((c) => c.key === d.key)?.width,
      })),
    [colWidth, columns],
  );
  const handleResize =
    (index: number) =>
    (_: React.SyntheticEvent<Element>, { size }: ResizeCallbackData) => {
      const newWidth = [...colWidth];
      newWidth[index] = {
        ...newWidth[index],
        width: size.width,
      };
      setColWidth(newWidth);
    };
  const mergeColumns: ColumnsType<T> = cols.map((col, index) => ({
    ...col,
    onHeaderCell: (column: ColumnsType<T>[number]) => ({
      width: column.width,
      onResize: handleResize(index) as React.ReactEventHandler<unknown>,
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
