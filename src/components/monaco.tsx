import Editor, { EditorProps } from '@monaco-editor/react';
import debounce from 'lodash/debounce';
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useEffect, useRef } from 'react';

const Monaco = (props: EditorProps) => {
  const wrapper = useRef<HTMLDivElement>(null);
  const editor = useRef<monaco.editor.IStandaloneCodeEditor>(null);

  useEffect(() => {
    if (!editor.current || !wrapper.current) return;
    const mona = editor.current;
    const resetLayout = () => {
      mona.layout({ width: 0, height: 0 });
      window.requestAnimationFrame(() => {
        const rect = wrapper.current.getBoundingClientRect();
        mona.layout({ width: rect.width, height: rect.height });
      });
    };
    const debounced = debounce(resetLayout, 300);
    window.addEventListener('resize', debounced);

    return () => {
      window.removeEventListener('resize', resetLayout);
    };
  }, []);

  return (
    <div ref={wrapper}>
      <Editor
        onMount={(monaco) => (editor.current = monaco)}
        height="20vh"
        options={{
          minimap: {
            enabled: false,
          },
          lineNumbers: 'off',
          glyphMargin: false,
          folding: false,
          lineDecorationsWidth: 0,
          lineNumbersMinChars: 0,
          contextmenu: false,
        }}
        {...props}
      />
    </div>
  );
};

export default Monaco;
