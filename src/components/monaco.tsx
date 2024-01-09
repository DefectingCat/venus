import Editor, { EditorProps, loader } from '@monaco-editor/react';
import { useBoolean, useMount } from 'ahooks';
import clsx from 'clsx';
import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
import { useTheme } from 'next-themes';
import { useEffect, useRef, useState } from 'react';

type MonacoProps = {
  onFocus?: () => void;
  onBlur?: () => void;
  wrapperClass?: string;
} & EditorProps;

loader.config({
  paths: {
    vs: '/vs',
  },
  'vs/nls': {
    availableLanguages: {
      '*': '',
    },
  },
});

const vsThemeMap = {
  light: 'light',
  dark: 'vs-dark',
};

const Monaco = (props: MonacoProps) => {
  const wrapper = useRef<HTMLDivElement>(null);
  const [editor, setEditor] =
    useState<monaco.editor.IStandaloneCodeEditor>(null);

  // Init events
  useEffect(() => {
    if (!editor) return;
    props?.onFocus && editor.onDidFocusEditorWidget(props?.onFocus);
    props?.onBlur && editor.onDidBlurEditorWidget(props?.onBlur);
  }, [editor, props]);

  const { options, ...rest } = props;

  // adapt theme
  const [mounted, setMounted] = useBoolean(false);
  useMount(setMounted.setTrue);
  const { theme, systemTheme } = useTheme();
  const currentTheme = theme === 'system' ? systemTheme : theme;

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
    <div
      ref={wrapper}
      className={clsx('h-full rounded-lg overflow-hidden', props.wrapperClass)}
    >
      <Editor
        onMount={(editor) => {
          setTimeout(() => {
            editor.getAction('editor.action.formatDocument').run();
          }, 0);
          setEditor(editor);
        }}
        height="20vh"
        theme={mounted ? vsThemeMap[currentTheme] : 'light'}
        options={{
          minimap: {
            enabled: false,
          },
          lineNumbers: 'off',
          glyphMargin: false,
          folding: false,
          lineDecorationsWidth: 0,
          lineNumbersMinChars: 0,
          ...options,
        }}
        {...rest}
      />
    </div>
  );
};

export default Monaco;
