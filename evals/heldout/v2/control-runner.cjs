'use strict';
// GitHub Actions clean/seeded fixture controls; never invokes a coding model.
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const crypto = require('node:crypto');
const cp = require('node:child_process');
const base = __dirname;
const enabled = new Set(['serde-numeric-range','ripgrep-escaped-glob-literal','clap-option-terminator','regex-escaped-class-range','tokio-timeout-deadline','hyper-chunked-extensions']);
const taskId = process.argv[2];
if (!enabled.has(taskId)) throw Error('task not enabled for controls');
const manifest = JSON.parse(fs.readFileSync(path.join(base,'candidates.json'),'utf8'));
const task = manifest.cases.find(c=>c.id===taskId);
const repo = manifest.repositories.find(r=>r.id===task.repo_id);
const pkg = repo.packages.find(p=>p.name===task.target_packages[0]);
if (!new Set(['serde-rs/serde','BurntSushi/ripgrep','clap-rs/clap','rust-lang/regex','tokio-rs/tokio','hyperium/hyper']).has(repo.github) ||
    !/^[0-9a-f]{40}$/.test(repo.revision) || task.target_packages.length!==1) {
  throw Error('untrusted source or ambiguous package');
}
for (const p of [pkg.manifest_path,task.seed_patch,task.withheld_acceptance_test]) {
  if (!/^[A-Za-z0-9_.\/-]+$/.test(p) || p.includes('..') || path.isAbsolute(p)) {
    throw Error('unsafe source/test path');
  }
}
const abs=p=>path.join(base,p);
const sha=x=>crypto.createHash('sha256').update(x).digest('hex');
const temp=process.env.RUNNER_TEMP || fs.mkdtempSync(path.join(os.tmpdir(),'v2-controls-'));
const checkout=path.join(temp,'preagent-v2-'+taskId+'-'+process.pid);
const outputDir=path.resolve(base,'../../../target/validation/v2-controls');
fs.mkdirSync(outputDir,{recursive:true});
const report={schema_version:1,suite:'heldout-v2-candidate-controls',id:taskId,
 repository:repo.github,revision:repo.revision,package:pkg.name,
 crate_manifest:pkg.manifest_path,
 seed_sha256:sha(fs.readFileSync(abs(task.seed_patch))),
 acceptance_sha256:sha(fs.readFileSync(abs(task.withheld_acceptance_test))),
 clean:null,seeded:null,lockfile_sha256:null,lockfile_in_upstream:null,error:null};
