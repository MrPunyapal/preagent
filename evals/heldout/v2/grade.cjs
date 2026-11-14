'use strict';
// Local-only v2 candidate grader. No agent invocation or acceptance injection
// into the agent-owned checkout. Only cases with verified controls are eligible.
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');

const base = __dirname;
const MAX_FILES = 25000;
const MAX_BYTES = 512 * 1024 * 1024;
const TIMEOUT_MS = 15 * 60 * 1000;
const SOURCES = new Set([
  'tokio-rs/tokio', 'serde-rs/serde', 'BurntSushi/ripgrep',
  'rust-lang/regex', 'clap-rs/clap', 'hyperium/hyper'
]);
const sha256 = content => crypto.createHash('sha256').update(content).digest('hex');
const safePath = value => typeof value === 'string' &&
  /^[A-Za-z0-9_.\/-]+$/.test(value) && !value.startsWith('/') &&
  value.split('/').every(piece => piece && piece !== '.' && piece !== '..');

function configFor(id, manifest, catalog) {
  if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(id)) throw Error('invalid task ID');
  const task = manifest.cases.find(item => item.id === id);
  if (!task) throw Error('unknown v2 task: ' + id);
  if (task.controls?.clean?.status !== 'acceptance_passed' ||
      task.controls?.seeded?.status !== 'acceptance_failed') {
    throw Error('task controls are not verified; candidate grading is disabled');
  }
  if (!safePath(task.withheld_acceptance_test) ||
      task.withheld_acceptance_test !== 'acceptance/' + id + '.rs' ||
      !Array.isArray(task.target_packages) || task.target_packages.length !== 1 ||
      !Array.isArray(task.source_paths) || task.source_paths.length < 1 ||
      task.source_paths.some(p => !safePath(p)) ||
      ![undefined, 'source_test_module'].includes(task.acceptance_injection)) {
    throw Error('unsupported fixture configuration');
  }
  const repo = manifest.repositories.find(item => item.id === task.repo_id);
  const pkg = repo?.packages.find(item => item.name === task.target_packages[0]);
  const locked = catalog.repositories.find(item => item.id === task.repo_id);
  if (!repo || !pkg || !locked || !SOURCES.has(repo.github) ||
      !/^[0-9a-f]{40}$/.test(repo.revision) ||
      repo.revision !== locked.revision || !safePath(pkg.manifest_path) ||
      !safePath(locked.lockfile) || locked.lockfile !== 'locks/' + repo.id + '.lock' ||
      !/^[0-9a-f]{64}$/.test(locked.lock_sha256)) {
    throw Error('missing or invalid pinned repository, package or dependency snapshot');
  }
  const acceptance = path.join(base, task.withheld_acceptance_test);
  const lockfile = path.join(base, locked.lockfile);
  if (!fs.statSync(acceptance).isFile() ||
      sha256(fs.readFileSync(lockfile)) !== locked.lock_sha256) {
    throw Error('acceptance fixture or pinned dependency lockfile is missing or modified');
  }
  return {task, repo, pkg, locked, acceptance, lockfile};
}

function run(program, args, cwd, timeout = TIMEOUT_MS, extraEnv = {}) {
  const result = cp.spawnSync(program, args, {
    cwd, encoding: 'utf8', shell: false, windowsHide: true,
    timeout, maxBuffer: 12 * 1024 * 1024,
    env: {...process.env, ...extraEnv}
  });
  return {code: result.status, signal: result.signal,
    error: result.error?.message || null,
    stdout: result.stdout || '', stderr: result.stderr || ''};
}

function validateCandidate(candidate, revision) {
  const absolute = fs.realpathSync(candidate);
  if (!fs.statSync(absolute).isDirectory()) throw Error('candidate must be a directory');
  // Use Git's repository-relative prefix rather than comparing absolute
  // path strings. Git for Windows may return a different canonical spelling.
  const prefix = run('git', ['rev-parse', '--show-prefix'], absolute, 15000);
  const workingTree = run('git', ['rev-parse', '--is-inside-work-tree'], absolute, 15000);
  const head = run('git', ['rev-parse', 'HEAD'], absolute, 15000);
  if (prefix.code !== 0 || workingTree.code !== 0 || head.code !== 0 ||
      prefix.error || workingTree.error || head.error ||
      prefix.stdout.trim() !== '' || workingTree.stdout.trim() !== 'true') {
    throw Error('candidate must be the root of a local Git checkout');
  }
  if (head.stdout.trim() !== revision) {
    throw Error('candidate HEAD does not match the pinned upstream revision');
  }
  return absolute;
}

