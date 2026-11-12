# Held-out Windows workstation evaluation: 12 November 2026

## Findings

The workstation build and controls completed successfully. Deterministic rules
matched **2/4** frozen labels. Of the three installed SLMs tested, `qwen2.5-coder:1.5b`
had the highest label agreement at **3/4**, compared with **1/4** for the 0.5B
and 3B models. All 36 SLM replies were valid. This is a four-task descriptive
pilot, not a general routing-accuracy estimate.

Warm four-task SLM runs took a median **0.79–0.86 seconds** end to end; rules
alone took a median **22.3 ms**. The 1.5B model improved agreement by one task
on this set, while adding roughly 0.75 seconds per four-task route and a first
request of 8.66 seconds. No coding agent repair was tested, so coding-task
correctness remains **not tested**.

Recommendation: keep deterministic rules as the default. Keep SLM routing
available only as an explicit opt-in; the four-case result does not justify
adding its latency and resource use to default routing. Expand the independently
selected benchmark and obtain separate local-agent correctness measurements
before reconsidering that decision.

## Provenance and environment

- Evaluation date: 2026-11-12; repository source at `af7a318589e4f3c464a77687f303a794d4de95f1`.
- Windows 11 Home Single Language, build 26300, x86_64.
- Intel Core i7-12700H, 20 logical processors; 15.63 GiB visible RAM. Free RAM
  was 1.25 GiB at the routing snapshot and 0.45 GiB after the model runs.
- NVIDIA GeForce RTX 3050 Laptop GPU, 4 GiB VRAM; Intel Iris Xe integrated GPU.
- Rust/Cargo 1.96.1; Git 2.45.2; Ollama 0.32.5; Codex CLI 0.154.0.
- The installed Build Tools instance was discovered at Visual Studio **18.12.0**
  (`VSCMD_VER=18.12.0`), with MSVC **14.52.37025** and Windows SDK
  **10.0.26100.0**. The x86_64 MSVC Rust target was installed. `VsDevCmd.bat`
  supplied `cl.exe`, `link.exe`, `rc.exe`, and the SDK library paths. This records
  the detected toolchain version rather than assuming the installed instance is
  VS 2022.
- Claude Code and OpenCode were not installed. `preagent doctor --json` found
  Git, Codex and Ollama loopback, and did not find Claude Code or OpenCode.
- The smoke-corpus file hash remained
  `8c766652d528f6165ec51a63d0cd885b4fd36949`; the frozen held-out manifest hash
  remained `1f8c1f7417220609b54a1a5c828f4794e4086760`.

No model weights or additional tools were downloaded or installed for this
evaluation. The four public pinned source repositories were cloned locally
without partial-clone filters because the earlier cached copies were partial
and lacked objects.

## Rust checks and CLI

All requested local checks passed with the Visual Studio developer environment
loaded:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Pass |
| `cargo clippy --all-targets -- -D warnings` | Pass |
| `cargo test --all-targets --locked` | 51 unit and 7 integration tests passed |
| `cargo build --release --locked` | Pass |

The release binary reported `eval --json` as **45/45** correct, with no
mismatches. The held-out suite and frozen task descriptions were unchanged.

Exact workstation commands included:

```powershell
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets --locked
cargo build --release --locked
.\target\release\preagent.exe doctor --json
.\target\release\preagent.exe eval --json
.\target\release\preagent.exe heldout list --json
.\target\release\preagent.exe heldout route --json
.\target\release\preagent.exe heldout route --json --slm qwen2.5-coder:0.5b
.\target\release\preagent.exe heldout route --json --slm qwen2.5-coder:1.5b
.\target\release\preagent.exe heldout route --json --slm qwen2.5-coder:3b
```

## Deterministic routing

Twelve fresh `heldout route --json` processes produced the same **2/4** label
agreement. Each row's latency is the median of its 12 in-process rule timings.
The process-level wall time median was **22.3 ms**, p95 **53.7 ms**; sampled
maximum Preagent working set was **4.99 MiB**. The external working-set sampler
polled every 5 ms and may miss short peaks.

| Task | Frozen tier | Rules tier | Median rule time | Result |
| --- | --- | --- | ---: | --- |
| `itoa-decimal-pairs` | standard | standard | 21 μs | Match |
| `ryu-negative-infinity` | simple | simple | 12.5 μs | Match |
| `byteorder-endian-read` | standard | simple | 11 μs | Disagreement |
| `byteorder-signed-width` | complex | standard | 12.5 μs | Disagreement |

The complete run samples and one full route result are in
[`heldout-local-windows-2026-11-12.json`](../evals/results/heldout-local-windows-2026-11-12.json).

## Local SLM routing and resources

Each model ran three four-task routes. All returned **4/4 valid answers in each
run**; there were no invalid replies, timeouts or fallbacks. The first request
was measured separately from the later warm requests. The first 0.5B and 1.5B
requests were made while those model IDs were absent from `ollama ps`; the 3B
model's residency immediately before its first timed route was not captured, so
its first-request time is not claimed as a verified cold start. Ollama's load
duration is not emitted by Preagent's route JSON; the first request duration
includes any model-loading time.

