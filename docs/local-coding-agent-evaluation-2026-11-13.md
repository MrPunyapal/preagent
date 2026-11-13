# Local coding-agent evaluation — 2026-11-13

## Question and scope

This follow-up tested whether the installed local models could use bounded file,
patch, and test tools to repair Preagent's four frozen seeded regressions. It
did not repeat the held-out tier study or change router behavior. The frozen
task manifest, seed patches, acceptance tests, labels, and earlier reports were
left unchanged.

The previous patch-generation experiment remains separate: its 12 attempts
across Qwen 2.5 Coder 0.5B, 1.5B, and 3B all returned `NO_PATCH`. Those are
abstentions, not acceptance-test failures; see
[`heldout-local-patch-correctness-2026-11-13.json`](../evals/results/heldout-local-patch-correctness-2026-11-13.json).

## Runtime capability findings

The machine had Ollama 0.32.5, Node v24.22.2, Codex CLI 0.154.0, and the
already installed Qwen 2.5 Coder 0.5B, 1.5B, and 3B models. Claude Code,
OpenCode, and Aider were not installed. Codex CLI documents an OSS/Ollama
option, but a nested Codex invocation is disallowed in this evaluation and was
not launched.

Ollama's documented tool loop uses structured `message.tool_calls` entries and
returns tool results to the model in a follow-up request
([Ollama tool-calling documentation](https://docs.ollama.com/capabilities/tool-calling)).
Direct `/api/chat` probes of all three installed Qwen Coder sizes returned
tool-shaped JSON in ordinary `message.content`; none returned the structured
`tool_calls` field. The 0.5B and 1.5B probes also used the wrong argument name
for the requested schema. A second 1.5B probe emitted the expected JSON shape,
but still only as text.

To measure whether a minimal local loop could work despite that limitation, a
dependency-free Node runner was added. It starts each attempt from a fresh
copy of the seeded fixture and gives the model four fixed tools: read only the
seed-touched source file, apply one validated unified patch to that file, run
only `cargo test --offline --quiet`, and finish. No model-provided shell
commands are executed. Hidden acceptance tests stay outside the candidate and
are invoked only after an applied patch and the end of the model session.
Only the first valid action from a response is dispatched; additional actions
are discarded and logged, then the model receives the actual tool result.
Candidate and fixture trees are checked for unexpected changes.

## Results

The final fixed-protocol 1.5B run made one attempt on each pilot task. It
produced zero applied patches and zero acceptance-test runs:

| Task | Agent wall time | Ollama inference time | Tool actions | Result |
| --- | ---: | ---: | --- | --- |
| `ryu-negative-infinity` | 11.1 s | 10.8 s | read; rejected malformed patch; finish | No repair; not graded |
| `itoa-decimal-pairs` | 19.4 s | 19.2 s | read; malformed follow-up | Protocol error; not graded |
| `byteorder-endian-read` | 16.1 s | 15.9 s | read; malformed follow-up | Protocol error; not graded |
| `byteorder-signed-width` | 16.3 s | 16.1 s | read; malformed follow-up | Protocol error; not graded |

The model read the permitted source file in each final run. On the Ryu task it
then emitted a patch response without the required ordinary diff headers; the
runner rejected it. On the other three tasks its follow-up was not valid JSON
tool intent. None of the candidates was graded, so there are no measured
acceptance failures or successes in this experiment.

Two exploratory 3B attempts were run on the Ryu task. The model read the file,
then emitted either multiple intents in a single response or a malformed patch
envelope. No patch was applied and the task was not graded. The 3B model is
subject to the Qwen Research License, so it was kept to local exploratory
evaluation; no commercial-use claim is made.

Timing includes model load and tool-loop wall time. The final 1.5B task runs
had a median total wall time of 16.2 s and median Ollama-reported inference
time of 16.0 s. The 3B exploratory attempts took 7.5 s and 14.7 s wall time.
For comparison, the earlier patch-generation study's model medians were 2.63 s
(0.5B), 2.75 s (1.5B), and 4.34 s (3B), with every model abstaining on all four
tasks.

Per-turn resource samples from the final 1.5B task runs reached 1,197 MiB GPU
memory used of 4,096 MiB and 88% sampled utilization; the lowest sampled free
system memory was 1,036 MiB. The two 3B samples reached 2,149 MiB VRAM and 85%
sampled utilization, with 1,134 MiB lowest sampled free system memory. These
are per-turn samples, not continuous peak measurements. The 7B model was not
run: it is larger than the available GPU memory and the machine had limited
free system RAM.

There was no meaningful rules-versus-SLM-versus-agent comparison. Rules and
SLM still provide advisory tiers; this test did not connect those tiers to
different executing agents or change model/effort selection. The previous
routing agreement and overhead results therefore remain independent. The
evidence does not show that Preagent improves coding correctness, latency, or
cost.

## Runner review and validation

The added code is isolated evaluation tooling, not a production agent
framework. Model text is untrusted: JSON intent parsing is strict, tool names
and arguments are allowlisted, patch paths and file headers are validated,
patch size is capped, and `git apply --check` precedes application. The model
cannot select a process, command, working directory, or test target. Test
execution is one fixed offline `cargo test` invocation with a timeout and no
shell. Fixture copying refuses symlinks, attempt directories are never
overwritten, and the grader's candidate copy is hashed before and after.

Regression tests cover the intent parser, malformed and multi-intent replies,
path confinement, caller-supplied command rejection, and the existing patch
application checks. Timeout handling was also hardened after review: fixed
test and grader subprocesses run without a shell, and a Windows timeout invokes
`taskkill /T /F` against that runner-owned process ID to terminate descendants.
A regression test starts a child with its own child and verifies tree cleanup.
No production Rust code changed. Frozen task inputs remain untouched.

Automated validation on the updated tree:

- `node --test evals/heldout/local-patch-runner.test.cjs evals/heldout/local-tool-agent.test.cjs` — 10 tests passed.
- `node --check` for both runner files and the new test file — passed.
- `cargo fmt --all --check` — passed.
- `cargo clippy --all-targets -- -D warnings` — passed.
- `cargo test --all-targets --locked` — passed (51 unit, 7 integration).
- `cargo build --release --locked` — passed.

Raw tool-agent messages, prompts, patch text, timings, tool outcomes, and
available per-turn resource measurements are recorded in
[`local-tool-agent-2026-11-13.json`](../evals/results/local-tool-agent-2026-11-13.json).
The committed raw file redacts source returned by `read_file` to its byte count
and SHA-256 rather than republishing upstream code; full local attempt logs
remain under ignored `target/validation/local-tool-agent-attempts/`.

## Next benchmark milestone

Do not tune routing from this four-task pilot. Before another routing
comparison, build a separate, versioned 36-task set: six task classes across
six permissively licensed Rust repositories. Candidate pools include Tokio
(MIT), Serde (MIT OR Apache-2.0), ripgrep (MIT or Unlicense), regex (MIT OR
Apache-2.0), clap (MIT OR Apache-2.0), and Hyper (MIT). Verify each repository,
revision, and dependency/license scope at the pinned commit before inclusion;
license references include the upstream [Tokio manifest](https://github.com/tokio-rs/tokio/blob/master/tokio/Cargo.toml),
[Serde manifest](https://github.com/serde-rs/serde/blob/master/serde/Cargo.toml),
[ripgrep license FAQ](https://github.com/BurntSushi/ripgrep/blob/master/FAQ.md),
[regex manifest](https://github.com/rust-lang/regex/blob/master/Cargo.toml),
[clap repository](https://github.com/clap-rs/clap), and
[Hyper license](https://github.com/hyperium/hyper/blob/master/LICENSE).

Use six tasks per class across the project pool: bug fixes, refactoring, test
improvements, security-sensitive fixes, multi-file changes, and architecture
changes. Select tasks independently from dated issues or reproducible defects;
have separate reviewers write the task and acceptance criteria before any
model is run. Pin repository commits, freeze prompts and tests, confirm seeded
controls are red and clean controls green, and blind the grader. Include
rules-only, SLM-assisted, and unmodified-agent controls only when each really
changes the executing model or effort. Use repeated attempts and report
correctness, total latency, inference overhead, and resources separately.

**Recommendation:** next milestone is to find or configure a local runtime and
model that returns valid structured tool calls across a multi-step repair, then
prove it on a small non-held-out preflight set before spending the 36-task
benchmark budget. Keep Preagent routing advisory until that preflight and a
meaningful execution-condition comparison pass.
