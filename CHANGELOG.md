# Changelog

Preagent follows semantic versioning for its command-line interface, with
experimental features explicitly identified. This file describes the
unpublished v0.1.0 release candidate; no GitHub release exists yet.

## 0.1.0 (release candidate, not yet published)

### Included

- Rust CLI with deterministic, explainable simple/standard/complex task tiers.
- Optional local Ollama classification and local registered-model discovery;
  both are opt-in and never download a model.
- Native Codex, Claude Code and OpenCode launch commands with explicit
  agent/model/effort overrides and unchanged prompt forwarding.
- Explicit Codex `--effort none` for models that do not support reasoning;
  no effort override is applied by default.
- Exact prompt input from a positional argument, bounded UTF-8 stdin or a
  local UTF-8 prompt file. File input leaves interactive stdin available.
- `doctor`, JSON output for machine-readable inspection, and a routing
  regression smoke corpus.
- Separate pinned regression-control tooling for coding-task evaluations.
  The v2 candidate dataset is not frozen and has no model-performance scores.
- Windows x64, Linux x64 and macOS ARM64 CI binary archives and SHA-256
  checksums, with version and completeness gates before publication.

### Deliberately not included

- Automatic model choice, automatic effort overrides, or hidden prompt changes.
- A bundled small language model, model downloads, or hosted inference.
- Verified agent performance improvements or a frozen v2 benchmark.
- Code signing, notarization, a package-manager installation, or support for
  platforms outside the three currently tested binary targets.

Read [release readiness](docs/release-readiness.md) for known compatibility
limits and required final checks before the first tag.
