# Held-out v2 AI pre-review, 15 November 2026

Two AI subagents independently inspected the task registry, fixture controls,
pinned source checkouts and declared dependency-license inventories. This is a
technical pre-review only. It is **not** an independent human task review,
legal advice or license approval, and it does not satisfy any freeze gate.

## Registry and fixture findings

- The UTF-8 regex control artifact had been attached to
  `tokio-bounded-backpressure-tests` instead of
  `regex-utf8-boundary-tests`. Commit `b518eac` corrected the manifest: Tokio's
  incomplete fixture is null again, and the regex case points to its
  successful clean/seeded artifact. The underlying Actions run executed all
  17 complete fixtures successfully; the corrected registry now agrees.
- `serde-derive-alias-coverage` points to `serde_derive/src/de.rs`, which has no
  inline tests. The existing alias test is in
  `test_suite/tests/test_annotations.rs` (`fn alias`), and does not exercise
  the proposed default interaction. Its target package and acceptance criteria
  need redesign before a controlled fixture can be claimed.
- `clap-alias-validation-tests` points to `clap_builder/src/parser/validator.rs`,
  which has no inline tests for the proposed behaviors. Existing required-group
  and subcommand-alias tests are in the repository's top-level
  `tests/builder/` tree, outside the declared `clap_builder` package. Its target
  package and seed plan need redesign before fixture work.
- `serde-core-derive-visitor-boundary` and `regex-unicode-flag-crossmodule`
  each name two target packages. Both `control-runner.cjs` and `grade.cjs`
  currently require exactly one target package. These proposals need a
  single-package scope or deliberate multi-package harness support before
  controls can run.

These findings concern incomplete proposals. They are not recorded as verified
benchmark labels or successful fixtures.

## Dependency-license inventory findings

The six pinned inventory records, package scopes and lock hashes are internally
consistent. They are metadata inventories for 12 package scopes, not license
approvals. The collector filters to all features on
`x86_64-unknown-linux-gnu`; other platform-specific dependencies and copied or
generated assets need separate review. Its simple expression detector flags
some slash-form MIT/Apache declarations, `MIT-0`, and `LLVM-exception` forms for
manual inspection. These warnings do not establish violations, and a zero
warning count is not approval.

The license workflow now supports manual dispatch and reruns when the candidate
package/revision manifest or pinned lock catalog changes. Its next run should
confirm the trigger and inventory behavior.

## Human work still required

GitHub review coordination [issue #2](https://github.com/MrPunyapal/preagent/issues/2)
is open without assignees or reviewer comments. Every task still needs two
separately recorded human complexity reviews, and each of the 12 package scopes
needs a human license audit. The registry remains unfrozen until those reviews,
the remaining fixtures, and the content lock are complete.
