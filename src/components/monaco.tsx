import Editor, { EditorProps } from '@monaco-editor/react';
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useEffect, useRef, useState } from 'react';

type MonacoProps = {
  onFocus?: () => void;
  onBlur?: () => void;
} & EditorProps;

const Monaco = (props: MonacoProps) => {
  const wrapper = useRef<HTMLDivElement>(null);
  const [editor, setEditor] =
    useState<monaco.editor.IStandaloneCodeEditor>(null);

  // Init events
  useEffect(() => {
    if (!editor) return;
    editor.onDidFocusEditorWidget(props?.onFocus);
    editor.onDidBlurEditorWidget(props?.onBlur);
  }, [editor, props]);

  const { options, ...rest } = props;

  /* useEffect(() => {
    if (!editor || !wrapper.current) return;
    const resetLayout = () => {
      editor.layout({ width: 0, height: 0 });
      window.requestAnimationFrame(() => {
        const rect = wrapper.current.getBoundingClientRect();
        editor.layout({ width: rect.width, height: rect.height });
      });
    };
    const debounced = debounce(resetLayout, 300);
    window.addEventListener('resize', debounced);

    return () => {
      window.removeEventListener('resize', resetLayout);
    };
  }, [editor]); */

  return (
    <div ref={wrapper}>
      <Editor
        onMount={(monaco) => setEditor(monaco)}
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
          ...options,
        }}
        {...rest}
      />
    </div>
  );
};

export default Monaco;
