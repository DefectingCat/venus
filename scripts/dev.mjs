import fs from 'fs/promises';
import fsSync from 'fs';
import path from 'path';

const rootPath = process.cwd();

/**
 * @args -p production mode
 */
const args = process.argv.slice(2);
const production = args.includes('-p');

export async function devMain(mona, publicPath, andMap = false) {
  await fs.cp(
    `${mona}/editor/editor.main.js${andMap ? '.map' : ''}`,
    `${publicPath}/editor/editor.main.js${andMap ? '.map' : ''}`,
    {
      recursive: true,
    },
  );
  await fs.cp(
    `${mona}/editor/editor.main.nls.js${andMap ? '.map' : ''}`,
    `${publicPath}/editor/editor.main.nls.js${andMap ? '.map' : ''}`,
    {
      recursive: true,
    },
  );
  await fs.cp(`${mona}/base/`, `${publicPath}/base/`, {
    recursive: true,
  });
  await fs.cp(
    `${mona}/loader.js${andMap ? '.map' : ''}`,
    `${publicPath}/loader.js${andMap ? '.map' : ''}`,
    {
      recursive: true,
    },
  );
  if (!andMap) {
    await fs.cp(`${mona}/language/json`, `${publicPath}/language/json`, {
      recursive: true,
    });
    await fs.cp(
      `${mona}/editor/editor.main.css`,
      `${publicPath}/editor/editor.main.css`,
      {
        recursive: true,
      },
    );
  }
}

if (production) {
  const publicMap = path.resolve(rootPath, './public/min-maps/');
  if (fsSync.existsSync(publicMap)) {
    await fs.rm(publicMap, {
      recursive: true,
    });
  }
  const mona = path.resolve(rootPath, './node_modules/monaco-editor/min/vs');
  const publicFolder = path.resolve(rootPath, './public/vs');
  await devMain(mona, publicFolder, false);
} else {
  const monaMap = path.resolve(
    rootPath,
    './node_modules/monaco-editor/min-maps/vs',
  );
  const publicMap = path.resolve(rootPath, './public/min-maps/vs');
  await devMain(monaMap, publicMap, true);
  const mona = path.resolve(rootPath, './node_modules/monaco-editor/min/vs');
  const publicFolder = path.resolve(rootPath, './public/vs');
  await devMain(mona, publicFolder, false);
}
