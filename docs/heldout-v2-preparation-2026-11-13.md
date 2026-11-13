# Held-out v2 preparation, 13 November 2026

## Status

**36 independently reviewable candidate contracts prepared; NOT FROZEN.**
The exact requested freeze cannot be honestly certified yet: 36 executable
seed/acceptance pairs, independent human reviewers, case-by-case control
evidence, and audited transitive dependency scopes are missing. This is a
task-design and gating deliverable, **not a benchmark result**.

All work is confined to `evals/heldout/v2/`, tests, documentation and the
normal CI workflow. Production Rust routing and the existing v1/smoke
fixtures, prompts, labels and results were not altered. No coding model,
local Ollama runtime, nested Codex or paid/cloud inference was invoked.

## What is prepared

- `evals/heldout/v2/candidates.json`: 36 concrete proposed task descriptions;
  one per task class per upstream Rust repository, with two explicit
  behavioral acceptance criteria and a controlled-regression seed plan.
- Six classes: bug fixes, refactoring, test improvements, security-sensitive
  fixes, multi-file changes and architecture changes; **six per class**.
- Six repositories: **six candidate cases per repository**, source paths
  and package-scope metadata at pinned Git SHA-1 commit IDs.
- `evals/heldout/v2/check.cjs` checks schema, balance, unique cases,
  safe relative paths, no v1/smoke prompt reuse, declared package scopes,
  expected source/test paths, reviewer records and control evidence.
- A candidate can be structurally valid without being freeze-ready.
  `ready` intentionally fails until every requirement is met.
- A future `frozen` status additionally requires `freeze.lock.json`
  with canonical text SHA-256 hashes for the manifest and fixture files
  plus at least two separately recorded human signoffs.
- `evals/heldout/v2/check.test.cjs` validates the draft and rejection
  paths. CI runs the draft checker and tests on Linux, Windows and macOS.

## Pinned sources and license evidence

| Repository | Commit | Pinned source/package license | Scopes |
| --- | --- | --- | ---: |
| [Tokio](https://github.com/tokio-rs/tokio) | `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0` | MIT | 1 |
| [Serde](https://github.com/serde-rs/serde) | `aa7913ad0211fef960df68e7812a6211145a888c` | MIT OR Apache-2.0 | 2 |
| [ripgrep](https://github.com/BurntSushi/ripgrep) | `178cc027e283fd965cb72d0a7395b219eada170c` | Unlicense OR MIT | 4 |
| [regex](https://github.com/rust-lang/regex) | `3d79fe92666a9f191d7677ebc123ac165eac200c` | MIT OR Apache-2.0 | 3 |
| [clap](https://github.com/clap-rs/clap) | `ec93e71255c0b833a34f50662a3fdb22f53f6ded` | MIT OR Apache-2.0 | 1 |
| [Hyper](https://github.com/hyperium/hyper) | `378f8479b8b703d098b46de1cec9ea648f12ea69` | MIT | 1 |

The 12 relevant Rust package manifest declarations were inspected at
these revisions (including Serde derive/core, ripgrep globset/ignore/
printer and regex syntax/automata/lite). GitHub's pinned recursive trees
also confirmed **all 48 listed source-path references exist**, and all six
initial package manifest paths exist. This verifies source locations and
first-party package license declarations, **not** every transitive
dependency or test-fixture license. That scope audit remains required before
acceptance into the frozen dataset.

## Independence, accuracy and methodological limits

- Selection uses proposed source areas and user-defined task classes, not
  the earlier routing or local-agent scores.
- Task prompts, acceptance criteria and seed plans are **proposals**. No
  claim is made that these are historical issues, real upstream defects,
  executable patches or independently confirmed fixes. Prefer dated
  upstream issues or merged fixes before final task selection.
- Independent complexity-tier reviewers have **not** participated. There
  are no invented reviewer identities, labels or consensus scores.
- The candidate manifest records full reviewer lists so disagreement can
  remain visible. Never tune labels against routing or model output.
- Hidden acceptance tests must be withheld from the agent at execution
  time. Tests later committed to a public repository are not guaranteed
  secret from model training and must not be represented as such.
- No v2 agent attempts, inference calls, tier agreement measurements or
  coding outcomes have been executed or reported.

## Specific blockers to a truthful freeze

| Freeze requirement | Completed | Pending |
| --- | ---: | ---: |
| Concrete task proposals | 36 | 0 |
| Pinned source paths existing | 48 references | 0 |
| Package-level license declarations | 12 scopes | 0 |
| Full transitive dependency/feature license audits | 0 | 12 scopes |
| Verified seed patches | 0 | 36 |
| Withheld executable acceptance tests | 0 | 36 |
| Clean-positive and seeded-negative controls | 0 | 36 pairs |
| Independent human complexity reviews | 0 | 36 tasks × 2 reviewers |
| Frozen content lock and human signoffs | 0 | 1 lock |

**External blocker:** Independent reviewer assignments and attestations
cannot be generated by the assistant. They must be collected from actual
reviewers before any label or benchmark score can be certified.
**Engineering work still required:** Validate proposals against upstream
behavior, create/verify 36 seed-and-test pairs, audit dependency scopes,
and implement a v2 isolated grader after deciding the correct crate-specific
feature and test invocation for each package. The existing four-case v1
grader is intentionally unchanged and remains tested.

## Reproduction

```sh
node evals/heldout/v2/check.cjs draft --json
node --test evals/heldout/v2/check.test.cjs
node evals/heldout/v2/check.cjs ready --json
```

`draft` succeeds only when the 36-case candidate registry is structurally
consistent; `ready` deliberately returns nonzero before acceptance, reviews,
controls and a content lock are complete. No command downloads models,
fetches external services or executes an agent. The checker does not modify
working trees.

The frozen v1 and smoke Git blob IDs remain:

- `evals/heldout/v1.tsv`: `1f8c1f7417220609b54a1a5c828f4794e4086760`.
- `evals/baseline.tsv`: `8c766652d528f6165ec51a63d0cd885b4fd36949`.

CI includes an explicit Windows CRLF normalization regression test for
those text-file blob comparisons.

## Local workspace and next step

This work was performed through the connected GitHub repository interface.
The assistant could not inspect the user's `E:\projects\packages\preagent`
worktree or the preexisting Git stash named
`preserve local Windows heldout clone fix before sync`.
Neither was modified or discarded.

**Next concrete step:** Independently review and revise the 36 proposals,
then implement one complete seed/hidden-test/green-red control path against
a pinned local checkout with a validated offline Cargo invocation.
Scale the verified procedure to the remaining cases, conduct the package
dependency/license audit, collect two real human labels per case, and
only then generate the immutable freeze lock. Do not run models or score
this draft before that gate passes.
