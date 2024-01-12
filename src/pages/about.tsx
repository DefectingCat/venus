import clsx from 'clsx';
import Title from 'components/pages/page-title';
import MainLayout from 'layouts/main-layout';

const About = () => {
  return (
    <MainLayout>
      <div className={clsx('mt-1')}>
        <Title>About</Title>
      </div>
    </MainLayout>
  );
};

export default About;
