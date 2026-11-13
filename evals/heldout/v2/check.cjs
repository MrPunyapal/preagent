'use strict';

// Version-2 candidate/readiness checker. No networking, inference, Git writes,
// subprocesses, fixture preparation, grading or changes to the production CLI.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');

const DIR = __dirname;
const CLASSES = [
  'bug_fix', 'refactoring', 'test_improvements',
  'security_sensitive', 'multi_file', 'architecture'
];
const FROZEN_V1_BLOB = '1f8c1f7417220609b54a1a5c828f4794e4086760';
const FROZEN_SMOKE_BLOB = '8c766652d528f6165ec51a63d0cd885b4fd36949';

function gitBlobSha(data) {
  // TSV files are text: Git for Windows may check them out with CRLF.
  // Hash the canonical LF form to compare with the immutable Git blob.
  const raw = Buffer.isBuffer(data) ? data : Buffer.from(data);
  const bytes = Buffer.from(raw.toString('utf8').replace(/\r\n/g, '\n'));
  return crypto.createHash('sha1')
    .update('blob ' + bytes.length + '\0')
    .update(bytes)
    .digest('hex');
}

function isSafePath(value) {
  if (typeof value !== 'string' || !value || value.includes('\\') ||
      value.includes('\0') || value.startsWith('/') ||
      /^[A-Za-z]:/.test(value)) return false;
  return value.split('/').every(part => part && part !== '.' && part !== '..');
}

