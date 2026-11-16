'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const crypto = require('node:crypto');
const {packageVersion, lockVersion, checkTag, checkAssets} = require('./release-check.cjs');

const cargo = '[package]\nname = "preagent"\nversion = "0.1.0"\n\n[dependencies]\n';
const locked = 'version = 3\n\n[[package]]\nname = "preagent"\nversion = "0.1.0"\n';

test('release tags must match Cargo.toml and Cargo.lock exactly', () => {
  assert.equal(packageVersion(cargo), '0.1.0');
  assert.equal(lockVersion(locked), '0.1.0');
  assert.equal(checkTag('v0.1.0', cargo, locked), '0.1.0');
  for (const tag of ['v0.1.1', 'v0.1', 'v0.1.0-extra', '0.1.0', 'v0.1.0; echo unsafe']) {
    assert.throws(() => checkTag(tag, cargo, locked));
  }
  assert.throws(() => checkTag('v0.1.0', cargo, locked.replace('0.1.0', '0.2.0')));
  assert.throws(() => checkTag('v0.1.0', cargo.replace('version = "0.1.0"', ''), locked));
});

function fixture(t) {
  const directory = fs.mkdtempSync(path.join(os.tmpdir(), 'preagent-release-check-'));
  t.after(() => fs.rmSync(directory, {recursive: true, force: true}));
  const tag = 'v0.1.0';
  for (const platform of ['Linux-X64', 'Windows-X64', 'macOS-ARM64']) {
    const file = 'preagent-' + tag + '-' + platform + '.tar.gz';
    const bytes = Buffer.from('test-archive-' + platform);
    const sha = crypto.createHash('sha256').update(bytes).digest('hex');
    fs.writeFileSync(path.join(directory, file), bytes);
    fs.writeFileSync(path.join(directory, file + '.sha256'), sha + '  ' + file + '\n');
  }
  return directory;
}

test('publish gate requires a complete exact three-platform asset set', t => {
  const directory = fixture(t);
  assert.equal(checkAssets(directory, 'v0.1.0').length, 6);
  const filename = 'preagent-v0.1.0-Windows-X64.tar.gz.sha256';
  fs.unlinkSync(path.join(directory, filename));
  assert.throws(() => checkAssets(directory, 'v0.1.0'), /incomplete or unexpected/);
});

test('publish gate rejects corrupted archives and incorrect checksum filenames', t => {
  const directory = fixture(t);
  const archive = path.join(directory, 'preagent-v0.1.0-Linux-X64.tar.gz');
  fs.appendFileSync(archive, 'corrupted');
  assert.throws(() => checkAssets(directory, 'v0.1.0'), /checksum mismatch/);
  const bytes = fs.readFileSync(archive);
  const hash = crypto.createHash('sha256').update(bytes).digest('hex');
  fs.writeFileSync(archive + '.sha256', hash + '  other.tar.gz\n');
  assert.throws(() => checkAssets(directory, 'v0.1.0'), /checksum mismatch/);
});

test('publish gate rejects extra assets and empty archives', t => {
  const directory = fixture(t);
  fs.writeFileSync(path.join(directory, 'unexpected.txt'), 'not allowed');
  assert.throws(() => checkAssets(directory, 'v0.1.0'), /incomplete or unexpected/);
  fs.unlinkSync(path.join(directory, 'unexpected.txt'));
  fs.writeFileSync(path.join(directory, 'preagent-v0.1.0-macOS-ARM64.tar.gz'), '');
  assert.throws(() => checkAssets(directory, 'v0.1.0'), /empty/);
});
