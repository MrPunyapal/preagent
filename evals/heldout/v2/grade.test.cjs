'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const {safePath, configFor, snapshot, injectAcceptance, cargoTestTarget, validateCandidate} = require('./grade.cjs');

function sandbox(t) {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'preagent-grade-test-'));
  t.after(() => fs.rmSync(dir, {recursive: true, force: true}));
  return dir;
}

test('v2 path and control gates reject unsafe or unverified input', () => {
  for (const input of ['../file', '/etc/passwd', 'C:/windows', 'a/../b', 'a//b', 'a\\b', '', null]) {
    assert.equal(safePath(input), false, String(input));
  }
  assert.equal(safePath('crates/regex-syntax/src/ast.rs'), true);
  assert.throws(() => configFor('BAD TASK', {cases: []}, {repositories: []}), /invalid task ID/);
  assert.throws(() => configFor('unfinished', {
    cases: [{id: 'unfinished', controls: {clean: null, seeded: null}}]
  }, {repositories: []}), /not verified/);
});

test('snapshot is bounded and excludes Git internals and old build output', t => {
  const root = sandbox(t);
  const input = path.join(root, 'in');
  const output = path.join(root, 'out');
  fs.mkdirSync(input);
  fs.mkdirSync(output);
  for (const folder of ['.git', 'target', 'src']) fs.mkdirSync(path.join(input, folder));
  fs.writeFileSync(path.join(input, '.git', 'config'), 'secret');
  fs.writeFileSync(path.join(input, 'target', 'binary'), 'large build file');
  fs.writeFileSync(path.join(input, 'Cargo.lock'), 'untrusted');
  fs.writeFileSync(path.join(input, 'src', 'lib.rs'), 'fn main() {}');
  const totals = snapshot(input, output);
  assert.equal(totals.files, 1);
  assert.equal(fs.readFileSync(path.join(output, 'src', 'lib.rs'), 'utf8'), 'fn main() {}');
  assert.equal(fs.existsSync(path.join(output, '.git')), false);
  assert.equal(fs.existsSync(path.join(output, 'target')), false);
  assert.equal(fs.existsSync(path.join(output, 'Cargo.lock')), false);
});

test('snapshot rejects symlinks instead of following files outside candidate', t => {
  const root = sandbox(t);
  const input = path.join(root, 'in');
  const output = path.join(root, 'out');
  fs.mkdirSync(input);
  fs.mkdirSync(output);
  fs.writeFileSync(path.join(root, 'private'), 'sensitive');
  try {
    fs.symlinkSync(path.join(root, 'private'), path.join(input, 'linked'));
  } catch (e) {
    if (e.code === 'EPERM' || e.code === 'EACCES') return t.skip('symlink privilege unavailable');
    throw e;
  }
  assert.throws(() => snapshot(input, output), /symlink/);
  assert.equal(fs.existsSync(path.join(output, 'linked')), false);
});

test('snapshot materializes only in-tree regular file symlinks', t => {
  const root = sandbox(t);
  const input = path.join(root, 'in');
  const output = path.join(root, 'out');
  fs.mkdirSync(path.join(input, 'src'), {recursive: true});
  fs.mkdirSync(path.join(input, '.git'));
  fs.mkdirSync(output);
  fs.writeFileSync(path.join(input, 'src', 'LICENSE'), 'permissive license text');
  fs.writeFileSync(path.join(input, '.git', 'config'), 'private Git settings');
  try {
    fs.symlinkSync(path.join('src', 'LICENSE'), path.join(input, 'LICENSE'));
  } catch (e) {
    if (e.code === 'EPERM' || e.code === 'EACCES') return t.skip('symlink privilege unavailable');
    throw e;
  }
  snapshot(input, output);
  assert.equal(fs.readFileSync(path.join(output, 'LICENSE'), 'utf8'), 'permissive license text');
  assert.equal(fs.lstatSync(path.join(output, 'LICENSE')).isSymbolicLink(), false);
  fs.symlinkSync(path.join('.git', 'config'), path.join(input, 'settings'));
  assert.throws(() => snapshot(input, output), /symlink/);
  assert.equal(fs.existsSync(path.join(output, 'settings')), false);
});

test('snapshot materializes only safe directory symlinks and rejects cycles', t => {
  const root = sandbox(t);
  const input = path.join(root, 'in');
  const output = path.join(root, 'out');
  fs.mkdirSync(input);
  fs.mkdirSync(output);
  fs.mkdirSync(path.join(input, 'crate'));
  fs.writeFileSync(path.join(input, 'crate', 'lib.rs'), 'pub fn value() -> u8 { 2 }');
  try {
    fs.symlinkSync('crate', path.join(input, 'alias'), 'dir');
  } catch (e) {
    if (e.code === 'EPERM' || e.code === 'EACCES') return t.skip('symlink privilege unavailable');
    throw e;
  }
  snapshot(input, output);
  assert.equal(fs.readFileSync(path.join(output, 'alias', 'lib.rs'), 'utf8'),
    'pub fn value() -> u8 { 2 }');
  assert.equal(fs.lstatSync(path.join(output, 'alias')).isSymbolicLink(), false);
  fs.symlinkSync('..', path.join(input, 'crate', 'parent'), 'dir');
  const other = path.join(root, 'other');
  fs.mkdirSync(other);
  assert.throws(() => snapshot(input, other), /symlink cycle|symlink escapes/);
});

