import clsx from 'clsx';
import NaviItem from './navi-item';
import { useRouter } from 'next/router';
import useStore from 'store';

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
];

export default function SideBar() {
  const router = useRouter();
  const { coreStatus } = useStore();

  const handleRoute = (item: SingleNavi) => {
    router.push(item.path);
  };

  return (
    <nav
      className={clsx(
        'w-56 max-w-xs flex',
        'py-6 px-5',
        'bg-gray-100 flex-col',
        'dark:bg-rua-gray-800'
      )}
    >
      {/* logo */}
      <div className={clsx('flex')}>Logo</div>

      {/* navi */}
      <div className={clsx('flex flex-col justify-between', 'felx-1 h-full')}>
        <div className="my-4">
          {navi.map((n) => (
            <NaviItem
              key={n.id}
              onClick={() => handleRoute(n)}
              className="mb-2 w-full"
              active={router.pathname === n.path}
            >
              {n.name}
            </NaviItem>
          ))}
        </div>

        {/* core status */}
        <div>{coreStatus}</div>
      </div>
    </nav>
  );
}