const save=()=>fs.writeFileSync(path.join(outputDir,taskId+'.json'),JSON.stringify(report,null,2)+'\n');
function exec(program,args,cwd,timeout=900000,extraEnv={}) {
 const r=cp.spawnSync(program,args,{cwd,encoding:'utf8',shell:false,timeout,maxBuffer:12*1024*1024,
   env:{...process.env,...extraEnv}});
 return {code:r.status,stdout:r.stdout||'',stderr:r.stderr||'',error:r.error?.message||null,signal:r.signal||null};
}
function must(program,args,cwd,timeout=900000) {
 const r=exec(program,args,cwd,timeout);
 if(r.code!==0 || r.error || r.signal)throw Error(program+' '+args.slice(0,3).join(' ')+
   ' failed: '+(r.error||r.stderr||r.stdout).slice(-2300));
 return r;
}
try {
 if(fs.existsSync(checkout))throw Error('checkout destination exists');
 must('git',['clone','--quiet','--','https://github.com/'+repo.github+'.git',checkout],temp,300000);
 must('git',['checkout','--quiet','--detach',repo.revision],checkout);
 if(must('git',['rev-parse','HEAD'],checkout).stdout.trim()!==repo.revision)throw Error('pinned SHA mismatch');
 const manifestFile=path.join(checkout,...pkg.manifest_path.split('/'));
 const sourceModule=task.acceptance_injection==='source_test_module';
 if(sourceModule){
   // Internal tests are appended in this temporary CI checkout only.
   // Agent-facing workspaces must NOT include this acceptance module.
   const target=path.join(checkout,...task.source_paths[0].split('/'));
   fs.appendFileSync(target,'\n'+fs.readFileSync(abs(task.withheld_acceptance_test),'utf8'));
 }else{
   const acceptanceDest=path.join(path.dirname(manifestFile),'tests','preagent_v2.rs');
   if(fs.existsSync(acceptanceDest))throw Error('acceptance destination already exists upstream');
   fs.mkdirSync(path.dirname(acceptanceDest),{recursive:true});
   fs.copyFileSync(abs(task.withheld_acceptance_test),acceptanceDest);
 }
 const lockFile=path.join(checkout,'Cargo.lock');
 report.lockfile_in_upstream=fs.existsSync(lockFile);
 report.upstream_lockfile_sha256=report.lockfile_in_upstream?
    sha(fs.readFileSync(lockFile)):null;
 // Cargo's full dependency resolution must match the exact archived inventory.
 const lockCatalog=JSON.parse(fs.readFileSync(path.join(base,'locks','catalog.json'),'utf8'));
 const snapshot=lockCatalog.repositories.find(x=>x.id===task.repo_id);
 if(!snapshot || snapshot.revision!==repo.revision)throw Error('no pinned dependency resolution');
 const pinned=path.join(base,...snapshot.lockfile.split('/'));
 const pinnedBytes=fs.readFileSync(pinned);
 if(sha(pinnedBytes)!==snapshot.lock_sha256)throw Error('tampered pinned Cargo.lock');
 fs.writeFileSync(lockFile,pinnedBytes);
 report.lockfile_sha256=sha(pinnedBytes);
 report.lockfile_pinned=true;
 must('cargo',['fetch','--locked','--manifest-path',manifestFile],checkout);
 if(sha(fs.readFileSync(lockFile))!==snapshot.lock_sha256)
   throw Error('Cargo modified pinned dependency resolution');
 const extraFeatures=task.repo_id==='tokio'?['--features','rt,time']:
   task.repo_id==='hyper'?['--features','full']:[];
 const args=sourceModule?
   ['test','--offline','--locked','--manifest-path',manifestFile,...extraFeatures,'--lib','preagent_v2','--','--nocapture']:
   ['test','--offline','--locked','--manifest-path',manifestFile,...extraFeatures,'--test','preagent_v2','--','--nocapture'];
 const clean=exec('cargo',args,checkout,900000,{CARGO_NET_OFFLINE:'true'});
 report.clean={status:clean.code===0&&/test result: ok\. [1-9]\d* passed/.test(clean.stdout)?
   'acceptance_passed':'infrastructure_or_build_failure',exit_code:clean.code,
   stdout_tail:clean.stdout.slice(-2000),stderr_tail:clean.stderr.slice(-1700)};
 save();
 if(report.clean.status!=='acceptance_passed'){
  process.stderr.write('CLEAN CONTROL DETAILS '+JSON.stringify(report.clean)+'\n');
  throw Error('clean control did not pass');
 }
 must('git',['apply','--check',abs(task.seed_patch)],checkout);
 must('git',['apply',abs(task.seed_patch)],checkout);
 const changed=must('git',['diff','--name-only'],checkout).stdout.trim().split(/\r?\n/);
 if(changed.length!==1||changed[0].replace(/\\/g,'/')!==task.source_paths[0])throw Error('unrelated source changed');
 const red=exec('cargo',args,checkout,900000,{CARGO_NET_OFFLINE:'true'});
 report.seeded={status:red.code!==0&&/test result: FAILED\.\s+\d+ passed;\s+[1-9]\d* failed/.test(red.stdout)?
   'acceptance_failed':'infrastructure_or_build_failure',exit_code:red.code,
   stdout_tail:red.stdout.slice(-2200),stderr_tail:red.stderr.slice(-1700)};
 save();
 if(report.seeded.status!=='acceptance_failed'){
  process.stderr.write('SEEDED CONTROL DETAILS '+JSON.stringify(report.seeded)+'\n');
  throw Error('seeded control did not fail tests');
 }
 process.stdout.write('VERIFIED '+taskId+': clean PASS / seeded FAIL\n');
} catch(error) {
 report.error=error.message;
 save();
 process.stderr.write('UNVERIFIED '+taskId+': '+(error.stack||error)+'\n');
 process.exitCode=1;
}
