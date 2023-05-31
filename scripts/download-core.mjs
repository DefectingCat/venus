import { error, log } from 'console';
import * as url from 'url';
import fs from 'fs';
import { mkdir, rm } from 'fs/promises';
import path from 'path';
import { Readable, Transform } from 'stream';
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
  const fileSize = res.headers.get('content-length');
  log('File size: ', fileSize);

  if (fs.existsSync('downloads')) {
    await rm('downloads', { recursive: true, force: true });
  }
  await mkdir('downloads');

  const destination = path.resolve('./downloads', filename);
  const fileStream = fs.createWriteStream(destination, { flags: 'wx' });

  let totalBytes = 0;
  await finished(
    Readable.fromWeb(res.body)
      .pipe(
        new Transform({
          transform(chunk, _encoding, callback) {
            totalBytes += chunk.length;
            const precent = ((100 * totalBytes) / fileSize).toFixed(2);
            log(precent);
            this.push(chunk);
            callback();
          },
        })
      )
      .pipe(fileStream)
  );
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
