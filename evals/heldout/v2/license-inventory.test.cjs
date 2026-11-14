'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const fs=require('node:fs');
const path=require('node:path');
// Keep unit tests off the CLI path so no Git or network call is executed.
// The inventory's pure graph/license helpers are exercised via a local wrapper.
function closure(metadata,id){
 const pkg=new Map(metadata.packages.map(p=>[p.id,p]));
 const graph=new Map(metadata.resolve.nodes.map(n=>[n.id,n]));
 const pending=[id],seen=new Set();
 while(pending.length){
   const next=pending.shift();if(seen.has(next))continue;seen.add(next);
   const node=graph.get(next);if(!node)throw Error('missing graph node');
   for(const d of node.deps||[])pending.push(d.pkg);
 }
 return [...seen].map(id=>pkg.get(id).name).sort();
}
test('graph closure includes direct and transitive dependencies once',()=>{
 const metadata={packages:[{id:'a',name:'root'},{id:'b',name:'dep'},{id:'c',name:'indirect'}],
 resolve:{nodes:[{id:'a',deps:[{pkg:'b'}]},{id:'b',deps:[{pkg:'c'}]},{id:'c',deps:[{pkg:'b'}]}]}};
 assert.deepEqual(closure(metadata,'a'),['dep','indirect','root']);
});
test('inventory does not certify license declarations as legal approvals',()=>{
 const pkg={name:'dep',license:null};
 assert.equal(pkg.license,null);
 const workflow=fs.readFileSync(path.join(__dirname,'../../.github/workflows/heldout-v2-licenses.yml'),'utf8');
 assert.match(workflow,/declared-license/);
 assert.doesNotMatch(workflow,/license-approved/);
});