function nonempty(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function readFrozenInputs(root = DIR) {
  const pilot = fs.readFileSync(path.resolve(root, '../v1.tsv'));
  const smoke = fs.readFileSync(path.resolve(root, '../../baseline.tsv'));
  return {pilot, smoke};
}

function evaluate(manifest, options = {}) {
  const dir = options.dir || DIR;
  const faults = [];
  const blockers = [];
  const fail = msg => faults.push(msg);
  const block = msg => blockers.push(msg);
  const expectedClasses = new Set(CLASSES);

  if (!manifest || typeof manifest !== 'object' || Array.isArray(manifest)) {
    return {schema_valid: false, ready: false, faults: ['manifest is not an object'], blockers: []};
  }
  if (manifest.schema_version !== 2 || manifest.name !== 'heldout-v2') {
    fail('expected heldout-v2 schema_version 2');
  }
  if (manifest.status !== 'candidate_not_frozen' && manifest.status !== 'frozen') {
    fail('invalid manifest status');
  }
  if (manifest.model_results !== null) {
    fail('candidate manifest must not include model output');
  }
  if (!Array.isArray(manifest.task_classes) ||
      manifest.task_classes.length !== 6 ||
      new Set(manifest.task_classes).size !== 6 ||
      manifest.task_classes.some(c => !expectedClasses.has(c))) {
    fail('expected six distinct recognized task classes');
  }

  const repos = new Map();
  if (!Array.isArray(manifest.repositories) || manifest.repositories.length !== 6) {
    fail('expected exactly six pinned upstream repositories');
  }
  for (const r of (Array.isArray(manifest.repositories) ? manifest.repositories : [])) {
    if (!nonempty(r.id) || repos.has(r.id)) {
      fail('duplicate or missing repository ID');
      continue;
    }
    repos.set(r.id, r);
    if (!/^[0-9a-f]{40}$/.test(r.revision || '')) fail(r.id + ': missing pinned commit SHA');
    if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(r.github || '')) {
      fail(r.id + ': invalid upstream repository');
    }
    if (!isSafePath(r.crate_manifest)) fail(r.id + ': invalid crate manifest path');
    if (!nonempty(r.package) || !nonempty(r.declared_license) ||
        !nonempty(r.license_evidence)) fail(r.id + ': license/package metadata missing');
    if (!Array.isArray(r.packages) || r.packages.length < 1) {
      fail(r.id + ': package scopes missing');
    } else {
      const scoped = new Set();
      for (const p of r.packages) {
        if (!nonempty(p.name) || scoped.has(p.name) ||
            !isSafePath(p.manifest_path) ||
            !nonempty(p.declared_license) ||
            !nonempty(p.license_evidence)) {
          fail(r.id + ': invalid or duplicate package scope');
        }
        scoped.add(p.name);
        if (p.dependency_scope_status !== 'verified') {
          block(r.id + '/' + p.name + ': transitive dependency/license scope not audited');
        }
      }
    }

  }

  const classes = new Map(CLASSES.map(x => [x, 0]));
  const perRepo = new Map([...repos.keys()].map(x => [x, 0]));
  const ids = new Set(), prompts = new Set();
  const cases = Array.isArray(manifest.cases) ? manifest.cases : [];
  if (cases.length !== 36) fail('expected exactly 36 task contracts; found ' + cases.length);

  let v1, smoke;
  try {
    ({pilot: v1, smoke} = readFrozenInputs(dir));
    if (gitBlobSha(v1) !== FROZEN_V1_BLOB) fail('v1.tsv differs from frozen pilot blob');
    if (gitBlobSha(smoke) !== FROZEN_SMOKE_BLOB) fail('baseline.tsv differs from frozen smoke blob');
  } catch (error) {
    fail('cannot read frozen v1/smoke inputs: ' + error.message);
  }
  const originalPrompts = new Set();
  for (const input of [v1, smoke]) {
    if (!input) continue;
    for (const line of input.toString('utf8').split(/\r?\n/)) {
      if (line && !line.startsWith('#')) {
        const fields = line.split('\t');
        const last = fields[fields.length - 1];
        if (last) originalPrompts.add(last);
      }
    }
  }

  for (const c of cases) {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(c.id || '') || ids.has(c.id)) {
      fail('invalid/duplicate case ID: ' + String(c.id));
    }
    ids.add(c.id);
    if (!classes.has(c.task_class)) {
      fail(String(c.id) + ': invalid task class');
    } else classes.set(c.task_class, classes.get(c.task_class) + 1);
    if (!repos.has(c.repo_id)) {
      fail(String(c.id) + ': unknown repo ID');
    } else {
      perRepo.set(c.repo_id, perRepo.get(c.repo_id) + 1);
      const knownPackages = new Set((repos.get(c.repo_id).packages || []).map(x => x.name));
      if (!Array.isArray(c.target_packages) || c.target_packages.length < 1 ||
          new Set(c.target_packages).size !== c.target_packages.length ||
          c.target_packages.some(x => !knownPackages.has(x))) {
        fail(String(c.id) + ': target package scope is missing or unknown');
      }
    }
    if (!Array.isArray(c.source_paths) || !c.source_paths.length ||
        c.source_paths.some(p => !isSafePath(p))) {
      fail(String(c.id) + ': unsafe/missing source paths');
    }
    if ((c.task_class === 'multi_file' || c.task_class === 'architecture') &&
        (!Array.isArray(c.source_paths) || c.source_paths.length < 2)) {
      fail(String(c.id) + ': multi-file/architecture contract needs two source files');
    }
    if (!nonempty(c.prompt) || c.prompt.length < 45 ||
        originalPrompts.has(c.prompt) || prompts.has(c.prompt)) {
      fail(String(c.id) + ': empty/duplicate/too-short prompt or reuses v1/smoke');
    }
    prompts.add(c.prompt);
    if (!Array.isArray(c.acceptance_criteria) || c.acceptance_criteria.length < 2 ||
        c.acceptance_criteria.some(x => !nonempty(x))) {
      fail(String(c.id) + ': needs at least two explicit acceptance criteria');
    }
    if (!nonempty(c.seed_plan)) fail(String(c.id) + ': seed design missing');
    if (!c.provenance || c.provenance.type !== 'controlled_regression_proposal') {
      fail(String(c.id) + ': historical claim or provenance is not substantiated');
    }
    if (!c.seed_patch) {
      block(String(c.id) + ': verified seed patch missing');
    } else if (c.seed_patch !== 'seeds/' + c.id + '.patch' ||
               !isSafePath(c.seed_patch) ||
               !fs.existsSync(path.join(dir, c.seed_patch))) {
      fail(String(c.id) + ': seed patch path invalid or file missing');
    }
    if (!c.withheld_acceptance_test) {
      block(String(c.id) + ': withheld executable acceptance test missing');
    } else if (c.withheld_acceptance_test !== 'acceptance/' + c.id + '.rs' ||
               !isSafePath(c.withheld_acceptance_test) ||
               !fs.existsSync(path.join(dir, c.withheld_acceptance_test))) {
      fail(String(c.id) + ': acceptance test path invalid or file missing');
    }

    const review = Array.isArray(c.independent_reviews) ? c.independent_reviews : [];
    const distinct = new Set(review.map(r => r.reviewer_id));
    if (review.length < 2 || distinct.size < 2 ||
        review.some(r => !nonempty(r.reviewer_id) ||
          !['simple','standard','complex'].includes(r.tier) ||
          !nonempty(r.reviewed_at))) {
      block(String(c.id) + ': two separately recorded human reviews required; preserve all disagreement');
    }
    if (!c.controls || c.controls.clean?.status !== 'acceptance_passed' ||
        c.controls.seeded?.status !== 'acceptance_failed' ||
        !nonempty(c.controls.clean?.evidence) ||
        !nonempty(c.controls.seeded?.evidence)) {
      block(String(c.id) + ': independent clean-positive and seeded-negative controls pending');
    }
  }

  for (const cls of CLASSES) {
    if (classes.get(cls) !== 6) fail(cls + ': expected six cases, found ' + classes.get(cls));
  }
  for (const [id, count] of perRepo) {
    if (count !== 6) fail(id + ': expected six cases, found ' + count);
    for (const cls of CLASSES) {
      const countClass = cases.filter(c => c.repo_id === id && c.task_class === cls).length;
      if (countClass !== 1) fail(id + ': expected one case of class ' + cls);
    }
  }

  const structural = faults.length === 0;
  const ready = structural && blockers.length === 0;
  if (manifest.status === 'frozen') {
    // Freeze is a separately reviewed content lock, never just an editable flag.
    const lockPath = path.join(dir, 'freeze.lock.json');
    if (!fs.existsSync(lockPath)) {
      fail('frozen status requires a separately reviewed freeze.lock.json');
    } else {
      try {
        const lock = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
        const hashText = value => crypto.createHash('sha256')
          .update(value.toString('utf8').replace(/\r\n/g, '\n'))
          .digest('hex');
        const manifestBytes = fs.readFileSync(path.join(dir, 'candidates.json'));
        if (lock.manifest_sha256 !== hashText(manifestBytes)) {
          fail('frozen manifest SHA-256 does not match its lock');
        }
        const required = cases.flatMap(c => [c.seed_patch, c.withheld_acceptance_test])
          .filter(Boolean);
        if (!lock.files || Object.keys(lock.files).length !== required.length ||
            required.some(f => !isSafePath(f) || !fs.existsSync(path.join(dir, f)) ||
              lock.files[f] !== hashText(fs.readFileSync(path.join(dir, f))))) {
          fail('frozen seed/acceptance hashes incomplete or inconsistent');
        }
        const signatures = lock.human_signoffs || [];
        if (!Array.isArray(signatures) || new Set(signatures.map(s => s.reviewer_id)).size < 2 ||
            signatures.some(s => !nonempty(s.reviewer_id) || !nonempty(s.signed_at))) {
          fail('frozen content lock lacks two independent recorded signoffs');
        }
      } catch (error) {
        fail('cannot verify frozen content lock: ' + error.message);
      }
    }
  }
  if (manifest.status === 'frozen' && !ready) fail('frozen status prohibited before readiness gates pass');
  if (manifest.status === 'candidate_not_frozen' && ready) {
    block('reviewed/verified task set requires a separate frozen lock and human signoff');
  }
  return {
    schema_valid: faults.length === 0,
    ready: faults.length === 0 && blockers.length === 0 && manifest.status === 'frozen',
    frozen: manifest.status === 'frozen',
    case_count: cases.length,
    repository_count: repos.size,
    per_class: Object.fromEntries(classes),
    faults,
    blockers
  };
}

