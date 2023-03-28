import clsx from 'clsx';
import NaviItem from './navi-item';
import { useState } from 'react';

const navi = [
  {
    id: 0,
    name: 'General',
    path: '',
  },
  {
    id: 1,
    name: 'Proxies',
    path: '',
  },
];

export default function SideBar() {
  const [current, setCurrent] = useState(0);

  return (
    <nav
      className={clsx(
        'w-56 max-w-xs flex',
        'py-6 px-5',
        'bg-gray-100 flex-col'
      )}
    >
      {/* logo */}
      <div className={clsx('flex')}>Logo</div>

      {/* navi */}
      <div className="my-4">
        {navi.map((n) => (
          <NaviItem
            className="mb-2"
            key={n.id}
            active={current === n.id}
            onClick={() => setCurrent(n.id)}
          >
            {n.name}
          </NaviItem>
        ))}
      </div>
    </nav>
  );
}
