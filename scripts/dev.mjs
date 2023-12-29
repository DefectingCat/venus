import fs from 'fs/promises';
import path from 'path';

const rootPath = process.cwd();
const mona = path.resolve(rootPath, './node_modules/monaco-editor/min/vs');
const publicFolder = path.resolve(rootPath, './public/vs');

export async function devMain() {
  await fs.cp(`${mona}/language/json`, `${publicFolder}/language/json`, {
    recursive: true,
  });
  await fs.cp(
    `${mona}/editor/editor.main.js`,
    `${publicFolder}/editor/editor.main.js`,
    {
      recursive: true,
    },
  );
  await fs.cp(
    `${mona}/editor/editor.main.nls.js`,
    `${publicFolder}/editor/editor.main.nls.js`,
    {
      recursive: true,
    },
  );
  await fs.cp(
    `${mona}/editor/editor.main.css`,
    `${publicFolder}/editor/editor.main.css`,
    {
      recursive: true,
    },
  );
  await fs.cp(`${mona}/base/`, `${publicFolder}/base/`, {
    recursive: true,
  });
  await fs.cp(`${mona}/loader.js`, `${publicFolder}/loader.js`, {
    recursive: true,
  });
}

devMain();
