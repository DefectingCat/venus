#!/usr/bin/env node
import { error, log } from 'console';
import { execa } from 'execa';
import fs from 'fs';
import { mkdir, rm } from 'fs/promises';
import path from 'path';
import { Readable, Transform } from 'stream';
import { finished } from 'stream/promises';
import kit from 'terminal-kit';
import * as url from 'url';
import { reanmeFile } from './rename-sidecar.mjs';

const { terminal } = kit;

const rootPath = process.cwd();
const downloadFolder = path.resolve(rootPath, './downloads');
const coreFolder = path.resolve(rootPath, './src-tauri/binaries/core/');
const resourcesFolder = path.resolve(rootPath, './src-tauri/resources');

const unixCommand = async (filename) => {
  log(`Start extract ${filename}`);
  log(
    (
      await execa('unzip', [
        `${downloadFolder}/${filename}`,
        '-d',
        downloadFolder
      ])
    ).stdout
  );
  log('Start copy file');

  if (!fs.existsSync(coreFolder)) {
    await mkdir(coreFolder, { recursive: true });
  }
  log(
    'Copy v2ary',
    (await execa('cp', [`${downloadFolder}/${binName}`, coreFolder])).stdout
  );
  log(
    'Copy geoip-only-cn-private.dat',
    (
      await execa('cp', [
        `${downloadFolder}/geoip-only-cn-private.dat`,
        resourcesFolder
      ])
    ).stdout
  );
  log(
    'Copy geosite.dat',
    (await execa('cp', [`${downloadFolder}/geosite.dat`, resourcesFolder]))
      .stdout
  );
  log(
    'Copy geoip.dat',
    (await execa('cp', [`${downloadFolder}/geoip.dat`, resourcesFolder]))
      .stdout
  );
};
const winCommand = async (filename) => {
  log(`Start extract ${filename}`);
  // powershell -command "Expand-Archive -Force ./v2ray-windows-64.zip ./v2ray"
  // log(
  //   await execa('powershell', [
  //     '-command',
  //     `"Expand-Archive -Force downloads/${filename} downloads/"`,
  //   ]).stdout
  // );
  log(
    await execa('powershell -command Expand-Archive', [
      `-Force -LiteralPath downloads/${filename}`,
      `-DestinationPath downloads/`
    ])
  );
  log('Start copy file');

  if (!fs.existsSync(coreFolder)) {
    await mkdir(coreFolder, { recursive: true });
  }
  log(
    'Copy v2ary',
    (await execa('powershell -command cp', ['downloads/v2ray.exe', coreFolder]))
      .stdout
  );
  log(
    'Copy geoip-only-cn-private.dat',
    (
      await execa('powershell -command cp', [
        'downloads/geoip-only-cn-private.dat',
        resourcesFolder
      ])
    ).stdout
  );
  log(
    'Copy geosite.dat',
    (
      await execa('powershell -command cp', [
        'downloads/geosite.dat',
        resourcesFolder
      ])
    ).stdout
  );
  log(
    'Copy geoip.dat',
    (
      await execa('powershell -command cp', [
        'downloads/geoip.dat',
        resourcesFolder
      ])
    ).stdout
  );
};

const platformMap = {
  darwin: 'macos',
  win32: 'windows',
  linux: 'linux'
};
const archMap = {
  arm64: 'arm64-v8a',
  x64: '64'
};
const platformCommand = {
  darwin: unixCommand,
  win32: winCommand,
  linux: unixCommand
};

const __filename = url.fileURLToPath(import.meta.url);
const __dirname = url.fileURLToPath(new URL('.', import.meta.url));
log(__filename, __dirname);

async function downloadFile(url, filename = '.') {
  const res = await fetch(url);
  const fileSize = res.headers.get('content-length');
  log('File size: ', fileSize);

  if (fs.existsSync(downloadFolder)) {
    await rm(downloadFolder, { recursive: true, force: true });
  }
  await mkdir(downloadFolder);

  const destination = path.resolve(downloadFolder, filename);
  const fileStream = fs.createWriteStream(destination, { flags: 'wx' });

  const progressBar = terminal.progressBar({
    width: 80,
    title: filename,
    eta: true,
    precent: true
  });
  let totalBytes = 0;
  await finished(
    Readable.fromWeb(res.body)
      .pipe(
        new Transform({
          transform(chunk, _encoding, callback) {
            totalBytes += chunk.length;
            const precent = ((100 * totalBytes) / fileSize).toFixed(2);
            progressBar.update(Number(precent));
            this.push(chunk);
            callback();
          }
        })
      )
      .pipe(fileStream)
  );
}

async function downloadWithWget(url, filename) {
  if (fs.existsSync(downloadFolder)) {
    await rm(downloadFolder, { recursive: true, force: true });
  }
  await mkdir(downloadFolder);
  const { stdout } = await execa(`wget ${url} ${downloadFolder}/${filename}`);
  log(stdout);
}

async function downloadCore(manual, manualPlat, skipDownload) {
  const { arch, platform } = process;
  log('Current platform: ', platform, 'current arch: ', arch);
  const targetName = `v2ray-${manual ? manualPlat : platformMap[platform]}-${
    archMap[arch]
  }.zip`;
  log('Target file: ', targetName);

  try {
    if (!skipDownload) {
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
      log('Start downloading: ', url, 'to', downloadFolder);
      await downloadFile(url, targetName);
      log(`Download ${targetName} sucess`);
    }

    const command = platformCommand[process.platform];
    if (!command) {
      throw new Error(
        `Cannot found target platform command ${process.platform}`
      );
    }
    await command(targetName);
    await (manual ? reanmeFile(renameExt, targetTriple) : reanmeFile());
  } catch (err) {
    error(err);
  }
}

let binName = 'v2ray';
let renameExt = '';
let targetTriple = '';

async function main() {
  const args = process.argv.slice(2);

  const manual = args.includes('-m');
  const skipDownload = args.includes('--skip-download');

  terminal.clear();
  if (manual) {
    log('Select platform');
    terminal.singleColumnMenu(
      ['macos', 'windows', 'linux'],
      {},
      async (err, res) => {
        switch (res.selectedText) {
          case 'macos':
            binName = 'v2ray';
            break;
          case 'linux':
            binName = 'v2ray';
            break;
          case 'windows':
            binName = 'v2ray.exe';
            renameExt = '.exe';
            targetTriple = 'x86_64-pc-windows-msvc';
            break;
        }
        await downloadCore(manual, res.selectedText, skipDownload);
        process.exit();
      }
    );
  } else {
    await downloadCore(false, null, skipDownload);
  }
}

main();
