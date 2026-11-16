# v0.1.0 release-readiness checklist

**Status: release candidate, not published.** The Windows interactive-session
checks passed on 2026-11-16 with the explicit local configuration below. Main
CI artifacts may be used to verify the candidate before a release is authorized.

## Core scope

Version `0.1.0` is an offline-first routing and execution **preview**.
Rules-based classification, advisory effort tiers, opt-in Ollama classification,
local model-name listing, explicit native agent launchers, and diagnostics are
in scope. The CLI forwards the original prompt without a shell. Prompt-file
input allows the agent to retain terminal stdin; piped stdin is also supported.

Neither automatic model selection nor automatic prompt enhancement is part
of this first version. The program must not imply otherwise. Rules/SLM
agreement with hand-authored labels does not establish coding-task success.

## Verified before tagging

- [x] Apache-2.0 license, Rust package metadata, security/contribution docs.
- [x] Pinned dependency-free core and offline routing without API keys.
- [x] Integration tests for prompt boundaries, native launch argument
      construction, exit codes and Windows Git metadata.
- [x] Cross-platform test, lint, formatting and release binary builds in CI.
- [x] Minimum supported Rust 1.74 CI test.
- [x] Local Windows build, routing, Ollama and Codex flag checks documented
      in `docs/windows-validation.md` (2026-11-16 follow-up).
- [x] Separate experimental evaluation controls with verified green/red
      regression fixtures. Do not mistake these for agent success.
- [x] Automated rejection of a release tag mismatched with Cargo.toml or
      Cargo.lock, plus missing or tampered archive checksums.
- [x] Production-style release archive packaging exercised in regular
      Windows, Linux and macOS CI.

## Final candidate checks

- [x] On the release candidate code, test a **real interactive Codex session**
      using the installed Codex CLI and an explicitly approved available
      model/provider. No unexpected paid API charges.
- [x] Verify `run --prompt-file` with an actual interactive coding agent,
      including Ctrl+C, working directory, terminal I/O and exit status.
- [x] Reconfirm the latest native Windows artifact and local Ollama routing
      fallback on 2026-11-16.
- [x] Check installed Claude Code and OpenCode executables. Claude is absent;
      OpenCode is available only as a PowerShell script shim, so both remain
      unverified as native agent launches.
- [x] Extract the latest Windows archive, verify its SHA-256 checksum and
      smoke-test its executable.
- [x] Review the v0.1.0 tag candidate and release notes. Document the known
      Codex/Ollama/model compatibility limit and its explicit workaround.

Codex CLI 0.154.0 with Ollama 0.32.5 and installed `qwen2.5-coder:1.5b`
completed the native and Preagent interactive smoke sessions with explicit
`--effort none`. The prompt-file session displayed the original multiline
Unicode prompt, answered a second prompt entered through terminal stdin,
cancelled an active turn on Ctrl+C, and exited with code 0 after another
Ctrl+C. The sentinel file was unchanged, and no test agent process remained.
This validates interactive launch and I/O, not coding-task success. See the
[2026-11-16 Windows validation update](windows-validation.md#release-candidate-validation-2026-11-16).

That model rejects the default thinking request and explicit `low` effort.
Codex still reports an Ollama model-list response-shape warning and uses
fallback model metadata. Other model/runtime combinations remain unverified.
Claude Code and native OpenCode were unavailable; their launchers have only
stub coverage on this machine. No model download or paid model was needed.

The candidate's technical checks are complete once CI is green for the final
commit. A release tag and GitHub Release still require separate authorization.

## Release procedure after machine-specific checks

1. Confirm clean main and a green CI run on the exact commit.
2. Verify `cargo fmt --all --check`, `cargo clippy --all-targets -- -D
   warnings`, `cargo test --all-targets --locked`, and native smoke tests.
3. Check `node scripts/release-check.cjs tag v0.1.0`. Version must match
   Cargo.toml and Cargo.lock.
4. Review this checklist and release notes. Deliberately push an annotated
   `v0.1.0` tag **only once**. No branch or pull request is needed.
5. The tagged Release workflow repeats format/lint/tests, native binary
   smoke tests and archive packaging on Windows, Linux and macOS. Its publish
   job checks exactly three archives and matching SHA-256 checksums before
   creating the GitHub Release.
6. Verify the published archive names, checksums, and executable versions
   manually. Do not describe artifacts as signed or notarized.

## Supported binaries

- `preagent-v0.1.0-Windows-X64.tar.gz`: `preagent.exe`
- `preagent-v0.1.0-Linux-X64.tar.gz`: `preagent`
- `preagent-v0.1.0-macOS-ARM64.tar.gz`: `preagent`

Every archive has a neighboring `.sha256` text file. GitHub Actions
development artifacts are not GitHub Releases. Platform coverage does not
include Windows ARM, Linux ARM or Intel macOS.

## Intentionally deferred

- Automatic matching of tasks to agent/model/effort, which requires
  validated model capabilities and outcome evidence.
- Embedded SLM and model redistribution/license checks.
- Public claims based on v2 candidate tasks before authentic human reviews,
  transitive license audit and frozen fixture controls.
- Paid service integrations, background telemetry, installers, signing or
  automatic configuration changes.

Keep the user-facing status accurate even if these milestones are not reached.
