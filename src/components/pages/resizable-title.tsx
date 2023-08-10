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
}) => {
  if (!width) {
    return <th {...rest} />;
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
      <th {...rest} />
    </Resizable>
  );
};

export default ResizableTitle;
