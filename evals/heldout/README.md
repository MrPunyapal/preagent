# Held-out pilot v1: frozen evaluation tasks

This pilot is **separate** from `evals/baseline.tsv`. These labels and acceptance
tests were defined in advance; never tune the router using these evaluation
outcomes, relabel tasks to make accuracy look better, or fold these cases into
the 45-case smoke corpus.

All four tasks are **controlled, seeded regressions** in genuine open-source
Rust codebases. They are not claims of real upstream bugs. The original upstream
commits and licenses are pinned below and in `v1.tsv`. Only our small test cases
and diffs are included here; source code remains in the upstream repositories.

| Task | Repository / license | Pinned commit | What must be repaired |
| --- | --- | --- | --- |
| `ryu-negative-infinity` | [dtolnay/ryu](https://github.com/dtolnay/ryu), Apache-2.0 OR BSL-1.0 | `75f9e707e77c89717ffb9c2a01fe2c28a016cc82` | Restore sign of negative infinity for f32 and f64, preserve NaN and finite formatting |
| `itoa-decimal-pairs` | [dtolnay/itoa](https://github.com/dtolnay/itoa), MIT OR Apache-2.0 | `9339ca99a43c550b5ee36e2afe524d1c90413461` | Restore exact decimal formatting across signed/unsigned boundaries, without heap allocation |
| `byteorder-endian-read` | [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder), Unlicense OR MIT | `5a82625fae462e8ba64cec8146b24a372b4d75c6` | Correct network-byte-order u16 decoding while preserving other widths and LE |
| `byteorder-signed-width` | [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder), Unlicense OR MIT | `5a82625fae462e8ba64cec8146b24a372b4d75c6` | Restore signed 64- and 128-bit partial-width sign extension across both byte orders |

License declarations are from each pinned repository's `Cargo.toml` and license
files. These licenses permit a local derivative used for benchmarking subject
to their terms. Do not publish changed upstream fixture checkouts as releases.

## Reproducibility / isolation

1. Clone each **upstream** repository manually, or use an already cached local
   checkout. The harness itself never clones remote URLs or downloads weights.
2. Use `preagent heldout prepare <task-id> <local-upstream-checkout> <new-output-dir>`
   to make an isolated local clone pinned to the required commit and apply the
   checked-in regression patch. It refuses to overwrite an output directory.
3. Confirm the acceptance tests fail on the seeded checkout **before** asking a
   coding agent to repair it: `preagent heldout grade <task-id> <output-dir>`.
   A failure here is an expected negative control; an infrastructure error
   is *not* a meaningful negative control.
4. Ask each agent to solve the task on a separate prepared checkout using
   exactly the frozen prompt from `v1.tsv`. Do not show the acceptance
   test source to the agent or let the router read it. Use the same permissions,
   task prompts, model availability and execution limits per condition.
5. Grade only after the attempt is complete. Grading copies the candidate to a
   disposable directory, injects the hidden acceptance test, and runs
   `cargo test --offline --test preagent_holdout`. Candidate files are not changed.
6. On a **clean original upstream checkout**, the same acceptance tests should
   pass (positive control). Verify the control before reporting measured
   task success; reject a task if it is already green after seeding.
7. Record each task's candidate commit + diff, agent/model/version, seed SHA,
   environment, elapsed time, resource usage, tool permissions and outcome.
   Process exit codes alone are not sufficient evidence of correctness.

`cargo test --offline` can fail when Rust dependencies are not already
cached. This is an **infrastructure blocker**, not evidence that an agent's code
was incorrect. The harness does not install packages or contact crates.io.

### Local-tier comparison

Run `preagent heldout route --json` for deterministic tier agreement and
`preagent heldout route --json --slm <already-installed-model>` to compare
local SLM and safety-floor hybrid decisions on the same frozen prompts.
Record model quantization, Ollama/runtime version and machine details from
your local environment. This mode never executes any coding agent.

The four task labels are subjective complexity judgments, not model
capability ground truth. The small set is an **engineering pilot** only.
It cannot establish statistical generalization, economic savings, or relative
coding-agent quality. Expand later with independently selected, truly held-out
cases and multiple languages/repositories; use blind grading and repeated
runs before deciding that routing is useful.

### Outcome definitions

- **Tier agreement:** Router suggestion equals the frozen human tier label.
- **SLM agreement:** Local SLM suggestion equals the frozen label; missing/
  invalid SLM responses are reported separately, not silently scored as hits.
- **Hybrid agreement:** Safety floor combining rules and SLM agrees with label.
- **Coding-task correctness:** Hidden acceptance tests pass in a candidate
  worktree after the agent attempt and both controls are verified.
- **Missing outcome:** No agent run, no grade, unavailable Rust dependencies,
  or invalid controls. Never treat this as a successful completed task.

The manifest is held separate from the smoke corpus and router. It is not
an instruction to alter routing thresholds based on evaluation results.
