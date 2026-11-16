'use strict';
// Release checks require Node.js only in development/CI, never at runtime.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const PLATFORMS = ['Linux-X64', 'Windows-X64', 'macOS-ARM64'];
const TAG_PATTERN = /^v(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z][0-9A-Za-z.-]*)?$/;

function packageVersion(toml) {
  let section = null;
  for (const line of toml.split(/\r?\n/)) {
    const trimmed = line.trim();
    if (/^\[[^\]]+\]$/.test(trimmed)) section = trimmed;
    if (section !== '[package]') continue;
    const match = trimmed.match(/^version\s*=\s*"([^"]+)"(?:\s*#.*)?$/);
    if (match) return match[1];
  }
  throw Error('missing Cargo.toml [package] version');
}

function lockVersion(lockfile) {
  for (const chunk of lockfile.split(/^\[\[package\]\]\s*$/m).slice(1)) {
    const name = chunk.match(/^name = "([^"]+)"\s*$/m);
    const version = chunk.match(/^version = "([^"]+)"\s*$/m);
    if (name?.[1] === 'preagent') {
      if (!version) throw Error('missing preagent version in Cargo.lock');
      return version[1];
    }
  }
  throw Error('missing preagent package in Cargo.lock');
}

function checkTag(tag, cargoToml, cargoLock) {
  if (!TAG_PATTERN.test(tag)) throw Error('release tag must be vMAJOR.MINOR.PATCH[-PRERELEASE]');
  const version = packageVersion(cargoToml);
  if ('v' + version !== tag) throw Error('tag ' + tag + ' does not match Cargo.toml version ' + version);
  const locked = lockVersion(cargoLock);
  if (locked !== version) throw Error('Cargo.lock version ' + locked + ' does not match ' + version);
  return version;
}

function checkAssets(directory, tag) {
  if (!TAG_PATTERN.test(tag)) throw Error('invalid release asset tag');
  const expected = new Set();
  for (const platform of PLATFORMS) {
    const archive = 'preagent-' + tag + '-' + platform + '.tar.gz';
    expected.add(archive);
    expected.add(archive + '.sha256');
  }
  const actual = fs.readdirSync(directory).sort();
  const expectedNames = [...expected].sort();
  if (JSON.stringify(actual) !== JSON.stringify(expectedNames)) {
    throw Error('release assets incomplete or unexpected: expected ' +
      expectedNames.join(', ') + '; found ' + actual.join(', '));
  }
  for (const platform of PLATFORMS) {
    const filename = 'preagent-' + tag + '-' + platform + '.tar.gz';
    const archive = path.join(directory, filename);
    const checksum = path.join(directory, filename + '.sha256');
    if (!fs.lstatSync(archive).isFile() || !fs.lstatSync(checksum).isFile()) {
      throw Error('release asset must be a regular file: ' + filename);
    }
    const bytes = fs.readFileSync(archive);
    if (!bytes.length) throw Error('release archive is empty: ' + filename);
    const digest = crypto.createHash('sha256').update(bytes).digest('hex');
    const declared = fs.readFileSync(checksum, 'utf8').trimEnd();
    const parsed = declared.match(/^([a-f0-9]{64}) [ *](\S+)$/);
    if (!parsed || parsed[2] !== filename || parsed[1] !== digest) {
      throw Error('release checksum mismatch: ' + filename);
    }
  }
  return expectedNames;
}

function main(argv) {
  if (argv[0] === 'tag' && argv.length === 2) {
    const project = path.resolve(__dirname, '..');
    const toml = fs.readFileSync(path.join(project, 'Cargo.toml'), 'utf8');
    const lock = fs.readFileSync(path.join(project, 'Cargo.lock'), 'utf8');
    process.stdout.write('Release tag verified for preagent ' + checkTag(argv[1], toml, lock) + '\n');
    return;
  }
  if (argv[0] === 'assets' && argv.length === 3) {
    const files = checkAssets(path.resolve(argv[1]), argv[2]);
    process.stdout.write('Validated ' + files.length + ' release assets and SHA-256 checksums\n');
    return;
  }
  process.stderr.write('Usage: node scripts/release-check.cjs tag vVERSION | assets DIRECTORY vVERSION\n');
  process.exitCode = 2;
}

if (require.main === module) {
  try {
    main(process.argv.slice(2));
  } catch (error) {
    process.stderr.write('Release validation failed: ' + error.message + '\n');
    process.exitCode = 1;
  }
}
module.exports = {packageVersion, lockVersion, checkTag, checkAssets};
