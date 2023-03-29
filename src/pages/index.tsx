import { Button } from 'antd';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';

function App() {
  return (
    <MainLayout>
      <div className="mt-1 mb-4">
        <Title>Proxies</Title>
      </div>

      <div>
        <Title.h2>Subscription</Title.h2>
        <div>
          <Button>Add</Button>
        </div>
      </div>
    </MainLayout>
  );
}

export default App;
