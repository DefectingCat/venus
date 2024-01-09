import { Button } from 'antd';
import clsx from 'clsx';
import Monaco from 'components/monaco';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';

/**
 * Config file editor
 */
const Editor = () => {
  return (
    <MainLayout>
      <div className="flex flex-col h-full">
        <div className={clsx('mt-1 ')}>
          <Title>Settings</Title>
        </div>
        <Monaco wrapperClass="flex" height="100%" value="123" language="json" />
        <div className="mt-2">
          <Button className="mr-2">Save</Button>
          <Button>Rest</Button>
        </div>
      </div>
    </MainLayout>
  );
};

export default Editor;