function snapshot(source, destination, budget = {files: 0, bytes: 0}, depth = 0, root = source, active = new Set()) {
  if (depth > 64) throw Error('candidate directory depth limit exceeded');
  const canonical = fs.realpathSync(source);
  if (active.has(canonical)) throw Error('candidate directory symlink cycle: ' + source);
  active.add(canonical);
  try {
  const excluded = new Set(['.git', 'target', 'node_modules', '.venv', '__pycache__']);
  for (const entry of fs.readdirSync(source, {withFileTypes: true})) {
    if (excluded.has(entry.name)) continue;
    // Replaces the root lock with the pinned dependency resolution.
    if (depth === 0 && entry.name === 'Cargo.lock') continue;
    const from = path.join(source, entry.name);
    const to = path.join(destination, entry.name);
    const info = fs.lstatSync(from);
    let copyFrom = from;
    let fileInfo = info;
    if (info.isSymbolicLink()) {
      // Serde and other upstreams use legitimate in-repository license links.
      // Materialize only regular files or directories whose resolution stays inside
      // the candidate. Do not copy links to .git, build output or external data.
      const target = fs.readlinkSync(from);
      if (path.isAbsolute(target)) throw Error('absolute candidate symlink: ' + from);
      const resolved = fs.realpathSync(from);
      const relative = path.relative(fs.realpathSync(root), resolved);
      const pieces = relative.split(path.sep);
      if (!relative || relative === '..' || relative.startsWith('..' + path.sep) ||
          path.isAbsolute(relative) || pieces.some(part => excluded.has(part)) ||
          relative === 'Cargo.lock') {
        throw Error('candidate symlink escapes allowed snapshot files: ' + from);
      }
      fileInfo = fs.statSync(resolved);
      if (!fileInfo.isFile() && !fileInfo.isDirectory()) throw Error('unsupported candidate symlink: ' + from);
      copyFrom = resolved;
    }
    if (fileInfo.isDirectory()) {
      fs.mkdirSync(to);
      snapshot(copyFrom, to, budget, depth + 1, root, active);
    } else if (fileInfo.isFile()) {
      budget.files++;
      budget.bytes += fileInfo.size;
      if (budget.files > MAX_FILES || budget.bytes > MAX_BYTES) {
        throw Error('candidate snapshot exceeds file count or byte limit');
      }
      fs.copyFileSync(copyFrom, to);
      fs.chmodSync(to, fileInfo.mode & 0o777);
    } else throw Error('candidate contains unsupported file type: ' + from);
  }
  return budget;
  } finally {
    active.delete(canonical);
  }
}

function injectAcceptance(checkout, config) {
  const test = fs.readFileSync(config.acceptance, 'utf8');
  if (config.task.acceptance_injection === 'source_test_module') {
    const source = path.join(checkout, config.task.source_paths[0]);
    if (!fs.statSync(source).isFile()) throw Error('source test module target is missing');
    fs.appendFileSync(source, '\n' + test);
  } else {
    const destination = path.join(path.dirname(path.join(checkout, config.pkg.manifest_path)),
      'tests', 'preagent_v2.rs');
    if (fs.existsSync(destination)) throw Error('candidate has a conflicting acceptance test file');
    fs.mkdirSync(path.dirname(destination), {recursive: true});
    fs.writeFileSync(destination, test);
  }
}

function grade(id, candidate) {
  const manifest = JSON.parse(fs.readFileSync(path.join(base, 'candidates.json'), 'utf8'));
  const catalog = JSON.parse(fs.readFileSync(path.join(base, 'locks', 'catalog.json'), 'utf8'));
  const config = configFor(id, manifest, catalog);
  const source = validateCandidate(candidate, config.repo.revision);
  const scratch = fs.mkdtempSync(path.join(os.tmpdir(), 'preagent-v2-grade-'));
  try {
    const budget = snapshot(source, scratch);
    fs.copyFileSync(config.lockfile, path.join(scratch, 'Cargo.lock'));
    injectAcceptance(scratch, config);
    const manifestPath = path.join(scratch, config.pkg.manifest_path);
    const extraFeatures = config.repo.id === 'tokio' ? ['--features', 'rt,time,sync,process'] :
      config.repo.id === 'hyper' ? ['--features', 'full'] : [];
    const cargoArgs = config.task.acceptance_injection === 'source_test_module' ?
      ['test', '--offline', '--locked', '--manifest-path', manifestPath,
        ...extraFeatures, '--lib', 'preagent_v2', '--', '--nocapture'] :
      ['test', '--offline', '--locked', '--manifest-path', manifestPath,
        ...extraFeatures, '--test', 'preagent_v2', '--', '--nocapture'];
    const start = Date.now();
    const result = run('cargo', cargoArgs, scratch, TIMEOUT_MS, {CARGO_NET_OFFLINE: 'true'});
    const status = !result.error && !result.signal && result.code === 0 &&
      /test result: ok\.\s+[1-9]\d* passed/.test(result.stdout) ? 'acceptance_passed' :
      !result.error && !result.signal && result.code !== 0 &&
      /test result: FAILED\.\s+\d+ passed;\s+[1-9]\d* failed/.test(result.stdout) ?
        'acceptance_failed' : 'infrastructure_or_build_failure';
    return {schema_version: 1, suite: 'heldout-v2-candidate-grading', task_id: id,
      pinned_revision: config.repo.revision, pinned_lock_sha256: config.locked.lock_sha256,
      acceptance_sha256: sha256(fs.readFileSync(config.acceptance)),
      acceptance_status: status, test_exit_code: result.code, elapsed_ms: Date.now() - start,
      snapshot_files: budget.files, snapshot_bytes: budget.bytes,
      task_correctness: null, error: result.error || (result.signal ? 'signal: ' + result.signal : null),
      stdout_tail: result.stdout.slice(-1600), stderr_tail: result.stderr.slice(-1600)};
  } finally {
    fs.rmSync(scratch, {recursive: true, force: true});
  }
}

function main(args) {
  if (args.length !== 2) {
    process.stderr.write('Usage: node grade.cjs VERIFIED_TASK_ID PINNED_CANDIDATE_DIRECTORY\n');
    process.exitCode = 2;
    return;
  }
  try {
    const result = grade(args[0], args[1]);
    process.stdout.write(JSON.stringify(result, null, 2) + '\n');
    if (result.acceptance_status === 'infrastructure_or_build_failure') process.exitCode = 1;
  } catch (error) {
    process.stderr.write('V2 GRADING BLOCKED: ' + error.message + '\n');
    process.exitCode = 1;
  }
}
if (require.main === module) main(process.argv.slice(2));
module.exports = {safePath, configFor, snapshot, injectAcceptance, validateCandidate, grade};
