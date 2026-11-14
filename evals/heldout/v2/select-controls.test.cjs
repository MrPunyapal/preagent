'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {changes,testedIds}=require('./select-controls.cjs');
const data=JSON.parse(fs.readFileSync(path.join(__dirname,'candidates.json'),'utf8'));
test('all includes only complete fixtures',()=>{
 const list=testedIds(data);
 assert.ok(list.length>=6 && list.length<36);
 assert.ok(list.includes('tokio-timeout-deadline'));
 assert.ok(!list.includes('tokio-bounded-backpressure-tests'));
});
test('documentation and review edits do not rerun expensive controls',()=>{
 assert.deepEqual(changes(data,data,['evals/heldout/v2/review/README.md']),[]);
 const next=structuredClone(data);
 next.cases[0].controls.clean={status:'acceptance_passed',evidence:'test-only'};
 assert.deepEqual(changes(data,next,['evals/heldout/v2/candidates.json']),[]);
});
test('one seed patch only selects that task',()=>{
 assert.deepEqual(changes(data,data,['evals/heldout/v2/seeds/tokio-timeout-deadline.patch']),['tokio-timeout-deadline']);
});
test('changing the named acceptance target selects only its fixture',()=>{
 const next=structuredClone(data);
 next.cases.find(c=>c.id==='regex-utf8-boundary-tests').acceptance_target='other-integration';
 assert.deepEqual(changes(data,next,['evals/heldout/v2/candidates.json']),['regex-utf8-boundary-tests']);
});
test('harness changes require all enabled controls',()=>{
 assert.deepEqual(changes(data,data,['evals/heldout/v2/control-runner.cjs']),testedIds(data));
 assert.deepEqual(changes(data,data,['evals/heldout/v2/locks/serde.lock']),testedIds(data));
});
test('new proposed case becomes active only with seed and acceptance source',()=>{
 const next=structuredClone(data);
 const c=next.cases.find(c=>c.id==='tokio-bounded-backpressure-tests');
 c.seed_patch='seeds/tokio-bounded-backpressure-tests.patch';
 c.withheld_acceptance_test='acceptance/tokio-bounded-backpressure-tests.rs';
 assert.deepEqual(changes(data,next,['evals/heldout/v2/candidates.json']),['tokio-bounded-backpressure-tests']);
});
