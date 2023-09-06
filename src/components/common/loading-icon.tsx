import { LoadingOutlined } from '@ant-design/icons';
import { Spin } from 'antd';

const antIcon = <LoadingOutlined spin />;

const LoadingIcon = () => {
  return (
    <>
      <Spin indicator={antIcon} />
    </>
  );
};

export default LoadingIcon;
