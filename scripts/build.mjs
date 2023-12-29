import fs from 'fs/promises';
import path from 'path';

const rootPath = process.cwd();
const publicFolder = path.resolve(rootPath, './public/vs');
const targetFolder = path.resolve(rootPath, './scr-tauri/target/release');

async function main() {
  await fs.rm(publicFolder, { recursive: true, force: true });
  await fs.rm(targetFolder, { recursive: true, force: true });
}

main();
