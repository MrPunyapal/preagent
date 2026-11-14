'use strict';

// Validate a human review *submission's shape* against the exact versioned
// task contract. Does not verify authorship, independence or authenticity.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');

const manifestPath=path.resolve(__dirname,'../candidates.json');
const tiers=new Set(['simple','standard','complex']);

function contract(manifest){
  return manifest.cases.map(c=>{
    const repo=manifest.repositories.find(r=>r.id===c.repo_id);
    if(!repo)throw Error('review references missing repo: '+c.id);
    return {id:c.id,task_class:c.task_class,github:repo.github,
      revision:repo.revision,prompt:c.prompt,
      acceptance_criteria:c.acceptance_criteria};
  });
}
function fingerprint(manifest){
  return crypto.createHash('sha256').update(JSON.stringify(contract(manifest))).digest('hex');
}
function validate(submission,manifest){
  const errors=[];
  const invalid=reason=>errors.push(reason);
  if(!submission||typeof submission!=='object'||Array.isArray(submission)) {
    return {valid:false,errors:['submission must be an object']};
  }
  if(submission.schema_version!==1||submission.suite!=='heldout-v2')
    invalid('wrong review schema or suite');
  if(typeof submission.reviewer_id!=='string'||
      !/^[a-zA-Z0-9_-]{3,64}$/.test(submission.reviewer_id))
    invalid('reviewer_id must identify one reviewer (GitHub handle or stable ID)');
  const date=submission.reviewed_at;
  if(typeof date!=='string'||!/^\d{4}-\d{2}-\d{2}$/.test(date)||
    !Number.isFinite(Date.parse(date))||
    new Date(date).toISOString().slice(0,10)!==date)
    invalid('reviewed_at must be a valid YYYY-MM-DD calendar date');
  if(submission.independently_completed!==true)
    invalid('review must attest independent completion');
  const expected=fingerprint(manifest);
  if(submission.contract_sha256!==expected)
    invalid('contract_sha256 mismatch: packet changed or digest absent');
  const cases=contract(manifest), ids=new Set(cases.map(c=>c.id));
  const rows=Array.isArray(submission.records)?submission.records:[];
  if(rows.length!==cases.length)
    invalid('expected exactly '+cases.length+' review records, found '+rows.length);
  const observed=new Set();
  for(const row of rows) {
    if(!row||typeof row!=='object'||Array.isArray(row)) {
      invalid('invalid review record');
      continue;
    }
    const id=row.task_id;
    if(!ids.has(id)||observed.has(id)) {
      invalid('unknown or repeated task_id: '+String(id));
      continue;
    }
    observed.add(id);
    if(!tiers.has(row.tier))invalid(id+': invalid or missing tier');
    if(typeof row.rationale!=='string'||row.rationale.trim().length<12)
      invalid(id+': rationale required (at least 12 chars)');
    if(typeof row.uncertainty_notes!=='string')
      invalid(id+': uncertainty_notes must be string (may be empty)');
  }
  for(const c of cases)if(!observed.has(c.id))
    invalid('missing review for '+c.id);
  return {valid:errors.length===0,errors,expected_contract_sha256:expected,
    task_count:cases.length,reviewer_id:submission.reviewer_id||null};
}
function cli(){
  const args=process.argv.slice(2);
  if(args.length===1&&args[0]==='fingerprint'){
    const source=JSON.parse(fs.readFileSync(manifestPath,'utf8'));
    process.stdout.write(fingerprint(source)+'\n');
    return;
  }
  if(args.length!==1||args[0]==='--help'){
    process.stderr.write('Usage: node check-submission.cjs fingerprint | PATH_TO_JSON\n');
    process.exitCode=2;
    return;
  }
  const source=JSON.parse(fs.readFileSync(manifestPath,'utf8'));
  const user=JSON.parse(fs.readFileSync(args[0],'utf8'));
  const result=validate(user,source);
  process.stdout.write(JSON.stringify(result,null,2)+'\n');
  if(!result.valid)process.exitCode=1;
}
if(require.main===module)cli();
module.exports={fingerprint,validate,contract};