| Installed model | Size / quantization | First `itoa` request | Warm per-task median | SLM agreement | Hybrid agreement | Peak sampled Ollama host working set |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| `qwen2.5-coder:0.5b` | 494M, 397 MB, Q4_K_M, Apache-2.0 | 1,830 ms | 172–178 ms | 1/4 | 2/4 | 65 MiB |
| `qwen2.5-coder:1.5b` | 1.5B, 986 MB, Q4_K_M, Apache-2.0 | 8,659 ms | 176–181 ms | 3/4 | 3/4 | 344 MiB |
| `qwen2.5-coder:3b` | 3.1B, 1.9 GB, Q4_K_M, Qwen Research License | 4,603 ms* | 196–202 ms | 1/4 | 1/4 | 366 MiB |

`*` First observed request; cold status unverified. The warm per-task ranges show
the per-task medians across the repeated runs; exact values are in the raw JSON.
Median total process wall time across the two warm runs was 790 ms, 763 ms and
834 ms for the 0.5B, 1.5B and 3B models, respectively. The cold/first-request
process times were 2.45 s, 9.22 s and 5.28 s.

The sampled Ollama host working set is not model VRAM. Ollama reported each
tested model running **100% on GPU**, with allocations around 481 MB, 1.2 GB and
2.2 GB. One post-run `nvidia-smi` snapshot showed 29% utilization, 2,152 MiB of
4,096 MiB used and 1,811 MiB free; this is a point sample, not peak utilization.
Sampled Ollama process CPU time for a four-task route was 0.59–0.92 CPU seconds.
No reliable system-wide CPU percentage was captured. Memory samples can miss
brief peaks.

The installed `qwen2.5:7b` artifact is 4.7 GB (Q4_K_M). It was not run: at the
later snapshot only 0.45 GiB system RAM was free and the artifact exceeds the
GPU's 4 GiB VRAM. This avoids a high-risk memory-pressure experiment.

The 1.5B model raised hybrid label agreement by one case relative to rules on
this tiny set, but the 0.5B model tied rules and the 3B model scored lower. This
is agreement with subjective tier labels only, not coding correctness.

## Acceptance controls

The clean full checkouts matched the pinned manifest commits, had the declared
license files, and passed the acceptance tests. Each isolated seeded fixture
failed those tests as expected:

| Task | Clean pinned source | Seeded regression |
| --- | --- | --- |
| `itoa-decimal-pairs` | Pass | Expected acceptance failure |
| `ryu-negative-infinity` | Pass | Expected acceptance failure |
| `byteorder-endian-read` | Pass | Expected acceptance failure |
| `byteorder-signed-width` | Pass | Expected acceptance failure |

All seeded grade outputs had empty `infrastructure_detail`; they are test
failures, not build or dependency errors. SHA-256 snapshots of each candidate's
non-`.git` files were identical before and after grading. All original full
source checkouts remained clean at their pinned commits. Positive and negative
controls were completed before the partial-clone guard was added; the guard
does not change grading behavior.

## Coding-task correctness

**Not tested.** Claude Code and OpenCode were unavailable. Although Codex CLI and
local Ollama models were installed, starting a nested Codex coding-agent run
inside the active Codex session was not an available safe execution path. No
agent saw the frozen prompts, and no task was reported solved. There were no
paid or cloud inference calls.

## Harness issue and fix

The cached upstream repositories were Git partial clones with
`remote.origin.promisor=true` and `remote.origin.partialclonefilter=blob:none`.
`heldout prepare` attempted a local clone, after which Git tried to obtain
missing promisor objects and failed. This was an incomplete source checkout,
not a test failure. Full local clones allowed the controls to run.

To keep `prepare` reliably local-only, it now detects partial-clone Git
configuration before checking out or cloning and returns a clear instruction to
use a complete local checkout. A regression test covers that rejection, and the
held-out README now documents the complete-object requirement. Existing Windows
drive-path clone handling is included upstream at commit `5ab61e6`.

The frozen smoke corpus and held-out labels were not modified. No router rule,
threshold or task description was tuned against these results.

## Limitations and next milestone

- Four tasks are not enough to estimate general routing accuracy.
- These tasks are public seeded regressions; they are held out from router
  tuning, not necessarily from model training or agent familiarity.
- Coding-agent execution and correctness remain unmeasured.
- Host-memory sampling was periodic; GPU utilization was only a point sample.
- The 7B model was skipped for resource reasons, and no model was downloaded.
- This workstation report is separate from the GitHub-hosted CI measurements.

Next, expand the benchmark with independently selected, blindly graded tasks,
then compare unchanged agent defaults, rules-only routing and SLM-assisted
routing using a separately installed, fully local coding agent. Keep agent
correctness, routing agreement and overhead as separate outcomes.
