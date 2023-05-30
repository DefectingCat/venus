import { error, log } from 'console';

const platformMap = {
  darwin: 'macos',
};
const archMap = {
  arm64: 'arm64-v8a',
};

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
  } catch (err) {
    error(err);
  }
}

main();
