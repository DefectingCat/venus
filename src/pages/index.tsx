import { useBoolean } from 'ahooks';
import { Button, Input, Modal } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';

function App() {
  const [showAddSubs, setShowAddSubs] = useBoolean(false);

  return (
    <>
      <MainLayout>
        <div className="mt-1 mb-4">
          <Title>Proxies</Title>
        </div>

        <div>
          <Title.h2>Subscription</Title.h2>
          <div>
            <Button onClick={setShowAddSubs.toggle}>Add</Button>
          </div>
        </div>
      </MainLayout>

      <Modal
        title="Add subscription"
        open={showAddSubs}
        // onOk={handleOk}
        // confirmLoading={confirmLoading}
        onCancel={setShowAddSubs.setFalse}
      >
        <div className="flex items-center my-8">
          <div className="mr-2">URL</div>
          <Input placeholder="Subscription url" />
        </div>
      </Modal>
    </>
  );
}

export default App;
