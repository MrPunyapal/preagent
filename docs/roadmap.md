# Development roadmap

Build features only when they improve outcomes over a measured baseline.

## Milestone 1: Dependency-free Rust baseline

- CLI for local routing and transparent Codex execution.
- Bounded Git metadata access, never repository code contents or telemetry.
- Explainable intent, heuristic tier and advisory effort.
- JSON route output and true dry-run.
- User-owned overrides, optional local-provider flags, original prompt preserved.
- Unit tests, CLI integration tests, cross-platform CI.

Acceptance: all checks pass on Linux, macOS, and Windows; prompts never
pass through a shell; unsupported options fail clearly.

## Milestone 2: Evaluation harness

- A four-case frozen held-out pilot is separate from the smoke corpus.
- Pinned repositories, license metadata, seeded patches, and hidden acceptance tests.
- Offline local fixture preparation, isolated grading and explicit SLM comparisons.
- Router and human labels must not be tuned using the pilot outcomes.
- Expand to independently selected tasks and repeated agent runs before claiming performance improvements.


- Extend the existing labeled routing smoke corpus with independently held-out tasks.
- Compare agent defaults, rules-only routing, candidate SLM, and hybrid.
- Track task correctness, prompt intent, user overrides, CPU/RAM and latency.
- Report uncertainty, hardware details and reproducibility.

Acceptance: an independent contributor can reproduce the comparison.

### Held-out v2 candidate preparation (not frozen)

- A separate 36-case candidate registry exists at `evals/heldout/v2/`.
- All six source revisions and 12 package-level licenses have been checked;
  historical issue selection, dependency-scope audits and executable controls
  are still pending.
- Freeze requires 36 verified seed/acceptance pairs, independent human
  reviewer labels (disagreements preserved), and signed content hashes.
- No model or routing outcome may be reported against the candidate set.
- See `docs/heldout-v2-preparation-2026-11-13.md` for specific blockers.

## Milestone 3: Optional local SLM

- Evaluate compact, permissively licensed quantized models.
- Invoke a local SLM only when deterministic signals are insufficient.
- Validate outputs; fall back to rules on errors or latency limits.
- No model downloads or network access without explicit setup.
- Benchmark the now-available opt-in Ollama adapter before considering embedded llama.cpp.

Acceptance: SLM provides measurable value on held-out tasks or stays optional.

## Milestone 4: Real model selection and more agents

- Discover available models and effort levels from supported agent interfaces.
- Respect configured constraints, available models and permissions.
- Add OpenCode and Claude Code adapters with supported protocols.
- Preserve terminal behavior, sessions, signals, approval policy and errors.

Acceptance: validated routing works on both local and optional cloud agents.

## Milestone 5: Optional minimal prompt context

- Treat repository content as untrusted data, never as instructions.
- Preserve original prompt; additions must be opt-in and inspectable.
- Measure impact on coding outcomes, tokens, and latency.
- Do not keep the feature if performance or intent preservation regresses.
