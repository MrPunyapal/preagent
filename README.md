# preagent

An offline-first, open-source pre-agent for coding assistants.

**Status: initial Rust baseline.** This release provides deterministic, explainable
task classification and a safe Codex CLI pass-through. It is **not yet** an
automatic model router or a built-in SLM. We will add those features only
after benchmarking the rules-only baseline.

## Principles

- Free and open source (Apache 2.0).
- Zero runtime dependencies for the current baseline.
- Local classification without API keys, network calls, or telemetry.
- The developer's prompt is forwarded unchanged as a single process argument.
- No unverified model IDs, automatic provider changes, or hidden prompt rewrites.
- Native Codex approvals, authentication, and sandbox configuration stay in control.
- Future local inference will be optional; rules will work without a model.

## Build

Install Rust 1.74+ and Cargo. Build and test with:

    cargo build --release
    cargo test --all-targets --locked

## Usage

Classify a request with no AI service:

    cargo run -- route "Fix authentication redirect"
    cargo run -- route --no-git --json "Fix README typo"
    cargo run -- eval

Preview the Codex command without launching it:

    cargo run -- run --dry-run "Fix authentication redirect"

Launch the installed Codex CLI with the unmodified prompt:

    cargo run -- run "Fix authentication redirect"

Ask Codex to use an Ollama-hosted open model that you have downloaded
and verified as local and tool-capable:

    cargo run -- run --oss --local-provider ollama --model YOUR_LOCAL_MODEL "Fix bug"

Explicitly select an available model or effort if needed:

    cargo run -- run --model YOUR_AVAILABLE_MODEL --effort medium "Fix bug"

The preagent does not install Codex or Ollama and needs no credentials for
routing. Cloud Codex execution may require paid access. The --oss flag
supports a fully local path when the selected provider and model are
actually running on your computer; some runtimes also expose cloud models.

## Router behavior

The baseline reads the prompt and a bounded list of Git tracked changed-file
names (no source-file contents). It reports intent, heuristic tier, suggested
effort, and reasons. Suggested effort is never applied automatically.
Unrelated Git changes do not change route complexity. Pass --no-git to
disable Git inspection.

Heuristics are not calibrated confidence scores; please do not treat the
output as a verified measure of model capability.

## Routing smoke corpus

Run `preagent eval` to classify the checked-in labeled scenarios in
`evals/baseline.tsv`. The report includes per-tier accuracy and mismatches.
The initial labels are human-authored expectations, not independently proven
best model choices. This is a **regression smoke corpus**, not evidence that
any paid model or local SLM will solve the corresponding coding tasks. We will
use separate held-out tasks for future comparative benchmarks.

## Roadmap

See [Development roadmap](docs/roadmap.md).

## Development

GitHub Actions runs formatting, Clippy, unit tests, integration tests, and
release builds on Windows, Linux and macOS.

    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
    cargo test --all-targets --locked

## License

Apache License 2.0. See [LICENSE](LICENSE).
