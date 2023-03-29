import clsx from 'clsx';
import { ReactNode } from 'react';

const Title = ({ children }: { children: ReactNode }) => {
  return <h1 className={clsx('text-4xl')}>{children}</h1>;
};

Title.h1 = ({ children }: { children: ReactNode }) => {
  return <h1 className={clsx('text-4xl')}>{children}</h1>;
};

Title.h2 = ({ children }: { children: ReactNode }) => {
  return <h2 className={clsx('text-xl text-gray-700')}>{children}</h2>;
};

export default Title;
