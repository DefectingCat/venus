import fs from 'fs/promises';
import path from 'path';

const rootPath = process.cwd();
const mona = path.resolve(rootPath, './node_modules/monaco-editor/min/vs');
const publicFolder = path.resolve(rootPath, './public/vs');
const targetFolder = path.resolve(rootPath, './scr-tauri/target/release');

async function main() {
  await fs.rm(publicFolder, { recursive: true, force: true });
  await fs.rm(targetFolder, { recursive: true, force: true });
  await fs.cp(mona, publicFolder, { recursive: true });
}

main();
