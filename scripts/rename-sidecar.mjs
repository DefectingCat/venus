import { execa } from 'execa';
import fs from 'fs';

export async function reanmeFile(
  extension = process.platform === 'win32' ? '.exe' : '',
  targetTriple
) {
  const rustInfo = (await execa('rustc', ['-vV'])).stdout;
  if (!targetTriple) targetTriple = /host: (\S+)/g.exec(rustInfo)[1];
  if (!targetTriple) {
    console.error('Failed to determine platform target triple');
  }
  const file = `src-tauri/binaries/core/v2ray${extension}`;
  const tripleFile = `src-tauri/binaries/core/v2ray-${targetTriple}${extension}`;
  fs.renameSync(file, tripleFile);
  console.log(`Sucess rename binaries ${file} ${tripleFile}`);
}

// main().catch((e) => {
//   error(e);
// });
