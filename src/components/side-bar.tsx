import venusLogo from 'assets/venus.svg';
import clsx from 'clsx';
import Image from 'next/image';
import { useRouter } from 'next/router';
import CoreStatus from './core-status';
import NaviItem from './navi-item';

type SingleNavi = {
  id: number;
  name: string;
  path: string;
};
const navi = [
  {
    id: 0,
    name: 'Proxies',
    path: '/',
  },
  {
    id: 1,
    name: 'Settings',
    path: '/settings',
  },
  {
    id: 2,
    name: 'Logging',
    path: '/logging',
  },
  {
    id: 3,
    name: 'Editor',
    path: '/editor',
  },
  {
    id: 4,
    name: 'About',
    path: '/about',
  },
];

export default function SideBar() {
  const router = useRouter();

  const handleRoute = (item: SingleNavi) => {
    router.push(item.path);
  };

  return (
    <nav
      className={clsx(
        'w-56 max-w-xs flex',
        'py-6 px-5',
        'bg-gray-100 flex-col',
        'dark:bg-rua-gray-800',
      )}
    >
      {/* logo */}
      <div className={clsx('flex w-full justify-center')}>
        <Image
          className={clsx('object-contain w-28 h-28')}
          priority
          alt="Venus"
          src={venusLogo}
        />
      </div>

      {/* navi */}
      <div className={clsx('flex flex-col justify-between', 'felx-1 h-full')}>
        <div className="my-4">
          {navi.map((n) => (
            <NaviItem
              key={n.id}
              onClick={() => handleRoute(n)}
              className="w-full mb-2"
              active={router.pathname === n.path}
            >
              {n.name}
            </NaviItem>
          ))}
        </div>

        {/* core status */}
        <CoreStatus />
      </div>
    </nav>
  );
}
