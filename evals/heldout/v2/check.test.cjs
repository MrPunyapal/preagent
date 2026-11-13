'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const {evaluate, gitBlobSha, isSafePath, readFrozenInputs, CLASSES} = require('./check.cjs');
const root = __dirname;
const load = () => JSON.parse(fs.readFileSync(path.join(root, 'candidates.json'), 'utf8'));

test('36 balanced candidates are structurally valid but not a frozen benchmark', () => {
  const report = evaluate(load());
  assert.equal(report.schema_valid, true, report.faults.join('\n'));
  assert.equal(report.case_count, 36);
  assert.equal(report.repository_count, 6);
  for (const key of CLASSES) assert.equal(report.per_class[key], 6);
  assert.equal(report.frozen, false);
  assert.equal(report.ready, false);
  assert.ok(report.blockers.some(x => x.includes('human reviews required')));
  assert.ok(report.blockers.some(x => x.includes('seed patch missing')));
});

test('v1 and smoke remain at their frozen Git blob IDs', () => {
  const {pilot, smoke} = readFrozenInputs();
  assert.equal(gitBlobSha(pilot), '1f8c1f7417220609b54a1a5c828f4794e4086760');
  assert.equal(gitBlobSha(smoke), '8c766652d528f6165ec51a63d0cd885b4fd36949');
});

test('invalid or traversing source paths are rejected before any grading', () => {
  for (const bad of ['../other.rs','/etc/passwd','C:\\temp\\file.rs','dir//file.rs','a/./b','']) {
    assert.equal(isSafePath(bad), false, bad);
  }
  const data = load();
  data.cases[0].source_paths = ['../../secret'];
  assert.equal(evaluate(data).schema_valid, false);
});

test('duplicate cases and unbalanced class coverage cannot pass', () => {
  const data = load();
  data.cases[1].id = data.cases[0].id;
  data.cases[1].task_class = 'architecture';
  const result = evaluate(data);
  assert.equal(result.schema_valid, false);
  assert.ok(result.faults.some(x => x.includes('duplicate')));
});

test('premature frozen status is rejected', () => {
  const data = load();
  data.status = 'frozen';
  const result = evaluate(data);
  assert.equal(result.ready, false);
  assert.equal(result.schema_valid, false);
  assert.ok(result.faults.some(x => x.includes('frozen status prohibited')));
});

test('reviewer disagreement must remain in the record, not a fabricated consensus', () => {
  const data = load();
  data.cases[0].independent_reviews = [
    {reviewer_id:'reviewer-a',tier:'simple',reviewed_at:'2026-11-13'},
    {reviewer_id:'reviewer-b',tier:'complex',reviewed_at:'2026-11-13'}
  ];
  const result = evaluate(data);
  assert.equal(result.schema_valid, true);
  assert.deepEqual(data.cases[0].independent_reviews.map(x => x.tier), ['simple','complex']);
});
