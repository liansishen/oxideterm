'use strict';

const SETUP_NAME = (version) => `OxideTerm_${version}_windows_x64-setup.exe`;
const PORTABLE_NAME = (version) => `OxideTerm_${version}_windows_x64_portable.zip`;

async function readSignedAsset(signatureAsset, token, fetchImpl) {
  const response = await fetchImpl(signatureAsset.url, {
    headers: {
      Authorization: `Bearer ${token}`,
      Accept: 'application/octet-stream',
    },
  });
  if (!response.ok) {
    throw new Error(`failed to download ${signatureAsset.name}: HTTP ${response.status}`);
  }
  return Buffer.from((await response.text()).trim(), 'utf8').toString('base64');
}

async function buildForkReleasePlatforms({ version, release, token, fetchImpl }) {
  if (!version.match(/^\d+\.\d+\.\d+\+fork\.[1-9]\d*$/)) {
    throw new Error(`invalid fork release version: ${version}`);
  }
  const assets = new Map(release.assets.map((asset) => [asset.name, asset]));
  const platforms = {};
  async function add(keys, filename) {
    const binary = assets.get(filename);
    const signature = assets.get(`${filename}.sig`);
    if (!binary || !signature) throw new Error(`missing signed release asset: ${filename}`);
    const encodedSignature = await readSignedAsset(signature, token, fetchImpl);
    for (const key of keys) platforms[key] = { signature: encodedSignature, url: binary.browser_download_url };
  }
  await add(['windows-x86_64', 'windows-x86_64-nsis', 'x86_64-pc-windows-msvc', 'x86_64-pc-windows-msvc-nsis'], SETUP_NAME(version));
  await add(['windows-x86_64-portable'], PORTABLE_NAME(version));
  return { version, platforms };
}

module.exports = { buildForkReleasePlatforms };
