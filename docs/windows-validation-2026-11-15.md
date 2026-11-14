# Windows validation — 2026-11-15

Validation was performed on `main` at `736bef16f2c3c342ad843e5cac0d5084043d46f9`.
This report records local results and separates routing labels from coding
outcomes. No production Rust code changed during this validation.

## Workstation and installed tools

- Windows 11 Home Single Language, version 10.0.26300.
- Intel Core i7-12700H; 16,005 MiB visible system memory (16 GiB installed).
- NVIDIA GeForce RTX 3050 Laptop GPU, 4,096 MiB VRAM.
- Rust/Cargo 1.96.1; Node.js 24.22.2; Git 2.45.2 for Windows.
- Codex CLI 0.154.0. Claude Code, OpenCode, and Aider are not installed.
- Ollama 0.32.5 with `qwen2.5-coder:0.5b`, `qwen2.5-coder:1.5b`,
  `qwen2.5-coder:3b`, and `qwen2.5:7b` registered locally.
- `preagent models --json` listed all four models and correctly reported
  `offline_execution_verified: false`; registry discovery alone does not
  prove offline execution.

`qwen2.5-coder:1.5b` reports Q4_K_M quantization, tool capability, and Apache
License 2.0 through the installed Ollama metadata. No model was downloaded.

## Rust and CLI validation

All requested local commands passed on this checkout:

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --all-targets -- -D warnings` | Passed |
| `cargo test --all-targets --locked` | 55 unit + 8 integration tests passed |
| `cargo build --release --locked` | Passed |
| `cargo run --locked -- doctor --json` | Git, Codex, and Ollama available; Claude/OpenCode unavailable |
| `cargo run --locked -- eval --json` | 45/45 smoke labels agreed; not a coding-success measure |

The 11 SLM unit tests also passed, covering model discovery, malformed JSON and
HTTP framing, oversized responses, unavailable servers, chunked framing, and a
trickling response bounded by the total timeout.

Codex dry-run arguments matched the installed CLI's `--oss`,
`--local-provider`, `--model`, and `-c` options. Dry runs for Codex, Claude,
and OpenCode preserved a prompt containing quotes, Unicode, a newline, and
shell-looking text as one prompt argument. No dry run started an agent.
OpenCode rejected `--effort` as unsupported; without that option, it generated
the documented `mini --model ollama/... --prompt ...` command. Its executable
was unavailable, so real OpenCode behavior remains unverified.

A read-only, ephemeral Codex CLI execution explicitly selected Ollama and the
installed 1.5B model. It exited 1 before completing the request. Codex could
not parse Ollama's OpenAI-compatible model-list response as its expected
`models` response, then rejected the model because it "does not support
thinking." A previous Preagent-launched Codex TUI attempt with the installed
0.5B model failed on the same runtime limitation. The disposable repository
remained clean, and its README stayed unchanged. No paid API was used. This is
an incompatibility in this Codex/Ollama/model combination; no Preagent agent
argument defect was demonstrated.

## Local Ollama classification

The first 1.5B classification call for `Fix README typo` took 7,658 ms wall
time including model load and returned simple. Free system memory measured
2,013 MiB before loading and 1,202 MiB afterward; `nvidia-smi` then reported
1,193 MiB VRAM in use, with Ollama showing the model fully on GPU at about
1.2 GB. Subsequent warm prompt wall times were 1,189 ms for a profile-page
task and 873 ms for a critical authentication vulnerability task. After these
probes, free RAM was about 1,190 MiB. These are point-in-time samples, not
continuous peak measurements.

On the separate frozen four-task v1 routing pilot, the installed 1.5B model
returned valid tiers for all four cases. Agreement was rules 2/4, SLM 3/4,
and hybrid 3/4. Individual SLM times were 1,071, 969, 903, and 927 ms. All
four records kept `task_correctness: null`; no coding agent repaired these
tasks during this routing comparison. The sample is too small to claim a
routing benefit and was not used to tune the router.

An explicitly unavailable model returned Ollama HTTP 404, set SLM status to
unavailable, retained the rules-based complex recommendation, and exited 0.
No model pull was attempted.

## v2 grader and benchmark state

All five held-out v2 Node test files passed on Windows: 28 tests total. The
seven grader tests included rejection of external symlinks, materialization
of bounded in-tree file/directory symlinks, cycle handling, snapshot
exclusions, candidate-root and pinned-commit checks, and acceptance injection
into an isolated grading copy.

The full grader smoke executes Cargo against upstream source and dependencies.
It was not run directly on the Windows host because Windows Sandbox and
container runtimes were unavailable; the installed WSL2 distro has neither
Node.js nor Cargo. GitHub Actions successfully ran the isolated pinned Serde
smoke on an ephemeral Ubuntu runner: clean acceptance passed, seeded acceptance
failed as expected, and the original candidate checkout remained unchanged
([workflow run](https://github.com/MrPunyapal/preagent/actions/runs/43595214733)).
The six candidate-control jobs and current cross-platform CI run also passed
on the latest commit ([CI run](https://github.com/MrPunyapal/preagent/actions/runs/43595268135)).

The v2 registry remains a candidate, not a frozen benchmark: it has 36 cases
across six repositories and six task classes. After this local validation,
the expanded controls workflow verified 16/36 clean-positive and
seeded-negative controls (see [the follow-up evidence](heldout-v2-controls-2026-11-15.md));
20 tasks still lack complete fixture controls. The 12 package-level
dependency/license scopes still require human audits, and the candidate tasks
still require two independent human complexity reviews each. No v2 model or
coding-agent scores were produced. Do not use the candidate set for scoring or
router tuning until its gates are satisfied.

## Workflow history and remaining limitations

Earlier grader checks exposed two defects: the snapshotter needed to
materialize safe in-tree upstream symlinks, and the directory-cycle test
needed to prepare its scratch destination. Those fixes are present on this
checkout; the Windows grader tests and latest CI pass.

No source-code bug requiring a production change was confirmed in this
validation. Claude Code and OpenCode real execution remain unverified because
they are not installed. Codex local execution remains blocked by the runtime
compatibility error above. Full v2 benchmark readiness remains blocked on
fixture work, dependency/license audits, and genuine independent human review.
