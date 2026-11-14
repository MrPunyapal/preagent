# Held-out v2 verified control evidence — November 14, 2026

The v2 candidate set remains **unfrozen**. This record is limited to three
verified seeded regressions. The other 33 task proposals still lack executable
seed/acceptance tests and green/red control evidence. No models were evaluated.

[Actions run 43581079657](https://github.com/MrPunyapal/preagent/actions/runs/43581079657)
completed successfully on commit `604cfe19ab918d0c8d12a5172d980e97ebb9c332`, on an Ubuntu-hosted runner.
The reusable GitHub Actions control runner cloned each pinned upstream
repository, inserted the withheld acceptance test into the control checkout,
ran the clean test, applied a checked seed patch and reran the same test.

| Candidate task | Verified green/red job | Result |
| --- | --- | --- |
| `serde-numeric-range` | [job 132405732662](https://github.com/MrPunyapal/preagent/actions/runs/43581079657/job/132405732662) | Clean **passed**, seeded **failed** as intended |
| `clap-option-terminator` | [job 132405732848](https://github.com/MrPunyapal/preagent/actions/runs/43581079657/job/132405732848) | Clean **passed**, seeded **failed** as intended |
| `ripgrep-escaped-glob-literal` | [job 132405732889](https://github.com/MrPunyapal/preagent/actions/runs/43581079657/job/132405732889) | Clean **passed**, seeded **failed** as intended |

Raw per-task JSON evidence, including seed/test SHA-256, exact pinned source
commit, Cargo lockfile SHA-256, compiler output tails, and observed test status
is attached to the workflow as `v2-controls-<task-id>` artifacts. These CI
artifacts are not claimed to be immutable permanent archives.

Serde's pinned upstream repository has **no committed Cargo.lock**. The CI
control generated a dependency resolution and recorded its hash as ephemeral
evidence. This must be separately pinned and reviewed before the task can be
certified reproducible as a frozen benchmark. The other two pinned upstream
repositories have committed Cargo.lock files.

These control results are not agent coding success and do not establish model
routing quality. The candidate manifest still requires independent reviewers,
dependency/license-scope audit, and a final freeze lock. The existing v1
benchmark and 45-case smoke corpus remain unchanged.
