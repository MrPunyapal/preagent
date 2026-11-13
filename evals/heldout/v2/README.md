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

**No v2 agent runs, SLM classification, seed patches or hidden tests exist yet.**
The purpose of this registry is to make requirements explicit without
fabricating successful controls or independent reviews.

All existing `evals/heldout/v1.tsv`, its fixtures/results, and
`evals/baseline.tsv` remain unchanged. No production Rust router was edited.
