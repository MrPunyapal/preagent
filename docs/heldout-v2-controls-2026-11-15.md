# Held-out v2 fixture controls, 15 November 2026

**Status: candidate, not frozen.** The expanded control run verified 16 of 36
fixture pairs. It did not run a coding agent or score a model.

Commit `cb08042fc7e43f5b5510bcfcfab0b84e3aa44df4` added a
`workflow_dispatch` trigger. Its push-triggered run selected all 16 candidates
with both seed and acceptance files; future manual dispatches can rerun all
complete fixtures without editing benchmark files. The complete
[GitHub Actions run](https://github.com/MrPunyapal/preagent/actions/runs/43630854042)
succeeded: every clean control passed its acceptance tests and every seeded
control failed them as expected.

| Task | Job | Raw artifact |
| --- | ---: | ---: |
| `clap-argv-literal-safety` | [132557104187](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104187) | [14127682395](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127682395) |
| `clap-help-layout-refactor` | [132557104223](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104223) | [14128297284](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128297284) |
| `clap-option-terminator` | [132557104190](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104190) | [14127833412](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127833412) |
| `hyper-ambiguous-framing-rejection` | [132557104212](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104212) | [14127698759](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127698759) |
| `hyper-chunked-extensions` | [132557104169](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104169) | [14127608834](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127608834) |
| `hyper-header-parse-refactor` | [132557104204](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104204) | [14128132985](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128132985) |
| `regex-escaped-class-range` | [132557104230](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104230) | [14128061895](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128061895) |
| `regex-meta-search-consistency` | [132557104193](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104193) | [14128121823](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128121823) |
| `regex-regex-size-limit` | [132557104220](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104220) | [14127708914](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127708914) |
| `ripgrep-escaped-glob-literal` | [132557104210](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104210) | [14128510082](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128510082) |
| `ripgrep-ignore-precedence` | [132557104194](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104194) | [14127658707](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127658707) |
| `serde-deny-unknown-fields` | [132557104266](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104266) | [14127778499](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14127778499) |
| `serde-numeric-range` | [132557104255](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104255) | [14128306244](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128306244) |
| `tokio-oneshot-cleanup` | [132557104283](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104283) | [14128108111](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128108111) |
| `tokio-process-tree-cleanup` | [132557104275](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104275) | [14128371024](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128371024) |
| `tokio-timeout-deadline` | [132557104248](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/job/132557104248) | [14128212635](https://github.com/MrPunyapal/preagent/actions/runs/43630854042/artifacts/14128212635) |

Each artifact records the pinned upstream revision, package scope, lockfile
hash, seed and acceptance-source hashes, and clean/seeded test output. The
artifact records were downloaded and checked against the repository. Text
fixture hashes were compared after normalizing the Windows checkout's CRLF
line endings to the canonical Git content.

The candidate manifest now points to this run for all 16 verified controls.
Twenty proposals still lack a complete verified seed/acceptance pair and
clean/seeded evidence. All 36 proposals still need two genuine independent
human complexity reviews, preserving disagreements. The 12 dependency scopes
still need human license audits. Do not score models, tune routing, or freeze
the candidate based on these controls.
