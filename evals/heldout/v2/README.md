# Held-out v2 candidate registry (NOT FROZEN)

This is a separately versioned **proposal** containing exactly 36 task contracts:
six classes × six pinned permissively licensed upstream Rust repositories.
These prompts and behavioral acceptance criteria are **candidate designs only**;
the proposed regressions are not claims of existing upstream bugs or resolved
issues. Do not expose this registry as a validated benchmark, use it for model
tuning, or report tier agreement against its pending reviewer labels.

Run locally, without any network or model call:

```sh
node evals/heldout/v2/check.cjs draft
node --test evals/heldout/v2/check.test.cjs
```

The `ready` mode is intentionally a failing gate until all requirements are
independently satisfied. It does not generate code, prepare fixtures, or run
an agent:

```sh
node evals/heldout/v2/check.cjs ready --json
```

## What is verified

- Six pinned SHA-1 Git commit IDs and relevant subcrate-level permissive
  license declarations were checked against upstream package manifests.
  Each task records the exact target package(s), not just the repository root.
- Named Rust source paths correspond to inspected upstream areas. This is
  not yet a full case-by-case behavioral validation.
- Each task has a concrete prompt, a seeding plan and two acceptance criteria.
- The structural validator enforces 36 unique cases, six per class, one case
  per class per upstream repository, no v1/smoke prompt reuse, and safe paths.
- The unchanged v1 and smoke dataset Git blob IDs are verified by Node tests.

## Candidate grading (experimental, NOT a benchmark result)

Sixteen cases with independently recorded green/red fixture controls now have a
local, non-agent grader. After preparing a **disposable** pinned upstream Git
checkout and repairing the controlled regression, use:

```sh
node evals/heldout/v2/grade.cjs tokio-timeout-deadline /path/to/pinned-candidate
```

The directory must be the root of the exact pinned Git checkout. The grader
copies the candidate into a temporary directory without Git metadata or
build output, replaces Cargo.lock with the checked-in pinned resolution,
injects acceptance tests **only into that copy** (including into a registered
existing integration-test target when a package disables automatic test
discovery), and runs targeted
`cargo test --offline --locked`. It returns JSON with
`acceptance_status` and `task_correctness: null`. Build errors, missing
cached dependencies, crashes, timeouts and missing tests are not counted as
failed behavioral acceptance tests. Pending cases without verified controls
are rejected. This tool never launches a coding agent or performs inference.

The separate `heldout-v2-grader-smoke.yml` GitHub Actions workflow validates
both clean-passing and seeded-failing execution using the real pinned Serde
checkout, while checking that grading leaves its candidate untouched.
The smoke workflow fetches upstream source and dependencies; unlike the local
grader, it is not an offline preparation command.

**Security:** Copying is not process sandboxing. Cargo build scripts and
tests execute candidate code with the current user's privileges; Cargo's
`--offline` option restricts dependency fetching but does not prevent
arbitrary network or filesystem activity by that code. Run only reviewed
fixtures in a disposable, appropriately isolated environment. The v2 set
remains unfrozen, and these checks are not eligible for model-performance
claims or label tuning.

## Blocking conditions before a genuine freeze

1. Verify package-specific **transitive dependency license scope**, feature
   sets and Rust toolchain for each pinned revision in an isolated checkout.
   A root-package permissive license is not evidence for every dependency.
2. Convert each proposed task into a reviewed, tested seed patch and an
   **independent executable acceptance test**. Prefer actual historical fixes
   with verifiable issue/commit references over synthetic regressions. If a
   synthetic regression is retained, identify it explicitly as such.
3. In a clean complete local clone at the pinned SHA, confirm acceptance
   tests PASS before seeding and FAIL after seeding. Record CI run IDs, exact
   commands and fixture hashes; infrastructure failures are not test failures.
4. Obtain at least two independent human complexity-tier reviews for each
   case, using blinded prompts and acceptance criteria. Preserve both raw
   judgments, including disagreements. Do not invent reviewer IDs or labels.
5. Commit immutable prompt/fixture hashes and test an evaluator that adds
   withheld acceptance tests **only after** the model session ends.
   Public GitHub test files are not truly secret, so label the protocol
   withheld-at-execution rather than claiming model-training secrecy.
6. Obtain human signoff and move to a separate `frozen` manifest revision.
   A freeze must fail if any gate is unmet; do not relabel to increase scores.

**Current evidence (November 15, 2026):** 16 of 36 v2 tasks have seed patches,
executable acceptance tests and verified green-clean/red-seeded controls in
GitHub Actions. **Twenty other candidates still lack a complete controlled
fixture.** No v2 model runs, SLM classification, task-repair outcomes or
independent complexity reviews exist. See the
[latest control evidence](../../../docs/heldout-v2-controls-2026-11-15.md),
the [November 14 control snapshot](../../../docs/heldout-v2-controls-2026-11-14.md),
and the [pinned license-inventory findings](../../../docs/heldout-v2-license-inventory-2026-11-14.md).

For genuine human reviews, use the [blind review packet](review/packet.md),
[submission checker](review/check-submission.cjs), and
[review coordination issue #2](https://github.com/MrPunyapal/preagent/issues/2).
No one has been assigned a fabricated label or approval.

This candidate registry must remain unfrozen until the remaining independent
reviews, license audits, verified fixture controls and content lock are done.

All existing `evals/heldout/v1.tsv`, its fixtures/results, and
`evals/baseline.tsv` remain unchanged. No production Rust router was edited.
