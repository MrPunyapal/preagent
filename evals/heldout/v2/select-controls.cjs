'use strict';
// Select only held-out v2 fixtures affected by a push; workflow_dispatch runs all.
// No models, cloning, shell expansions, or changes to the benchmark itself.
const fs=require('node:fs');
const cp=require('node:child_process');
const path=require('node:path');

const root=path.resolve(__dirname,'../../..');
const manifestPath='evals/heldout/v2/candidates.json';
const manifest=JSON.parse(fs.readFileSync(path.join(root,manifestPath),'utf8'));
function testedIds(data){
 return data.cases.filter(c=>c.seed_patch&&c.withheld_acceptance_test).map(c=>c.id).sort();
}
function changes(before,after,paths){
 const available=testedIds(after);
 const impacted=new Set();
 const full=paths.some(p=>
   p==='.github/workflows/heldout-v2-controls.yml' ||
   p==='evals/heldout/v2/control-runner.cjs' ||
   p.startsWith('evals/heldout/v2/locks/') ||
   p.startsWith('evals/heldout/v2/licenses/'));
 if(full)return available;
 for(const p of paths){
   for(const id of available){
     if(p==='evals/heldout/v2/seeds/'+id+'.patch' ||
        p==='evals/heldout/v2/acceptance/'+id+'.rs')impacted.add(id);
   }
 }
 if(paths.includes(manifestPath)){
   const prior=new Map((before?.cases||[]).map(x=>[x.id,x]));
   for(const c of after.cases){
     if(!available.includes(c.id))continue;
     const x=prior.get(c.id);
     if(!x){impacted.add(c.id);continue;}
     const selected=t=>({
       prompt:t.prompt,acceptance_criteria:t.acceptance_criteria,
       seed_patch:t.seed_patch,withheld_acceptance_test:t.withheld_acceptance_test,
       source_paths:t.source_paths,acceptance_injection:t.acceptance_injection,
       repo_id:t.repo_id,target_packages:t.target_packages
     });
     if(JSON.stringify(selected(x))!==JSON.stringify(selected(c)))impacted.add(c.id);
   }
   if(before?.repositories){
     for(const r of after.repositories){
       const old=before.repositories.find(x=>x.id===r.id);
       if(!old || old.revision!==r.revision ||
         JSON.stringify(old.packages)!==JSON.stringify(r.packages)){
           for(const c of after.cases)if(c.repo_id===r.id&&available.includes(c.id))impacted.add(c.id);
       }
     }
   }
 }
 return [...impacted].sort();
}
function git(args){
 return cp.execFileSync('git',args,{cwd:root,encoding:'utf8',maxBuffer:8*1024*1024}).trim();
}
function main(){
 const cmd=process.argv[2];
 if(cmd==='all'){process.stdout.write(JSON.stringify(testedIds(manifest))+'\n');return;}
 if(cmd!=='ci')throw Error('Usage: node select-controls.cjs ci|all');
 const event=process.env.GITHUB_EVENT_NAME;
 let selected;
 if(event==='workflow_dispatch'){
   selected=testedIds(manifest);
 }else{
   let before=process.env.GITHUB_EVENT_BEFORE;
   const now=process.env.GITHUB_SHA||git(['rev-parse','HEAD']);
   if(!before || /^0+$/.test(before))before=git(['rev-parse','HEAD^']);
   let names,prior=null;
   try{
     names=git(['diff','--name-only',before,now]).split(/\r?\n/).filter(Boolean);
     const original=git(['show',before+':'+manifestPath]);
     prior=JSON.parse(original);
   }catch(err){
     // Unknown history: safest to run all enabled controls.
     selected=testedIds(manifest);
   }
   if(!selected) selected=changes(prior,manifest,names);
 }
 const out=JSON.stringify(selected);
 if(process.env.GITHUB_OUTPUT){
   fs.appendFileSync(process.env.GITHUB_OUTPUT,
     'tasks='+out+'\n'+'has_tasks='+(selected.length?'true':'false')+'\n');
 }
 process.stdout.write('Selected controls: '+out+'\n');
}
if(require.main===module)main();
module.exports={testedIds,changes};
