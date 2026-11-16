# Windows validation

This report includes a 2026-11-16 release-candidate revalidation and the
historical 2026-11-11/12 measurements below. No release tag or GitHub Release
was created.

## Release candidate validation (2026-11-16)

### Successful interactive follow-up

Commit `44b4bce` fixes a missing Codex effort option: Preagent previously
rejected explicit `--effort none`, although native Codex 0.154.0 accepts it.
No effort is applied automatically. Regression coverage verifies exact config
forwarding, unchanged default behavior, and rejection of `none` for Claude and
OpenCode. The final local Windows checks passed: format, Clippy with warnings
denied, 68 Rust tests (56 unit and 12 integration), and release build. All four
release-script tests and the v0.1.0 version check also passed.

Both native `codex exec` and the native interactive TUI completed a local
`READY` prompt with `model_reasoning_effort="none"`. The TUI then exited
with code 0 using `/quit`. The equivalent Preagent release binary was tested
in a disposable Git repository:

```text
preagent run --no-git --oss --local-provider ollama --model qwen2.5-coder:1.5b --effort none --prompt-file prompt.txt
```

The prompt file contained multiline text, Chinese characters, an emoji,
quotes, and shell-looking characters. Codex displayed the original text and
answered `PREAGENT_READY`. A second prompt entered through the inherited
terminal stdin returned `TERMINAL_READY`. Ctrl+C interrupted a further active
turn with Codex's "Conversation interrupted" message; another Ctrl+C at idle
shut it down. The wrapper recorded `PREAGENT_EXIT=0`. This is Codex's graceful
interactive shutdown status, not a forced Windows console termination status.
The TUI showed the disposable working directory. No test-model Codex or
Preagent process remained after shutdown.

The README sentinel's SHA-256 remained
`856b9f7445114bdfd756ae2e9a7bc78948dcb811df47afec3d11c724315f1994`.
The scratch directory contained only the original README and prompt, Git
metadata, and exit-code files written by the validation wrapper. No agent file
edits or tool actions were requested. These results verify session startup,
prompt preservation, terminal I/O, cancellation, exit propagation and file
safety for this smoke session; they do not measure coding-task performance.

No new model, runtime installation, credential change or paid inference was
needed. The tested versions remain Codex CLI 0.154.0 and Ollama 0.32.5 with
installed `qwen2.5-coder:1.5b`. Its default/high or explicit low thinking
requests still fail. Codex's model-list response-shape warning persists, but
does not prevent this explicit no-reasoning session. The earlier conclusion
that a different model was necessary was incomplete: the missing explicit
effort option resolves this tested path.

