'use strict';

const test = require('node:test');
const assert = require('node:assert/strict');
const { buildForkReleasePlatforms } = require('../fork_release_manifest.cjs');

function fixture(version = '2.1.0+fork.1') {
  const setup = `OxideTerm_${version}_windows_x64-setup.exe`;
  const portable = `OxideTerm_${version}_windows_x64_portable.zip`;
  const assets = [setup, portable].flatMap((name) => [
    { name, browser_download_url: `https://github.com/fork/oxideterm/releases/download/v${version}/${name}` },
    { name: `${name}.sig`, url: `https://api.github.com/assets/${name}.sig` },
  ]);
  return { assets, setup, portable };
}

test('fork manifest uses versioned Windows assets and authenticated signature downloads', async () => {
  const token = 'test-token';
  const data = fixture();
  const requests = [];
  const result = await buildForkReleasePlatforms({
    version: '2.1.0+fork.1',
    release: { assets: data.assets },
    token,
    fetchImpl: async (url, options) => {
      requests.push({ url, options });
      return { ok: true, status: 200, text: async () => `signature:${url}` };
    },
  });

  assert.deepEqual(requests.map(({ url }) => url), [
    `https://api.github.com/assets/${data.setup}.sig`,
    `https://api.github.com/assets/${data.portable}.sig`,
  ]);
  for (const { options } of requests) {
    assert.deepEqual(options.headers, {
      Authorization: `Bearer ${token}`,
      Accept: 'application/octet-stream',
    });
  }
  assert.equal(result.version, '2.1.0+fork.1');
  assert.deepEqual(result.platforms['windows-x86_64'], {
    signature: Buffer.from(`signature:https://api.github.com/assets/${data.setup}.sig`).toString('base64'),
    url: `https://github.com/fork/oxideterm/releases/download/v2.1.0+fork.1/${data.setup}`,
  });
  assert.equal(result.platforms['windows-x86_64-nsis'].url, result.platforms['windows-x86_64'].url);
  assert.equal(result.platforms['windows-x86_64-portable'].url,
    `https://github.com/fork/oxideterm/releases/download/v2.1.0+fork.1/${data.portable}`);
});

test('manifest generation rejects failed authenticated asset downloads', async () => {
  const data = fixture();
  await assert.rejects(buildForkReleasePlatforms({
    version: '2.1.0+fork.1',
    release: { assets: data.assets },
    token: 'test-token',
    fetchImpl: async () => ({ ok: false, status: 403, text: async () => '' }),
  }), /HTTP 403/);
});
