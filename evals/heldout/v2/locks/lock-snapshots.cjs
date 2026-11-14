'use strict';
// Pin only verified GitHub Actions artifact bytes. No models or shell commands.
const fs=require('node:fs');
const path=require('node:path');
const crypto=require('node:crypto');
const root=__dirname;
const directory=path.resolve(root,'..');
const catalog=JSON.parse(fs.readFileSync(path.join(root,'catalog.json'),'utf8'));
const manifest=JSON.parse(fs.readFileSync(path.join(directory,'candidates.json'),'utf8'));
const sha=bytes=>crypto.createHash('sha256').update(bytes).digest('hex');
function validateEntry(entry){
  const repo=manifest.repositories.find(r=>r.id===entry.id);
  if(!repo || repo.revision!==entry.revision) throw Error(entry.id+': revision differs from pinned candidate');
  for(const [p,h] of [[entry.lockfile,entry.lock_sha256],[entry.inventory,entry.inventory_sha256]]){
    if(typeof p!=='string' || !/^(locks|licenses)\/[a-z]+.(lock|json)$/.test(p) ||
       !/^[0-9a-f]{64}$/.test(h) || p.includes('..')) throw Error(entry.id+': invalid snapshot');
  }
  if(entry.lockfile!=='locks/'+entry.id+'.lock' ||
     entry.inventory!=='licenses/'+entry.id+'.json') throw Error(entry.id+': path mismatch');
}
function verify(copied=false,sourceRoot=null){
  if(catalog.schema_version!==1 || catalog.repositories.length!==6) throw Error('incomplete lock catalog');
  const seen=new Set();
  const inventoryStatus={};
  for(const entry of catalog.repositories){
    if(seen.has(entry.id)) throw Error('duplicate repository '+entry.id);
    seen.add(entry.id);
    validateEntry(entry);
    const base=copied?path.join(sourceRoot,entry.artifact):directory;
    const lockSource=copied?path.join(base,entry.id+'-Cargo.lock'):path.join(base,entry.lockfile);
    const metadataSource=copied?path.join(base,entry.id+'.json'):path.join(base,entry.inventory);
    for(const [file,expected] of [[lockSource,entry.lock_sha256],[metadataSource,entry.inventory_sha256]]){
      if(!fs.existsSync(file))throw Error(entry.id+': missing artifact '+file);
      const content=fs.readFileSync(file);
      if(content.length>1024*1024 || sha(content)!==expected)throw Error(entry.id+': invalid artifact SHA-256 '+file);
    }
    const metadata=JSON.parse(fs.readFileSync(metadataSource,'utf8'));
    const upstream=manifest.repositories.find(r=>r.id===entry.id);
    if(metadata.repository!==upstream.github||metadata.revision!==entry.revision||
      metadata.kind!=='declared_dependency_license_inventory'||
      metadata.status!=='metadata_only_not_legal_approval'||metadata.error!==null||
      !Array.isArray(metadata.scopes) ||
      new Set(metadata.scopes.map(p=>p.name)).size!==upstream.packages.length ||
      upstream.packages.some(pkg=>!metadata.scopes.some(p=>p.name===pkg.name))){
      throw Error(entry.id+': metadata does not match pinned source/scopes');
    }
    inventoryStatus[entry.id]={scopes:metadata.scopes.length,license_audit:'PENDING_HUMAN_REVIEW'};
  }
  return inventoryStatus;
}
function materialize(sourceRoot){
  const result=verify(true,sourceRoot);
  for(const e of catalog.repositories){
    const base=path.join(sourceRoot,e.artifact);
    for(const [srcName,dest] of [[e.id+'-Cargo.lock',e.lockfile],[e.id+'.json',e.inventory]]){
      const target=path.join(directory,dest);
      fs.mkdirSync(path.dirname(target),{recursive:true});
      if(fs.existsSync(target))throw Error('refusing to overwrite existing snapshot '+target);
      fs.copyFileSync(path.join(base,srcName),target);
    }
  }
  verify(false);
  return result;
}
function main(){
  const args=process.argv.slice(2);
  if(args.length===1 && args[0]==='check'){
    const all=catalog.repositories.every(x=>fs.existsSync(path.join(directory,x.lockfile)));
    if(!all){process.stdout.write('Pinned locks are not materialized yet\n');return;}
    process.stdout.write(JSON.stringify(verify(false))+'\n');
  }else if(args.length===2 && args[0]==='materialize'){
    process.stdout.write(JSON.stringify(materialize(path.resolve(args[1])))+'\n');
  }else{
    process.stderr.write('Usage: node lock-snapshots.cjs check | materialize ARTIFACT_ROOT\n');
    process.exitCode=2;
  }
}
if(require.main===module) main();
module.exports={sha,verify,materialize};
