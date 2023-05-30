import { error, log } from 'console';
import * as url from 'url';
import fs from 'fs';
import { mkdir } from 'fs/promises';
import path from 'path';
import { Readable } from 'stream';
import { finished } from 'stream/promises';

const platformMap = {
  darwin: 'macos',
};
const archMap = {
  arm64: 'arm64-v8a',
};

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
log(__filename, __dirname);

async function downloadFile(url, filename = '.') {
  const res = await fetch(url);
  if (!fs.existsSync('downloads')) await mkdir('downloads');
  const destination = path.resolve('./downloads', filename);
  const fileStream = fs.createWriteStream(destination, { flags: 'wx' });
  await finished(Readable.fromWeb(res.body).pipe(fileStream));
}

async function main() {
  const { arch, platform } = process;
  log('Current platform: ', platform, 'current arch: ', arch);
  const targetName = `v2ray-${platformMap[platform]}-${archMap[arch]}.zip`;
  log('Target file: ', targetName);

  try {
    const result = await (
      await fetch(
        'https://api.github.com/repos/v2fly/v2ray-core/releases?per_page=1&page=1'
      )
    ).json();
    const assets = result[0].assets;
    const url = assets.reduce((prev, asset) => {
      return asset.name === targetName ? asset.browser_download_url : prev;
    }, '');
    if (!url) throw new Error('Cannot find taget url');
    log('Start downloading: ', url);
    await downloadFile(url, targetName);
    log(`Download ${targetName} sucess`);
  } catch (err) {
    error(err);
  }
}

main();
