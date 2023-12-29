import fs from 'fs/promises';
import path from 'path';

const rootPath = process.cwd();
const mona = path.resolve(rootPath, './node_modules/monaco-editor/min/vs');
const publicFolder = path.resolve(rootPath, './public/vs');

async function main() {
  await fs.cp(mona, publicFolder, { recursive: true });
}

main();
