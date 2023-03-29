import { useBoolean } from 'ahooks';
import { Button, Modal } from 'antd';
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
        title="Title"
        open={showAddSubs}
        // onOk={handleOk}
        // confirmLoading={confirmLoading}
        // onCancel={handleCancel}
      >
        <p>test</p>
      </Modal>
    </>
  );
}

export default App;
