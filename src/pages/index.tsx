import clsx from 'clsx';
import { useTheme } from 'next-themes';
import { useMemo } from 'react';

function App() {
  const { systemTheme, theme, setTheme } = useTheme();
  const currentTheme = theme === 'system' ? systemTheme : theme;

  return (
    <>
      <div className="text-5xl">Hello world!</div>
      <div>Current theme: {theme}</div>
      <button className={clsx('rounded p-4')}>
        {currentTheme === 'dark' ? (
          <>
            <span onClick={() => setTheme('light')}>Light</span>
          </>
        ) : (
          <>
            <span onClick={() => setTheme('dark')}>Dark</span>
          </>
        )}
      </button>
    </>
  );
}

export default App;
