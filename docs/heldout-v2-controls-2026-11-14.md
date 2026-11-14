# Held-out v2 verified controls, 14 November 2026

**Status: candidate, not frozen.** Six of 36 contracts currently have
executable controlled regressions and independent acceptance tests.
Thirty proposals still lack seed patches, tests, and control runs.
There have been no v2 model or coding-agent attempts or scores.

[Actions run 43582497392](https://github.com/MrPunyapal/preagent/actions/runs/43582497392)
succeeded at `345d1e899295b9929d20b3f2bfac9e3443778a74` on the Ubuntu hosted runner.
All six matrix jobs proved the test is green on an unchanged pinned
upstream clone and red after the specified controlled regression.
These are **fixture controls**, not agent-completed repairs.

| Task | Control job | Clean | Seeded |
| --- | --- | --- | --- |
| `tokio-timeout-deadline` | [job 132410009384](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009384) | Pass | Fail as expected |
| `serde-numeric-range` | [job 132410009319](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009319) | Pass | Fail as expected |
| `ripgrep-escaped-glob-literal` | [job 132410009351](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009351) | Pass | Fail as expected |
| `regex-escaped-class-range` | [job 132410009273](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009273) | Pass | Fail as expected |
| `clap-option-terminator` | [job 132410009125](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009125) | Pass | Fail as expected |
| `hyper-chunked-extensions` | [job 132410009327](https://github.com/MrPunyapal/preagent/actions/runs/43582497392/job/132410009327) | Pass | Fail as expected |

The source revisions are pinned in `evals/heldout/v2/candidates.json`.
Each matrix job uploads a `v2-controls-<task-id>` JSON artifact containing
pinned source SHA, SHA-256 of seed and acceptance sources, resolved Cargo
lockfile SHA-256, test results, and tail of compiler output. Artifacts are
ephemeral workflow evidence; no claim is made about their permanent retention.
No model saw acceptance-test source. For Hyper, which needs internal
`Decoder` access, the acceptance module is appended only in the temporary
control checkout. A future agent-run grader must withhold it until grading.

**Reproducibility qualifier:** the upstream Tokio, Serde, regex, and Hyper
repos do not commit `Cargo.lock` at the recorded source revision. GitHub
Actions resolved dependencies on the day of control execution and included
lockfile hashes in artifacts. These controls are measured for that resolution;
the dependency lockfiles must be pinned and reviewed before v2 can be frozen.
The ripgrep and clap pins do include `Cargo.lock`.

The `evals/heldout/v2/review/packet.md` packet requests two **real,
independent human** complexity ratings per task, with disagreements preserved.
Neither the independent reviews nor signoffs exist. A declared-license
inventory also exists but must not be mistaken for a full dependency/license
audit. The original v1 files, smoke corpus, and production routing are
untouched.

Prior control evidence: [three-task initial Actions
run 43581079657](https://github.com/MrPunyapal/preagent/actions/runs/43581079657)
and [five-task Actions run
43582224029](https://github.com/MrPunyapal/preagent/actions/runs/43582224029).
The six-task run above is the complete current set.
