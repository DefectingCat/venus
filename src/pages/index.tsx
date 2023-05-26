import { useBoolean } from 'ahooks';
import { Button } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';
import dynamic from 'next/dynamic';
import useStore from 'store';

const SubscriptionAdder = dynamic(
  () => import('components/pages/subscription-adder')
);

function App() {
  const [open, setOpen] = useBoolean(false);
  const { nodes } = useStore();

  return (
    <>
      <MainLayout>
        <div className="mt-1 mb-4">
          <Title>Proxies</Title>
        </div>

        <div>
          <Title.h2>Subscription</Title.h2>
          <div className="flex items-center">
            <Button className="mr-2" onClick={setOpen.setTrue}>
              Add
            </Button>
            <Button>Update All</Button>
          </div>
          {/* <div> */}
          {/*   {nodes.map((node) => ( */}
          {/*     <div key={node.id}>{node.id}</div> */}
          {/*   ))} */}
          {/* </div> */}
        </div>
      </MainLayout>

      {open && <SubscriptionAdder onCancel={setOpen.setFalse} />}
    </>
  );
}

export default App;
