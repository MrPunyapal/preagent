# v0.1.0 release-readiness checklist

**Status: release candidate, not published.** No tag or GitHub release should
be created until the remaining platform-specific checks have passed. Main CI
artifacts may be used to verify the candidate in advance.

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
      in `docs/windows-validation-2026-11-15.md`.
- [x] Separate experimental evaluation controls with verified green/red
      regression fixtures. Do not mistake these for agent success.
- [x] Automated rejection of a release tag mismatched with Cargo.toml or
      Cargo.lock, plus missing or tampered archive checksums.
- [x] Production-style release archive packaging exercised in regular
      Windows, Linux and macOS CI.

## Remaining checks before publishing

- [ ] On the latest main commit, test a **real interactive Codex session**
      using the installed Codex CLI and an explicitly approved available
      model/provider. No unexpected paid API charges.
- [ ] Verify `run --prompt-file` with an actual interactive coding agent,
      including Ctrl+C, working directory, terminal I/O and exit status.
- [ ] Reconfirm the latest native Windows artifact and local Ollama fallback,
      especially after any runtime updates.
- [ ] Test Claude Code and OpenCode with their own installed native executables
      where practical. If unavailable, explicitly mark them unverified instead
      of blocking unrelated release functionality.
- [ ] Review Windows archive extraction, executable behavior and SHA-256
      checksum against the latest main CI artifacts.
- [ ] Review the exact version-tag candidate and release notes; decide if the
      unsupported Codex/Ollama combination needs a documentation-only warning
      or an engineering fix.

A prior local Windows test with Codex 0.154.0 and Ollama 0.32.5 using
`qwen2.5-coder:1.5b` failed because of Codex/model compatibility. Preagent
did not demonstrate an argument construction defect. The first release should
not promise this exact integration works. Claude Code/OpenCode were absent
on that Windows machine; their launchers are stub-tested but not proven with
real interactive sessions.

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