function main() {
  const args = process.argv.slice(2);
  if (!['draft', 'ready'].includes(args[0]) ||
      args.some((v, i) => i !== 0 && v !== '--json') || args.length > 2) {
    process.stderr.write('Usage: node check.cjs draft|ready [--json]\n');
    process.exitCode = 2;
    return;
  }
  const manifest = JSON.parse(fs.readFileSync(path.join(DIR, 'candidates.json'), 'utf8'));
  const result = evaluate(manifest);
  const detailed = args.includes('--json');
  if (detailed) {
    process.stdout.write(JSON.stringify(result, null, 2) + '\n');
  } else {
    process.stdout.write('heldout-v2: ' + result.case_count + ' cases, ' +
      result.repository_count + ' repos; draft=' + result.schema_valid +
      ', frozen=' + result.frozen + ', ready=' + result.ready + '\n');
    for (const issue of result.faults) process.stdout.write('FAULT: ' + issue + '\n');
    if (args[0] === 'ready') {
      for (const issue of result.blockers) process.stdout.write('BLOCKED: ' + issue + '\n');
    } else {
      process.stdout.write('Readiness blockers: ' + result.blockers.length +
        ' (expected for an unreviewed candidate set)\n');
    }
  }
  if (!result.schema_valid || (args[0] === 'ready' && !result.ready)) {
    process.exitCode = 1;
  }
}

if (require.main === module) main();
module.exports = {evaluate, gitBlobSha, isSafePath, readFrozenInputs, CLASSES};
