const test = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const os = require('node:os');
const path = require('node:path');
const { decodeIntent, taskPaths, fixedTools, runLimited, TOOL_DEFINITIONS } = require('./local-tool-agent.cjs');

test('decodes the first JSON tool intent and records deferred calls', () => {
  assert.equal(decodeIntent({ content: '{"name":"read_file","arguments":{"path":"src/lib.rs"}}' }).name, 'read_file');
  assert.equal(decodeIntent({ content: '```json\n{"name":"run_public_tests","arguments":{}}\n```' }).name, 'run_public_tests');
  const native = decodeIntent({ tool_calls: [{ function: { name: 'read_file', arguments: { path: 'src/lib.rs' } } }, { function: { name: 'apply_patch', arguments: { patch: 'ignored' } } }] });
  assert.equal(native.name, 'read_file');
  assert.equal(native.deferred_count, 1);
  const batch = decodeIntent({ content: '{"name":"read_file","arguments":{"path":"src/lib.rs"}}\n{"name":"apply_patch","arguments":{"patch":"ignored"}}' });
  assert.equal(batch.name, 'read_file');
  assert.equal(batch.deferred_count, 1);
  assert.equal(batch.assistant_message.content, '{"name":"read_file","arguments":{"path":"src/lib.rs"}}');
});

test('rejects prose and malformed tool intents', () => {
  assert.throws(() => decodeIntent({ content: 'I will inspect the file now.' }), /neither one JSON/);
  assert.throws(() => decodeIntent({ content: '{"name":"read_file","arguments":[]}' }), /Malformed/);
  assert.throws(() => decodeIntent({ content: '{"name":"read_file","arguments":{}}\nnot json' }), /newline-separated/);
});

test('offers only fixed, argument-bounded local tools', () => {
  assert.deepEqual(TOOL_DEFINITIONS.map(tool => tool.function.name), ['read_file', 'apply_patch', 'run_public_tests', 'finish']);
  assert.throws(() => taskPaths('../outside', 'qwen'), /Unsafe task identifier/);
  assert.ok(taskPaths('ryu-negative-infinity', 'qwen').attemptNumber >= 1);
});

test('tool dispatcher confines reads and refuses caller-selected commands', async () => {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 'preagent-tool-agent-test-'));
  try {
    fs.mkdirSync(path.join(root, 'src'));
    fs.writeFileSync(path.join(root, 'src', 'lib.rs'), 'pub fn value() -> u8 { 1 }\n');
    const tools = fixedTools(root, 'src/lib.rs', { public_test_runs: [] });
    assert.equal((await tools.invoke('read_file', { path: 'src/lib.rs' })).content, 'pub fn value() -> u8 { 1 }\n');
    assert.equal((await tools.invoke('read_file', { path: '../outside' })).status, 'rejected');
    assert.equal((await tools.invoke('run_public_tests', { command: 'whoami' })).status, 'rejected');
    assert.equal((await tools.invoke('apply_patch', { patch: 'NO_PATCH', path: '../outside' })).status, 'rejected');
  } finally {
    fs.rmSync(root, { recursive: true, force: true });
  }
});

test('timed-out Windows process trees are terminated without a shell', async () => {
  const grandchild = 'setInterval(() => {}, 1000)';
  const parent = `require('node:child_process').spawn(process.execPath, ['-e', ${JSON.stringify(grandchild)}], {stdio:'ignore', windowsHide:true}); setInterval(() => {}, 1000);`;
  const result = await runLimited(process.execPath, ['-e', parent], { timeout: 500, maxBuffer: 4096 });
  assert.equal(result.error.code, 'ETIMEDOUT');
  if (process.platform === 'win32') assert.equal(result.cleanup_status, 'process_tree_killed');
});