test('withheld acceptance is injected only into grading copy', t => {
  const root = sandbox(t);
  const fixture = path.join(root, 'acceptance.rs');
  fs.writeFileSync(fixture, '#[test] fn preagent_v2() {}');
  const input = path.join(root, 'input');
  const output = path.join(root, 'copy');
  fs.mkdirSync(path.join(input, 'src'), {recursive: true});
  fs.mkdirSync(output);
  fs.writeFileSync(path.join(input, 'src', 'lib.rs'), '// untouched');
  snapshot(input, output);
  const config = {
    task: {source_paths: ['src/lib.rs'], acceptance_injection: 'source_test_module'},
    acceptance: fixture, pkg: {manifest_path: 'Cargo.toml'}
  };
  injectAcceptance(output, config);
  assert.equal(fs.readFileSync(path.join(input, 'src', 'lib.rs'), 'utf8'), '// untouched');
  assert.match(fs.readFileSync(path.join(output, 'src', 'lib.rs'), 'utf8'), /preagent_v2/);
  config.task.acceptance_injection = undefined;
  injectAcceptance(output, config);
  assert.equal(fs.existsSync(path.join(output, 'tests', 'preagent_v2.rs')), true);
  assert.throws(() => injectAcceptance(output, config), /conflicting acceptance/);
});

test('withheld acceptance can extend a named existing integration target', t => {
  const root = sandbox(t);
  const fixture = path.join(root, 'acceptance.rs');
  const input = path.join(root, 'input');
  const output = path.join(root, 'copy');
  fs.writeFileSync(fixture, '#[test] fn hidden_acceptance() {}');
  fs.mkdirSync(path.join(input, 'tests'), {recursive: true});
  fs.mkdirSync(output);
  fs.writeFileSync(path.join(input, 'tests', 'lib.rs'), 'fn existing_suite() {}');
  snapshot(input, output);
  const config = {
    task: {source_paths: ['tests/lib.rs'], acceptance_injection: 'integration_test_module',
      acceptance_target: 'integration'},
    acceptance: fixture, pkg: {manifest_path: 'Cargo.toml'}
  };
  injectAcceptance(output, config);
  assert.equal(fs.readFileSync(path.join(input, 'tests', 'lib.rs'), 'utf8'), 'fn existing_suite() {}');
  assert.match(fs.readFileSync(path.join(output, 'tests', 'lib.rs'), 'utf8'), /hidden_acceptance/);
  assert.deepEqual(cargoTestTarget(config.task), ['--test', 'integration']);
});

test('integration acceptance target names are constrained by the fixture registry', () => {
  const manifest = JSON.parse(fs.readFileSync(path.join(__dirname, 'candidates.json'), 'utf8'));
  const catalog = JSON.parse(fs.readFileSync(path.join(__dirname, 'locks', 'catalog.json'), 'utf8'));
  const task = manifest.cases.find(item => item.id === 'regex-utf8-boundary-tests');
  task.controls = {clean: {status: 'acceptance_passed'}, seeded: {status: 'acceptance_failed'}};
  assert.equal(configFor(task.id, manifest, catalog).task.acceptance_target, 'integration');
  task.acceptance_target = '../outside';
  assert.throws(() => configFor(task.id, manifest, catalog), /unsupported fixture configuration/);
});

test('candidate validation requires the exact root and pinned Git commit', t => {
  const root = sandbox(t);
  const repo = path.join(root, 'repo');
  fs.mkdirSync(repo);
  let result = cp.spawnSync('git', ['init', '--quiet', repo], {encoding: 'utf8'});
  assert.equal(result.status, 0, result.stderr);
  fs.writeFileSync(path.join(repo, 'README'), 'test');
  result = cp.spawnSync('git', ['add', 'README'], {cwd: repo, encoding: 'utf8'});
  assert.equal(result.status, 0, result.stderr);
  result = cp.spawnSync('git', ['-c', 'user.name=Preagent Test', '-c', 'user.email=test@example.invalid',
    '-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=', 'commit', '--quiet', '-m', 'first'],
  {cwd: repo, encoding: 'utf8'});
  assert.equal(result.status, 0, result.stderr);
  const sha = cp.execFileSync('git', ['rev-parse', 'HEAD'], {cwd: repo, encoding: 'utf8'}).trim();
  assert.equal(validateCandidate(repo, sha), fs.realpathSync(repo));
  fs.mkdirSync(path.join(repo, 'nested'));
  assert.throws(() => validateCandidate(path.join(repo, 'nested'), sha), /root of a local Git checkout/);
  assert.throws(() => validateCandidate(repo, '0'.repeat(40)), /pinned upstream revision/);
  assert.throws(() => validateCandidate(root, sha), /root of a local Git checkout/);
});