The Windows CI job on `44b4bce` passed and its development artifact
([run 43926992408](https://github.com/MrPunyapal/preagent/actions/runs/43926992408),
artifact `14255948141`) was downloaded and smoke-tested locally. Its archive
contained only `preagent.exe`, matched its SHA-256 sidecar and the standalone
binary, and passed version/help/doctor and explicit `--effort none` dry-run
checks. The archive SHA-256 was
`4d965706cb0a7e861ef85a8ac01af7cba1f1095e2ce0f46230f039c1e8490f2a`.

### Initial validation pass

The repository was clean before synchronization. `main` was safely fast-forwarded
to `8ca2b5b4a6110f29e35bc22d68a4da8ac4972aa3`; no local changes were present.
All requested Rust checks passed on Windows:

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Pass |
| `cargo clippy --all-targets -- -D warnings` | Pass |
| `cargo test --all-targets --locked` | 67 passed (56 unit, 11 integration) |
| `cargo build --release --locked` | Pass |

The release executable reported `preagent 0.1.0`. `doctor --json` detected
Git, Codex and loopback Ollama; it did not detect Claude or OpenCode. `models
--json` listed the four installed Ollama models and correctly marked offline
execution unverified. Two identical no-Git route calls returned byte-identical
JSON. `eval --json` scored the built-in smoke corpus 45/45; this remains a
regression smoke test, not evidence of coding outcomes.

An executable smoke harness verified exact preservation of Unicode, CRLF/LF,
multiline text, quotes, leading option text, and shell-looking characters for
argv, `--stdin`, and `--prompt-file`. A native local test agent received the
prompt-file prompt while independently reading the marker from inherited
stdin. The Windows launcher selected a valid `codex.exe` over co-located
`.cmd`/`.bat` files, and rejected batch-only shims without executing them.
The native-process tests also verified working directory, stdout/stderr,
prompt argument boundaries, and exit codes 37 and 3010. In the real Codex TUI
attempt below, terminal stdin remained interactive and Ctrl+C returned control
to the shell; its numeric exit code was not captured separately.

### Installed versions and local models

- Windows 11 Home Single Language, x64, build `10.0.26300`.
- Rust/Cargo 1.96.1; Git 2.45.2; Node 24.22.2.
- Visual Studio Build Tools 18.12.12405.243, MSVC 14.52.37025, Windows SDK
  10.0.26100.0.
- Codex CLI 0.154.0; Ollama 0.32.5.
- Ollama models: `qwen2.5-coder:0.5b`, `qwen2.5-coder:1.5b`,
  `qwen2.5-coder:3b`, and `qwen2.5:7b`.
- `claude.exe` is absent. OpenCode resolves only to `opencode.ps1`; no native
  `opencode.exe` is installed. Claude Code and OpenCode launches remain
  unverified.

No model was downloaded, no credentials were changed, and no paid API was
used. The optional SLM produced `simple`, `standard`, and `complex` for the
three representative prompts and matched the rules-based recommendation in
each case. An intentionally missing local model returned HTTP 404 and safely
preserved the rules-based complex tier. These four checks are small smoke
samples, not a quality or performance claim.

### Codex/Ollama compatibility

The Preagent dry run and installed Codex 0.154.0 help agree on `--oss`,
`--local-provider`, `--model`, and Codex's `-c
model_reasoning_effort="low"` configuration. A native, read-only, ephemeral
`codex exec` using local Ollama and `qwen2.5-coder:1.5b` returned `READY` when
no reasoning effort was set. With `model_reasoning_effort="low"`, native
Codex itself returned `"qwen2.5-coder:1.5b" does not support thinking`.

The Preagent interactive launcher was then tried in a disposable Git repo with
the same installed model and no effort override. Codex opened its TUI in the
correct directory, but defaulted to high reasoning and returned the same
unsupported-thinking error. This is a native Codex/model compatibility limit,
not an argument-rewriting defect in Preagent. Codex also logged that Ollama's
`/v1/models` response (`data`) did not match the `models` field it expected;
it fell back to model metadata and still issued the request. No file changes
were made in the disposable repo. The direct `exec` success does not prove the
interactive TUI works.

At this initial pass, successful interactive execution and the final process
status had not been verified. The explicit `none` follow-up above resolves
those checks using the same installed model.

### Initial Windows development artifact and release gate

CI run [43921221254](https://github.com/MrPunyapal/preagent/actions/runs/43921221254)
passed Minimum Rust 1.74 and the Linux, Windows and macOS jobs on commit
`8ca2b5b`. The downloaded `preagent-Windows-X64` artifact contained the raw
binary plus `preagent-v0.1.0-Windows-X64.tar.gz` and its checksum. The archive
SHA-256 matched the sidecar:
`83086678e11a61f02cb12f84dd199ad397ff137d366d264d0c73bf0f20078d08`.
Extraction contained only `preagent.exe`; it matched the raw CI binary and
passed `--version`, `--help`, and `doctor --json` smoke tests.

`node scripts/release-check.cjs tag v0.1.0` and all four
`scripts/release-check.test.cjs` tests passed. The v0.1.0 version matches
Cargo.toml and Cargo.lock. No Git tag or GitHub Release was created. The
interactive and prompt-file checks left open by this pass are completed in the
follow-up above. Claude Code and native OpenCode remain unavailable and
unverified.

## Historical validation baseline (2026-11-11 and 2026-11-12)

The fixes and regression tests were in commits `82cb0ff`, `74cca75`, `81d633d`, and `43e135a`. The starting commit was `6bfae8c`. No existing user changes were present.

### Environment

- Windows 11 Home Single Language, build `10.0.26300`, x64.
- Intel Core i7-12700H, 20 logical processors, 15.63 GiB visible RAM. Available RAM varied from 0.8 to 2.6 GiB during checks.
- NVIDIA GeForce RTX 3050 Laptop GPU, 4 GiB VRAM, and Intel Iris Xe integrated graphics. VRAM availability varied during testing.
- Rust 1.96.1, Cargo 1.96.1, Git 2.45.2.
- Visual Studio Build Tools 2026 with MSVC 14.52 and Windows SDK 10.0.26100.
- Codex CLI 0.154.0. Claude Code and OpenCode were not on PATH.
- Ollama 0.32.5. Local models were `qwen2.5-coder:3b` (Q4_K_M, 3.1B), `qwen2.5-coder:1.5b` (Q4_K_M, 1.5B), `qwen2.5-coder:0.5b` (Q4_K_M, 494M), and `qwen2.5:7b` (Q4_K_M, 7.6B). Only the already installed 1.5B model was used.

### Rust checks and CLI

All requested checks pass on Windows:

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Pass |
| `cargo clippy --all-targets -- -D warnings` | Pass |
| `cargo test --all-targets --locked` | 48 tests pass (42 unit, 6 integration) |
| `cargo build --release --locked` | Pass |

The standalone `target/release/preagent.exe` passed `--help`, `--version`, `doctor`, `doctor --json`, `eval`, and `eval --json`. JSON was parsed. Invalid flags return 2, and an unavailable agent reports its executable and exits 1. Tests cover Unicode, quotes, multiline and 24,000-character prompts, leading hyphens, shell metacharacters, Git rename records, and operation outside a repository. Native process fixtures verified original argument boundaries, working directory, inherited stdin/stdout/stderr, and Windows exit code 3010. A hidden isolated console test sent Ctrl+C and verified that Preagent and its child both exited without leaving the child running.

The installed Codex CLI help was checked for the `--oss`, `--local-provider`, `--model`, and `-c model_reasoning_effort` options without starting an agent session. Current official Claude Code and OpenCode documentation supports the flags used by the adapters. Live Claude Code and OpenCode launches were unavailable because those CLIs are not installed. Codex was not launched from inside itself. No paid inference was used.

### Deterministic routing

The existing, unchanged 45-case human-labeled corpus scores **45/45 (100%)** after fixes. Accuracy by expected tier is simple 12/12, standard 18/18, and complex 15/15. The initial binary scored 43/45, missing the database migration and data-loss cases. The router now recognizes those cases, common vulnerability terms, and sensitive Git paths at the repository root. Git metadata parsing now preserves Unicode, quotes, newlines, and rename destinations.

An additional 8-prompt security probe caught 7 missed security signals in the initial binary and 0 after the fix. An 8-prompt benign/security-word probe produced 2 conservative security escalations for typo or spelling edits to security-related documentation. These small probes are diagnostic examples, not an independently labeled benchmark. They do not measure coding-task success.

The built-in evaluation took a median 0.195 ms (p95 0.220 ms) over 1,000 in-process runs of the 45-case corpus. Separately, 100 fresh `eval --json` processes had a median wall time of 34.8 ms and p95 of 42.0 ms, including Windows process startup. Thirty sampled CLI runs showed a peak Preagent working set of 4.73 MiB. These measurements use different timing methods and should not be compared directly.

### Local SLM

Ollama was running at loopback and the installed `qwen2.5-coder:1.5b` model was confirmed locally. One cold inference took 3.10 seconds end to end. Three warm samples took 285, 281, and 288 ms. The Ollama API reported about 2.93 seconds model load on the cold request and 225–232 ms per warm generation. During the cold request the Ollama process and model runner peaked at a sampled combined 680 MiB working set. Preagent peaked at 5.18 MiB. Sampling can miss brief memory peaks. The model used 1.17 GB of VRAM according to Ollama, and ran on the GPU.

Across the same 45 corpus prompts, the SLM alone matched 31/45 expected tiers: simple 2/12, standard 18/18, complex 11/15. The hybrid recommendation matched 35/45: simple 2/12, standard 18/18, complex 15/15. The SLM suggested `standard` for the authentication and payment security cases. The rules-based complex tier remained the recommendation. There were no malformed model responses in those 45 calls. The corpus labels are subjective expectations, not evidence of downstream coding success.

Loopback mocks verified rejection of malformed, truncated, nested, duplicate, incomplete, oversized, and ambiguously framed replies. Missing models, invalid model identifiers, unavailable Ollama, malformed ports, and a stalled service all returned the deterministic tier. The stalled response hit the 60-second timeout and exited normally with fallback output. Routing, evaluation, and dry-run commands made zero connections to a configured loopback test port when SLM was not requested. SLM behavior with the service stopped was tested using an unreachable loopback port.

### Changes and outstanding checks

Three fixes were pushed to `main`: deterministic security and migration routing, safe Git status parsing, and strict bounded Ollama response handling with a total timeout. Agent argument handling, effort labels, Windows native executable detection, full Windows exit-code propagation, and integration tests were also corrected. The validation report is in this file.

Native CLI execution remains untested for Codex, Claude Code, and OpenCode. Claude Code and OpenCode are absent. Codex launch from within Codex was deliberately avoided. No external network tracing tool was used, but source inspection and a loopback sentinel verified that default routing/evaluation do not initiate a request. The model benchmark uses only one model and this hand-labeled smoke corpus.

The CI run for commit `81d633d` exposed a macOS-only test-fixture collision because parallel tests could generate the same temporary directory name. Commit `43e135a` adds a process-local atomic suffix. The new cross-platform CI run is pending at report time. Once it passes, the next engineering milestone is a held-out, independently labeled task set evaluated against unchanged agent defaults, rules-only routing, and SLM-assisted routing, with task correctness graded separately from tier agreement.
