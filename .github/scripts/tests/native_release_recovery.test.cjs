const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const test = require('node:test');
const vm = require('node:vm');

const workflow = fs.readFileSync(path.join(__dirname, '../../workflows/native-package.yml'), 'utf8');
function scriptFor(name) {
  const step = workflow.split(`      - name: ${name}\n`)[1].split('\n      - name: ')[0];
  const script = step.split('          script: |\n')[1];
  const lines = script.split('\n');
  const end = lines.findIndex((line) => line.trim() && !line.startsWith('            '));
  return lines.slice(0, end < 0 ? undefined : end).map((line) => line.slice(12)).join('\n');
}
function execute(name, github, env, requireFn = require) {
  return vm.runInNewContext(`(async () => {${scriptFor(name)}\n})()`, {
    github, process: { env }, require: requireFn,
    context: { repo: { owner: 'fork', repo: 'oxideterm' }, runId: 20 },
  });
}

test('package recovery requires the exact release commit and a successful Windows artifact', async () => {
  const actions = {
    getWorkflowRun: async () => ({ data: run }),
    listJobsForWorkflowRun: 'jobs', listWorkflowRunArtifacts: 'artifacts',
  };
  const run = { head_sha: 'release-commit', path: '.github/workflows/native-package.yml', event: 'workflow_dispatch', status: 'completed' };
  let jobs = [{ name: 'Package windows-x64', conclusion: 'success' }];
  let artifacts = [{ name: 'OxideTerm-windows-x64', expired: false }];
  const github = { rest: { actions }, paginate: async (api) => api === 'jobs' ? jobs : artifacts };
  const env = { SOURCE_RUN_ID: '19', RELEASE_COMMIT: 'release-commit', FORK_RELEASE: 'true', UPLOAD_RELEASE: 'true' };
  await execute('Validate reused package run', github, env);
  run.head_sha = 'other-commit';
  await assert.rejects(execute('Validate reused package run', github, env), /exact release commit/);
  run.head_sha = 'release-commit';
  jobs = [{ name: 'Package windows-x64', conclusion: 'failure' }];
  await assert.rejects(execute('Validate reused package run', github, env), /successful Windows package/);
  jobs[0].conclusion = 'success';
  artifacts[0].expired = true;
  await assert.rejects(execute('Validate reused package run', github, env), /available Windows package artifact/);
});

test('fork draft upload finds draft by listing and replaces only draft assets', async () => {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'oxideterm-release-test-'));
  try {
    fs.mkdirSync(path.join(directory, 'dist-release'));
    fs.writeFileSync(path.join(directory, 'dist-release', 'setup.exe'), 'verified-package');
    fs.writeFileSync(path.join(directory, 'notes.md'), 'Release description');
    const relativeFs = {
      readdirSync: (name) => fs.readdirSync(path.join(directory, name)),
      readFileSync: (name, options) => fs.readFileSync(path.join(directory, name), options),
    };
    const release = { id: 4, tag_name: 'v2.1.0+fork.1', draft: true, assets: [{ id: 7, name: 'setup.exe' }] };
    const calls = [];
    const repos = {
      listReleases: 'list',
      updateRelease: async (args) => calls.push(['update', args]),
      deleteReleaseAsset: async (args) => calls.push(['delete', args]),
      uploadReleaseAsset: async (args) => calls.push(['upload', args]),
      createRelease: async (args) => { calls.push(['create', args]); return { data: { ...release, assets: [] } }; },
    };
    let releases = [release];
    const github = { rest: { repos }, paginate: async () => releases };
    const env = { RELEASE_TAG: release.tag_name, RELEASE_VERSION: '2.1.0+fork.1', RELEASE_PRERELEASE: 'false', RELEASE_BODY_PATH: 'notes.md' };
    const requireFn = (name) => name === 'node:fs' ? relativeFs : require(name);
    await execute('Upload fork draft release assets', github, env, requireFn);
    assert.deepEqual(calls.map(([action]) => action), ['update', 'delete', 'upload']);
    assert.equal(calls[0][1].tag_name, 'v2.1.0+fork.1');
    assert.equal(calls[1][1].asset_id, 7);
    assert.equal(calls[2][1].release_id, 4);
    assert.equal(calls[2][1].data.toString(), 'verified-package');
    release.draft = false;
    calls.length = 0;
    await assert.rejects(execute('Upload fork draft release assets', github, env, requireFn), /Refusing to modify published/);
    assert.deepEqual(calls, []);
    releases = [];
    await execute('Upload fork draft release assets', github, env, requireFn);
    assert.equal(calls[0][0], 'create');
    assert.equal(calls[0][1].tag_name, 'v2.1.0+fork.1');
    assert.equal(calls[0][1].draft, true);
    assert.equal(calls[0][1].body, 'Release description');
    assert.equal(Object.hasOwn(calls[0][1], 'target_commitish'), false);
  } finally {
    fs.rmSync(directory, { recursive: true });
  }
});
