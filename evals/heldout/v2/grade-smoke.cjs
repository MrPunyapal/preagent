'use strict';
// End-to-end clean/seeded control for the isolated local v2 grader.
// Downloads only the pinned upstream source and pinned Cargo dependencies.
// Does not start coding agents, local models or model evaluation.
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const {grade} = require('./grade.cjs');

const base = __dirname;
const id = 'serde-numeric-range';
const manifest = JSON.parse(fs.readFileSync(path.join(base, 'candidates.json'), 'utf8'));
const task = manifest.cases.find(item => item.id === id);
const repo = manifest.repositories.find(item => item.id === task.repo_id);
const catalog = JSON.parse(fs.readFileSync(path.join(base, 'locks', 'catalog.json'), 'utf8'));
const lock = catalog.repositories.find(item => item.id === repo.id);
const hash = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

function run(program, args, cwd, timeout = 15 * 60 * 1000) {
  const result = cp.spawnSync(program, args, {
    cwd, shell: false, encoding: 'utf8', timeout, maxBuffer: 12 * 1024 * 1024
  });
  if (result.error || result.signal || result.status !== 0) {
    throw Error(program + ' failed: ' + (result.error?.message || result.signal ||
      result.stderr || result.stdout || result.status).toString().slice(-2400));
  }
  return result.stdout.trim();
}

function main() {
  if (repo.github !== 'serde-rs/serde' || !/^[0-9a-f]{40}$/.test(repo.revision) ||
      task.seed_patch !== 'seeds/' + id + '.patch' ||
      lock.revision !== repo.revision) throw Error('unrecognized pinned smoke fixture');
  const temp = fs.mkdtempSync(path.join(process.env.RUNNER_TEMP || os.tmpdir(), 'preagent-grade-smoke-'));
  const checkout = path.join(temp, 'serde');
  process.env.CARGO_TARGET_DIR = path.join(temp, 'cargo-target');
  try {
    run('git', ['clone', '--quiet', '--', 'https://github.com/serde-rs/serde.git', checkout], temp, 300000);
    run('git', ['checkout', '--quiet', '--detach', repo.revision], checkout);
    if (run('git', ['rev-parse', 'HEAD'], checkout) !== repo.revision) throw Error('pinned checkout mismatch');
    const lockfile = path.join(base, lock.lockfile);
    if (hash(fs.readFileSync(lockfile)) !== lock.lock_sha256) throw Error('pin hash mismatch');
    fs.copyFileSync(lockfile, path.join(checkout, 'Cargo.lock'));
    run('cargo', ['fetch', '--locked', '--manifest-path',
      path.join(checkout, 'serde_core', 'Cargo.toml')], checkout);

    const beforeClean = run('git', ['status', '--porcelain=v1', '--untracked-files=all'], checkout);
    const clean = grade(id, checkout);
    const afterClean = run('git', ['status', '--porcelain=v1', '--untracked-files=all'], checkout);
    if (afterClean !== beforeClean) throw Error('grader modified the clean candidate checkout');
    if (clean.acceptance_status !== 'acceptance_passed') {
      throw Error('clean grading did not pass: ' + JSON.stringify(clean));
    }

    const patch = path.join(base, task.seed_patch);
    run('git', ['apply', '--check', patch], checkout);
    run('git', ['apply', patch], checkout);
    const beforeSeeded = run('git', ['status', '--porcelain=v1', '--untracked-files=all'], checkout);
    const seeded = grade(id, checkout);
    const afterSeeded = run('git', ['status', '--porcelain=v1', '--untracked-files=all'], checkout);
    if (afterSeeded !== beforeSeeded) throw Error('grader modified the seeded candidate checkout');
    if (seeded.acceptance_status !== 'acceptance_failed') {
      throw Error('seeded grading did not fail acceptance: ' + JSON.stringify(seeded));
    }
    process.stdout.write(JSON.stringify({suite: 'heldout-v2-grader-smoke', id,
      pinned_revision: repo.revision, pinned_lock_sha256: lock.lock_sha256,
      clean: clean.acceptance_status, seeded: seeded.acceptance_status,
      original_candidate_preserved: true, task_correctness: null}, null, 2) + '\n');
  } finally {
    fs.rmSync(temp, {recursive: true, force: true});
  }
}
if (require.main === module) main();
module.exports = {run};
