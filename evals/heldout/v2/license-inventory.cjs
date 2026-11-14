'use strict';
// GitHub-hosted declared-license inventory for pinned v2 package scopes.
// This is metadata evidence, NOT a legal or transitive-license approval.
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const crypto = require('node:crypto');
const {spawnSync} = require('node:child_process');

const root = __dirname;
const allow = new Set(['tokio','serde','ripgrep','regex','clap','hyper']);
const repoId = process.argv[2];
if (!allow.has(repoId)) throw Error('unknown repository ID');
const manifest = JSON.parse(fs.readFileSync(path.join(root,'candidates.json'),'utf8'));
const upstream = manifest.repositories.find(r=>r.id===repoId);
const approved = new Set(['tokio-rs/tokio','serde-rs/serde','BurntSushi/ripgrep',
  'rust-lang/regex','clap-rs/clap','hyperium/hyper']);
if (!upstream || !approved.has(upstream.github) ||
    !/^[a-f0-9]{40}$/.test(upstream.revision)) throw Error('unapproved source');
const base = process.env.RUNNER_TEMP || fs.mkdtempSync(path.join(os.tmpdir(),'v2-licenses-'));
const checkout = path.join(base,'v2-license-'+repoId+'-'+process.pid);
const output = path.resolve(root,'../../../target/validation/v2-licenses');
fs.mkdirSync(output,{recursive:true});
const sha=data=>crypto.createHash('sha256').update(data).digest('hex');
function exec(exe,args,cwd,timeout=900000){
 const p=spawnSync(exe,args,{cwd,encoding:'utf8',shell:false,maxBuffer:64*1024*1024,timeout});
 if(p.status!==0 || p.error || p.signal)throw Error(exe+' '+args.slice(0,3).join(' ')+
  ' failed: '+(p.error?.message||p.stderr||p.stdout||'').slice(-2600));
 return p.stdout.trim();
}
function collectScope(metadata,rootPackageId){
 const pkgById=new Map(metadata.packages.map(p=>[p.id,p]));
 const nodeById=new Map(metadata.resolve.nodes.map(n=>[n.id,n]));
 const queue=[rootPackageId], seen=new Set();
 while(queue.length){
   const id=queue.shift();
   if(seen.has(id))continue;
   seen.add(id);
   const node=nodeById.get(id);
   if(!node)throw Error('missing dependency graph node for '+id);
   for(const dep of (node.deps||[])) queue.push(dep.pkg);
 }
 return [...seen].map(id=>{
   const p=pkgById.get(id);
   if(!p)throw Error('missing metadata package '+id);
   return {name:p.name,version:p.version,license:p.license||null,
     license_file:p.license_file||null,source:p.source||'local-workspace'};
 }).sort((a,b)=>a.name.localeCompare(b.name)||a.version.localeCompare(b.version));
}
function needsReview(p){
 const license=(p.license||'').trim();
 if(!license)return 'missing_license_declaration';
 // This is an alert for manual review, NOT an automatic approval.
 const recognized=new Set(['MIT','Apache-2.0','BSD-2-Clause','BSD-3-Clause',
   'ISC','Unlicense','0BSD','Zlib','CC0-1.0','Unicode-3.0','Unicode-DFS-2016']);
 const parts=license.replace(/\(|\)/g,' ').split(/\s+(?:OR|AND|WITH)\s+/);
 return parts.every(x=>recognized.has(x.trim()))?null:'non_simple_license_expression';
}
function runInventory(){
 if(fs.existsSync(checkout))throw Error('checkout destination already exists');
 exec('git',['clone','--quiet','--','https://github.com/'+upstream.github+'.git',checkout],base,300000);
 exec('git',['checkout','--quiet','--detach',upstream.revision],checkout);
 if(exec('git',['rev-parse','HEAD'],checkout)!==upstream.revision)throw Error('revision mismatch');
 const lock=path.join(checkout,'Cargo.lock');
 const upstreamLockPresent=fs.existsSync(lock);
 const result={schema_version:1,kind:'declared_dependency_license_inventory',
   status:'metadata_only_not_legal_approval',repository:upstream.github,
   revision:upstream.revision,os:process.platform,platform:'x86_64-unknown-linux-gnu',
   rustc:exec('rustc',['--version'],checkout),cargo:exec('cargo',['--version'],checkout),
   upstream_lockfile_present:upstreamLockPresent,lockfile_sha256:null,
   scopes:[],warnings:[],error:null};
 try{
  for(const pkg of upstream.packages){
    if(!/^[a-zA-Z0-9_.\/-]+$/.test(pkg.manifest_path) ||
       pkg.manifest_path.includes('..'))throw Error('invalid package manifest path');
    const pkgManifest=path.join(checkout,...pkg.manifest_path.split('/'));
    if(!fs.existsSync(pkgManifest))throw Error('missing pinned Cargo.toml: '+pkg.name);
    const args=['metadata','--format-version','1','--all-features',
      '--filter-platform','x86_64-unknown-linux-gnu','--manifest-path',pkgManifest];
    if(upstreamLockPresent)args.push('--locked');
    const meta=JSON.parse(exec('cargo',args,checkout,1200000));
    const resolved=meta.packages.find(p=>p.name===pkg.name &&
      path.resolve(p.manifest_path)===path.resolve(pkgManifest));
    if(!resolved)throw Error('cannot identify target package '+pkg.name);
    const dependencies=collectScope(meta,resolved.id);
    const flagged=dependencies.map(p=>({...p,issue:needsReview(p)})).filter(p=>p.issue);
    result.scopes.push({name:pkg.name,manifest_path:pkg.manifest_path,
      declared_license:resolved.license||null,dependency_count:dependencies.length,
      declared_dependencies:dependencies,manual_review:flagged});
    if(flagged.length)result.warnings.push(pkg.name+': '+flagged.length+' expressions need review');
  }
  if(!fs.existsSync(lock))throw Error('cargo metadata did not resolve a Cargo.lock');
  result.lockfile_sha256=sha(fs.readFileSync(lock));
  fs.copyFileSync(lock,path.join(output,repoId+'-Cargo.lock'));
 }catch(e){result.error=e.message;throw e}
 finally{
  fs.writeFileSync(path.join(output,repoId+'.json'),JSON.stringify(result,null,2)+'\n');
 }
 process.stdout.write('INVENTORY '+repoId+': '+result.scopes.length+' packages, '+
   result.warnings.length+' warnings, upstream lock='+upstreamLockPresent+'\n');
}
try{runInventory()}catch(e){process.stderr.write('AUDIT BLOCKED '+repoId+': '+(e.stack||e)+'\n');process.exitCode=1}
module.exports={collectScope,needsReview};
