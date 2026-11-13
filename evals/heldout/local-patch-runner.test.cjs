const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { parseSeed, validRelativePath, validatePatch, applyValidatedPatch } = require('./local-patch-runner.cjs');

const allowed = 'src/lib.rs';
const validDiff = [
  'diff --git a/src/lib.rs b/src/lib.rs',
  'index 1111111..2222222 100644',
  '--- a/src/lib.rs',
  '+++ b/src/lib.rs',
  '@@ -1 +1 @@',
  '-fn old() {}',
  '+fn new() {}',
].join('\n') + '\n';

test('accepts a single-file patch for the allowed source file', () => {
  assert.equal(validatePatch(validDiff, allowed), null);
});

test('rejects traversal and paths outside the allowlist', () => {
  const traversal = validDiff.replaceAll('src/lib.rs', '../outside.rs');
  const otherFile = validDiff.replaceAll('src/lib.rs', 'src/main.rs');
  assert.match(validatePatch(traversal, allowed), /unsafe path|allowlist/);
  assert.match(validatePatch(otherFile, allowed), /allowlist/);
  assert.equal(validRelativePath('src/../../outside.rs'), false);
  assert.equal(validRelativePath('C:/outside.rs'), false);
});

test('rejects prose, fences, multiple files, and file operations', () => {
  assert.match(validatePatch(`\u0060\u0060\u0060diff\n${validDiff}`, allowed), /raw unified/);
  assert.match(validatePatch(`${validDiff}\n${validDiff}`, allowed), /exactly one/);
  assert.match(validatePatch(validDiff.replace('index ', 'new file mode 100644\nindex '), allowed), /mode changes/);
  assert.match(validatePatch('NO_PATCH', allowed), /raw unified/);
});

test('derives the allowlist and source hunk from the frozen seed only', () => {
  const seed = parseSeed('seeds/byteorder-signed-width.patch');
  assert.equal(seed.allowedFile, 'src/lib.rs');
  assert.deepEqual(seed.hunks, [{ start: 86, count: 13 }]);
});

test('applies an allowed patch in an isolated Git worktree and blocks traversal', () => {
  const tmp = path.resolve(os.tmpdir());
  const root = fs.mkdtempSync(path.join(tmp, 'preagent-patch-runner-test-'));
  try {
    const repo = path.join(root, 'repo');
    fs.mkdirSync(repo);
    const { spawnSync } = require('node:child_process');
    const git = args => {
      const result = spawnSync('git', ['-C', repo, ...args], { encoding: 'utf8', shell: false });
      assert.equal(result.status, 0, result.stderr);
    };
    git(['init', '--quiet']);
    git(['config', 'user.name', 'Preagent Test']);
    git(['config', 'user.email', 'test@example.invalid']);
    git(['config', 'core.hooksPath', '']);
    git(['config', 'commit.gpgsign', 'false']);
    git(['config', 'core.autocrlf', 'false']);
    fs.mkdirSync(path.join(repo, 'src'));
    fs.writeFileSync(path.join(repo, 'src', 'lib.rs'), 'fn old() {}\n');
    git(['add', 'src/lib.rs']);
    git(['commit', '--quiet', '-m', 'fixture']);

    assert.deepEqual(applyValidatedPatch(repo, validDiff, allowed), { status: 'applied', error: null });
    assert.equal(fs.readFileSync(path.join(repo, 'src', 'lib.rs'), 'utf8'), 'fn new() {}\n');

    const traversal = validDiff.replaceAll('src/lib.rs', '../outside.rs');
    const rejected = applyValidatedPatch(repo, traversal, allowed);
    assert.equal(rejected.status, 'rejected');
    assert.equal(fs.existsSync(path.join(root, 'outside.rs')), false);
  } finally {
    const target = path.resolve(root);
    assert.equal(path.dirname(target), tmp);
    assert.match(path.basename(target), /^preagent-patch-runner-test-/);
    fs.rmSync(target, { recursive: true, force: true });
  }
});
