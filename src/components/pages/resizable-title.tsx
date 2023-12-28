import { Tooltip } from 'antd';
import { DetailedHTMLProps, ThHTMLAttributes } from 'react';
import { Resizable, ResizeCallbackData } from 'react-resizable';

const ResizableTitle = ({
  onResize,
  width,
  ...rest
}: {
  onResize: (
    e: React.SyntheticEvent<Element>,
    data: ResizeCallbackData,
  ) => void;
  width: number;
} & DetailedHTMLProps<
  ThHTMLAttributes<HTMLTableHeaderCellElement>,
  HTMLTableHeaderCellElement
>) => {
  const { children, ...props } = rest;

  const Th = (
    <th {...props}>
      <Tooltip title={children}>{children}</Tooltip>
    </th>
  );

  if (!width) {
    return Th;
  }

  return (
    <Resizable
      width={width}
      height={0}
      handle={
        <span
          className="react-resizable-handle"
          onClick={(e) => {
            e.stopPropagation();
          }}
        />
      }
      onResize={onResize}
      draggableOpts={{ enableUserSelectHack: false }}
    >
      {Th}
    </Resizable>
  );
};

export default ResizableTitle;
