# Held-out evaluation pilot: 12 November 2026

## Scope and provenance

This is the first **frozen, separate** four-task held-out pilot for
[Preagent](https://github.com/MrPunyapal/preagent). It does not modify the
existing `evals/baseline.tsv` 45-case smoke corpus or its router labels.

- Frozen task definitions: [`evals/heldout/v1.tsv`](../evals/heldout/v1.tsv).
- Acceptance fixtures, controlled regressions, licenses and reproducibility:
  [`evals/heldout/README.md`](../evals/heldout/README.md).
- Harness: `preagent heldout list|route|prepare|grade`. No new runtime dependencies.
- CI routing measurements: [run 43202535719](https://github.com/MrPunyapal/preagent/actions/runs/43202535719),
  at commit `5974a0a052c5b1da4f8e5eb1877c7d82fc3a4480`.
- Full-platform CI after adding controls:
  [run 43203059153](https://github.com/MrPunyapal/preagent/actions/runs/43203059153).
- Positive and negative fixture controls:
  [run 43203059013](https://github.com/MrPunyapal/preagent/actions/runs/43203059013)
  at commit `dfcbd19a1dc971bf029a5a3150b26187963aed10`.

The cases are **seeded regressions in real permissively licensed projects**,
not claims about outstanding upstream defects. Each task pins the exact
upstream Git commit and its license. The task descriptions, frozen human
difficulty labels and hidden acceptance tests are independent of the
45 smoke examples. These public cases are held out **from router tuning**,
not guaranteed unseen in model training data.

## Frozen tasks and grading criteria

- `itoa-decimal-pairs`: `dtolnay/itoa`, MIT OR Apache-2.0, commit
  `9339ca99a43c550b5ee36e2afe524d1c90413461`. Restore decimal
  digit-pair formatting for zero-containing and boundary integers.
  Acceptance: exact signed and unsigned output, including i128 minimum.
- `ryu-negative-infinity`: `dtolnay/ryu`, Apache-2.0 OR BSL-1.0,
  commit `75f9e707e77c89717ffb9c2a01fe2c28a016cc82`. Restore
  the minus sign on negative infinity for f32 and f64. Acceptance:
  both infinities, NaN and finite values.
- `byteorder-endian-read`: `BurntSushi/byteorder`, Unlicense OR MIT,
  commit `5a82625fae462e8ba64cec8146b24a372b4d75c6`. Restore
  big-endian u16 decoding. Acceptance: BE and LE examples plus wider
  operations and writing/reading roundtrip.
- `byteorder-signed-width`: same pinned byteorder repository and
  license. Restore signed 64/128-bit variable-width sign extension
  across big- and little-endian paths. Acceptance: signed edge values,
  boundary widths and read/write roundtrips.

Each case has a separate checked-in seed patch and Rust acceptance test.
A local-only `prepare` command creates a new pinned clone from an
already available local checkout; `grade` copies a candidate to a
temporary directory and runs `cargo test --offline --test preagent_holdout`
with acceptance tests added only to that snapshot. No workspace is
modified by grading, no coding agents run, no models are downloaded,
and there are no paid API calls.

## Verified fixture controls

[Held-out controls CI](https://github.com/MrPunyapal/preagent/actions/runs/43203059013)
passed for **all four cases**:

- Clean pinned upstream source: acceptance tests **pass**.
- Deliberately seeded regression: the same acceptance tests **fail**.

This verifies that all four test fixtures distinguish clean and deliberately
broken code. It is not evidence of an agent having repaired any task.
The control workflow fetches only public source repositories and free
Cargo dependencies, then runs the grader offline.

## Routing results (no SLM invoked)

[Linux CI held-out route output](https://github.com/MrPunyapal/preagent/actions/runs/43202535719)
at `5974a0a` reported **2/4 agreement** between deterministic routing and
the four frozen human difficulty labels. Individual results:

- `itoa-decimal-pairs`: expected standard, rules standard; 42 μs.
- `ryu-negative-infinity`: expected simple, rules simple; 22 μs.
- `byteorder-endian-read`: expected standard, rules simple; 20 μs.
- `byteorder-signed-width`: expected complex, rules standard; 25 μs.

These numbers are **per-task, in-process classification durations**,
not process startup, model selection, inference or coding execution times.
The small sample is descriptive only; do not use it to claim a 50%
real-world routing accuracy or improvement over other tools.

Because the SLM was not requested:
- Local SLM: 0 calls, 0 valid replies; agreement **not measured**.
- Hybrid with no SLM: same rules-only 2/4; **not an SLM comparison**.
- Coding agent execution: 0 attempts.
- Coding-task correctness: **not measured** (`task_correctness: null`),
  not zero successful outcomes.
- No routing thresholds, source code or frozen labels were adjusted in
  response to these two disagreements.

The existing Windows report's 45-case smoke classification scores
(45/45 rules, 31/45 SLM, 35/45 hybrid) are a **different, previously
seen test set**. They must never be combined with this held-out pilot.

## Platform, version, timing and memory provenance

The held-out route result was collected on a **GitHub Actions Linux
x86_64** hosted runner (Ubuntu 26.04.1, Rust 1.99.1, toolchain
`58cf8f9c3`). The program reported a Linux `VmHWM` process peak
resident high-water mark of **2,720 KiB**. This excludes Cargo, any
coding agent, the Ollama server, CPU/GPU model weights and GPU VRAM.
No CPU model, total physical RAM or runner-specific CPU utilization
was recorded. A single execution is not a calibrated performance study.

The previous [Windows validation](windows-validation.md) recorded
Intel Core i7-12700H (20 logical processors), 15.63 GiB RAM,
RTX 3050 Laptop GPU (4 GiB VRAM), Rust/Cargo 1.96.1,
Git 2.45.2, Ollama 0.32.5 and local
`qwen2.5-coder:1.5b` Q4_K_M. That hardware and the earlier
model latency/memory measurements were from **11–12 November's prior
smoke-corpus validation**; the Windows machine was not accessed for
this new held-out run. Do not represent those older measurements as
newly measured held-out inference performance.

## Checks and limitations

- [Cross-platform CI](https://github.com/MrPunyapal/preagent/actions/runs/43203059153)
  succeeded on Linux, Windows and macOS, including rustfmt, Clippy with
  warnings denied, tests, locked release build, existing smoke evaluation,
  held-out manifest parsing and held-out rules-only classification.
- The measured Linux run reported **49 unit tests and 7 integration tests passed**.
- The independent positive/negative fixture-control workflow passed
  against the pinned upstream repositories.
- No fresh local SLM comparisons were possible here because the
  previously installed Windows model was not available to this runner.
- No paid model, subscription, cloud agent or live coding-agent task was
  invoked. Real task correctness, model-to-model comparisons, cold/warm
  inference latency, model memory, GPU use, agent token usage, retries,
  and agent execution cost remain **untested for this pilot**.
- Full offline grading needs a previously cached checkout and any
  required Cargo dependencies. Build/dependency failures are reported
  separately from acceptance-test failures.
- Only four planted regressions across three Rust repositories are
  included. Their public tests may be discoverable by a coding agent.
  They do not constitute a statistically representative or
  genuinely secret test set.
- A passing acceptance test after an agent attempt is still only one
  correctness signal. The workflow does not automatically certify that
  the agent preserved unrelated invariants, licensing or user intent.

## Reproducing and extending

With a prebuilt binary (or `cargo run --`) run:

```sh
preagent heldout list --json
preagent heldout route --json
preagent heldout route --json --slm qwen2.5-coder:1.5b
```

Only run the SLM form if that exact model is **already installed
locally** and Ollama is running. Record Ollama version, quantization,
hardware, warm/cold timing, process and model-memory use, and complete
per-task decisions. No model is downloaded by Preagent.

For each pinned local upstream checkout:

```sh
preagent heldout prepare itoa-decimal-pairs ../itoa ../itoa-seeded
preagent heldout grade itoa-decimal-pairs ../itoa-seeded
```

Confirm the negative control, perform an isolated agent attempt without
showing the acceptance test code, then grade its candidate. Keep an
unchanged-agent control, rules-only and SLM-assisted conditions on the
same frozen prompts, time limits, permissions, models, and source commits.
Repeat attempts and perform blind grading on an expanded data set before
using the results to drive model routing changes.

**Next milestone:** collect local SLM routing results using the model
already on the user's Windows machine, plus permissioned, non-paid
end-to-end coding-agent trials where available. Record full provenance
and keep outcomes separate from tier agreement. No hidden tuning against
these four frozen labels.


## Windows CI continuation after MSVC clone fix (12 November 2026)

This subsection records **GitHub-hosted Windows validation**, not a rerun
on the user's Windows 11 workstation. The user reports Visual Studio Build
Tools 2022, MSVC x64/x86 tools and Windows 11 SDK are now installed locally.
This installation was **not directly accessible or independently verified**
from this environment. No local files, credentials or pending Codex changes
were inspected or modified.

- [Windows rules-only CI run 43203684926](https://github.com/MrPunyapal/preagent/actions/runs/43203684926)
  passed compilation and held-out routing on Windows Server 2025
  x86_64 with Rust 1.99.1 (`x86_64-pc-windows-msvc`).
  Frozen tier-label agreement remained **2/4**. Per-task in-process
  times (manifest order) were **54, 32, 29 and 35 microseconds**.
  This is one CI-run sample, **not** inference latency or Windows
  workstation speed. Windows process memory was not measured.
  See [raw Windows routing data](../evals/results/heldout-ci-windows-2026-11-12.json).
- [Windows controls run 43206322120](https://github.com/MrPunyapal/preagent/actions/runs/43206322120)
  initially failed in fixture preparation: Git for Windows treated an
  absolute `D:\\...` clone-source path as an scp-style remote and emitted
  `hostname contains invalid characters`. That was a real harness
  portability defect, **not** an absent compiler or failed coding task.
- Commit [5ab61e6](https://github.com/MrPunyapal/preagent/commit/5ab61e69d37cc8c4085fc08f241e82def83e0568)
  fixes local-only clones by using source directory `.` and preserving
  the caller's absolute destination. It also adds a regression test.
  Commit [ef46927](https://github.com/MrPunyapal/preagent/commit/ef46927d060f8d0b69fff2d17a13f3e0f3327985)
  configures the Windows controls to rerun on held-out harness changes.
- [Windows controls run 43206977924](https://github.com/MrPunyapal/preagent/actions/runs/43206977924)
  **passed all four tasks**: for each task, the clean pinned upstream
  fixture passed its acceptance tests and the deliberately seeded
  regression failed them. This confirms positive/negative controls
  on a Windows MSVC runner. Exact task IDs and results are saved in
  [raw Windows control results](../evals/results/heldout-windows-controls-ci-2026-11-12.json).
- [Cross-platform CI run 43206978180](https://github.com/MrPunyapal/preagent/actions/runs/43206978180)
  passed on Linux, macOS and Windows, including formatting, Clippy,
  tests, release builds, and deterministic routing checks.

### Workstation evaluation still pending

The completed CI runs **do not** verify the newly installed local
Visual Studio Build Tools 2022 environment. No workstation SLM
classification calls were made in this continuation. No local model
was downloaded, and no paid/cloud agent was executed. The following
are still **not measured on the workstation** for the held-out pilot:

- MSVC x64 toolchain and Windows SDK executable discovery from the
  local Codex terminal; local `cargo build --release --locked`.
- Local deterministic and installed-SLM frozen-task routing under
  the workstation's current hardware, with repeated latency and
  memory measurements.
- Local positive/negative fixture controls after a successful build,
  including model/tool version provenance.
- Actual agent repair attempts and graded coding-task correctness.
  `task_correctness: null` means **not tested**, not failed.

The existing `docs/windows-validation.md` documents earlier **smoke**
measurements and cannot substitute for the new held-out work.
Neither the frozen labels nor the 45-case smoke corpus were changed.
