# preagent

An offline-first, open-source pre-agent for coding assistants.

**Status: early Rust prototype.** This release provides deterministic, explainable
task classification and a safe coding-agent CLI pass-through. It is not yet an automatic model router or a bundled SLM. Optional local
Ollama classification is experimental and advisory.

## Principles

- Free and open source (Apache 2.0).
- Zero runtime dependencies for the current baseline.
- Local classification without API keys, network calls, or telemetry.
- The developer's prompt is forwarded unchanged as a single process argument.
- No unverified model IDs, automatic provider changes, or hidden prompt rewrites.
- Native Codex approvals, authentication, and sandbox configuration stay in control.
- Optional local inference remains opt-in; rules work without a model.

## Build

Install Rust 1.74+ and Cargo. Build and test with:

    cargo build --release
    cargo test --all-targets --locked

## Usage

Classify a request with no AI service:

    cargo run -- route "Fix authentication redirect"
    cargo run -- route --no-git --json "Fix README typo"
    cargo run -- eval
    cargo run -- doctor

Use an already installed local Ollama model to analyze uncertain prompts:

    cargo run -- route --slm qwen3.5:2b "Help with this problem"

Force local SLM comparison even when rules are certain:

    cargo run -- route --slm qwen3.5:2b --slm-always "Fix README typo"

The optional SLM connects only to 127.0.0.1:11434. Set
PREAGENT_OLLAMA_PORT to choose another loopback port. It never downloads
models, requests paid services, or contacts external hosts. A local runtime
and an already installed model are required. Failures preserve the
deterministic decision and model suggestions cannot downgrade a higher-risk
rules-based tier. The suggestion does not automatically override the Codex
model or reasoning effort.

Preview the Codex command without launching it:

    cargo run -- run --dry-run "Fix authentication redirect"

Launch the installed Codex CLI with the unmodified prompt:

    cargo run -- run "Fix authentication redirect"

Or launch installed Claude Code or OpenCode, using the same original prompt:

    cargo run -- run --agent claude "Fix authentication redirect"
    cargo run -- run --agent opencode "Fix authentication redirect"

Claude Code uses its native interactive CLI; OpenCode uses the officially
documented mini TUI with its initial prompt. Models are passed only when
explicitly selected, and agent-specific arguments are validated. No new
provider authentication or API credentials are requested by Preagent.

On Windows, install a native agent `.exe` on PATH. Shell-only `.cmd`/`.bat`
launchers (including some npm shims) are not launched; Preagent never routes
your prompt through a shell. `doctor` checks for native executables.
Effort support depends on the selected agent version and model; an accepted
Preagent label does not guarantee the provider supports that level.

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

## Offline diagnostics

Run cargo run -- doctor to check which CLIs are available on PATH, whether
Git is installed, and whether Ollama responds on the loopback interface.
The command never contacts cloud services or reveals credentials. It does
not authenticate with or execute any of the detected coding agents.

## Separate held-out evaluation pilot

Four frozen regression-repair tasks from permissively licensed Rust projects
are defined in [evals/heldout](evals/heldout/README.md). These cases are
**not** part of the legacy 45-case smoke corpus. Human complexity labels
are not coding-task success measures.

    cargo run -- heldout list
    cargo run -- heldout route --json
    cargo run -- heldout route --json --slm qwen2.5-coder:1.5b

The SLM command requires a locally installed model and a running Ollama.
Neither command runs a coding agent or downloads models.

Prepare a disposable local fixture from an existing pinned checkout, then
grade it using hidden acceptance tests injected in a temporary snapshot:

    cargo run -- heldout prepare itoa-decimal-pairs ../itoa ../scratch-itoa
    cargo run -- heldout grade itoa-decimal-pairs ../scratch-itoa

The source checkout must be at the commit recorded in the manifest, and the
output must not exist. Grading uses cargo test --offline and leaves the
candidate unchanged. Verify that the seeded fixture fails and the clean
upstream baseline passes before attributing a passing result to agent work.
See the task README for the full procedure.

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


## CI binaries and releases

Every successful main-branch CI run uploads native Windows, Linux, and macOS
executables under Actions artifacts. Choose the artifact matching your OS,
extract it, and run preagent --help or preagent doctor.

Versioned releases are built from an explicitly pushed v-prefixed Git tag.
Each release contains per-platform archives and SHA-256 checksums; the release
workflow never needs API keys or a paid inference service.
