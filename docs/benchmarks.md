# Benchmark status and limitations

The checked-in routing corpus is a hand-labeled smoke dataset and does not
measure coding-agent task success. Its accuracy score cannot establish that
a local SLM is better than deterministic rules or that selecting a different
cloud model reduces costs. Do not use it as a marketing benchmark.

## Reproducible next experiment

1. Build a held-out task set from real repositories with licenses permitting
   public evaluation. Keep tuning tasks separate from evaluation tasks.
2. Run Preagent without the SLM and capture deterministic routing output.
3. Run with --slm and --slm-always against an explicitly installed local
   model, recording model name, quantization, inference runtime and hardware.
4. Include time-to-first-response, total wall time, peak RAM, CPU usage,
   invalid model replies, fallback frequency, tier agreement and operator
   overrides. Do not count subprocess success as solved coding work.
5. Use the same task fixtures, agent versions and tool permissions to evaluate
   rule-only, SLM-assisted and unchanged Codex/OpenCode baselines.
6. Grade correctness with tests and predeclared acceptance criteria, manually
   reviewing cases where tests alone are inadequate.

The local SLM and live coding-agent evaluation require machines with the
respective runtimes and any credentials configured by their owners.
No cloud credentials are required by Preagent itself.
