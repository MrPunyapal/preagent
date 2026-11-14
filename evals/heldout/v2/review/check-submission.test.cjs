'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
const {fingerprint,validate}=require('./check-submission.cjs');
const manifest=JSON.parse(fs.readFileSync(path.resolve(__dirname,'../candidates.json'),'utf8'));
function candidate(){
 return {schema_version:1,suite:'heldout-v2',reviewer_id:'test-reviewer',
 reviewed_at:'2026-11-14',independently_completed:true,
 contract_sha256:fingerprint(manifest),
 records:manifest.cases.map(c=>({task_id:c.id,tier:'standard',
   rationale:'Test-only rationale, not an actual independent review.',
   uncertainty_notes:''}))};
}
test('blank published template is never mistaken for human review',()=>{
 const template=JSON.parse(fs.readFileSync(path.join(__dirname,'submission.template.json'),'utf8'));
 assert.equal(validate(template,manifest).valid,false);
});
test('test-only complete submission matches all case identifiers',()=>{
 const report=validate(candidate(),manifest);
 assert.equal(report.valid,true,JSON.stringify(report.errors));
 assert.equal(report.task_count,36);
});
test('duplicate record and omitted task are invalid',()=>{
 const input=candidate();input.records[1].task_id=input.records[0].task_id;
 const result=validate(input,manifest);
 assert.equal(result.valid,false);
 assert.ok(result.errors.some(x=>x.includes('repeated')));
 assert.ok(result.errors.some(x=>x.includes('missing review')));
});
test('a different prompt or acceptance criterion invalidates the digest',()=>{
 const input=candidate(),modified=structuredClone(manifest);
 modified.cases[0].acceptance_criteria[0]+=' rewritten without reviewer consent';
 const result=validate(input,modified);
 assert.equal(result.valid,false);
 assert.ok(result.errors.some(x=>x.includes('contract_sha256 mismatch')));
});
test('reviewer attestation, date, and rationale must be valid',()=>{
 const input=candidate();
 input.independently_completed=false;
 input.reviewed_at='2026-02-31';
 input.records[0].rationale='';
 const result=validate(input,manifest);
 assert.equal(result.valid,false);
 assert.ok(result.errors.length>=3);
});
test('control evidence updates do not modify review contract fingerprint',()=>{
 const modified=structuredClone(manifest);
 modified.cases[0].controls.clean={status:'acceptance_passed',evidence:'test-only'};
 assert.equal(fingerprint(modified),fingerprint(manifest));
});
