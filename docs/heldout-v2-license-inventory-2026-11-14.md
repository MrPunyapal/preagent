# Held-out v2 pinned-dependency license inventory, 14 November 2026

**Status: collected upstream declarations only. No dependency-license
approval or legal opinion.** The pinned-source inventory workflow uses Cargo
metadata on a GitHub-hosted Linux runner with all crate features and
`x86_64-unknown-linux-gnu` filtering. Each package's dependency closure
records version, declared SPDX expression, source and manual-review warnings.

[Workflow run 43581601670](https://github.com/MrPunyapal/preagent/actions/runs/43581601670)
completed successfully after CI validation at
`a7444cece70cc2fb05902b9ae9ad6ec6252b6b61`.
All **six repository jobs passed**, covering 12 distinct target package scopes.
For each repository the uploaded artifact `v2-declared-license-<repo>`
contains per-package inventory JSON and the resolved Cargo.lock snapshot.

| Repository | Scoped packages | Automatic expression warnings | Pinned upstream Cargo.lock? |
| --- | ---: | ---: | --- |
| Tokio | 1 | 1 | No |
| Serde | 2 | 0 | No |
| ripgrep | 4 | 2 | Yes |
| regex | 3 | 0 | No |
| clap | 1 | 1 | Yes |
| Hyper | 1 | 1 | No |

Warning counts are from the **inventory program's simple-expression detector**,
not license violations, and a zero count does not amount to legal approval.
This scope covers a specific target platform and all declared Cargo features
for the pinned revisions. Other platforms and generated native dependencies
are not automatically included. The record does not establish attribution
or inclusion obligations for copied upstream source or other assets.

A reviewer must inspect each uploaded dependency list and expressions,
evaluate transitives/build dependencies/target-specific assets for the target
benchmarks, resolve flagged declarations, confirm permitted redistribution
and attribution, and sign an audit for each target package. Store the
reviewer's actual identity, date, scope, findings and evidence links. Only
then may a package's `dependency_scope_status` be set to `verified`
in the v2 candidate manifest.

For repos lacking an upstream lockfile, snapshot/pin the generated lockfile
with a SHA-256 content lock **before** relying on stable execution. No
unreviewed status fields have been changed here. This workflow performs no
coding-agent or model inference and does not access the user's local PC.
