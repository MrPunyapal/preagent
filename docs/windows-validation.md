# Windows validation

Validation was performed on 2026-11-11 and 2026-11-12 against `main`. The fixes and regression tests are in commits `82cb0ff`, `74cca75`, `81d633d`, and `43e135a`. The starting commit was `6bfae8c`. No existing user changes were present.

## Environment

- Windows 11 Home Single Language, build `10.0.26300`, x64.
- Intel Core i7-12700H, 20 logical processors, 15.63 GiB visible RAM. Available RAM varied from 0.8 to 2.6 GiB during checks.
- NVIDIA GeForce RTX 3050 Laptop GPU, 4 GiB VRAM, and Intel Iris Xe integrated graphics. VRAM availability varied during testing.
- Rust 1.96.1, Cargo 1.96.1, Git 2.45.2.
- Visual Studio Build Tools 2026 with MSVC 14.52 and Windows SDK 10.0.26100.
- Codex CLI 0.154.0. Claude Code and OpenCode were not on PATH.
- Ollama 0.32.5. Local models were `qwen2.5-coder:3b` (Q4_K_M, 3.1B), `qwen2.5-coder:1.5b` (Q4_K_M, 1.5B), `qwen2.5-coder:0.5b` (Q4_K_M, 494M), and `qwen2.5:7b` (Q4_K_M, 7.6B). Only the already installed 1.5B model was used.

## Rust checks and CLI

All requested checks pass on Windows:

| Check | Result |
| --- | --- |
| `cargo fmt --all --check` | Pass |
| `cargo clippy --all-targets -- -D warnings` | Pass |
| `cargo test --all-targets --locked` | 48 tests pass (42 unit, 6 integration) |
| `cargo build --release --locked` | Pass |

The standalone `target/release/preagent.exe` passed `--help`, `--version`, `doctor`, `doctor --json`, `eval`, and `eval --json`. JSON was parsed. Invalid flags return 2, and an unavailable agent reports its executable and exits 1. Tests cover Unicode, quotes, multiline and 24,000-character prompts, leading hyphens, shell metacharacters, Git rename records, and operation outside a repository. Native process fixtures verified original argument boundaries, working directory, inherited stdin/stdout/stderr, and Windows exit code 3010. A hidden isolated console test sent Ctrl+C and verified that Preagent and its child both exited without leaving the child running.

The installed Codex CLI help was checked for the `--oss`, `--local-provider`, `--model`, and `-c model_reasoning_effort` options without starting an agent session. Current official Claude Code and OpenCode documentation supports the flags used by the adapters. Live Claude Code and OpenCode launches were unavailable because those CLIs are not installed. Codex was not launched from inside itself. No paid inference was used.

## Deterministic routing

The existing, unchanged 45-case human-labeled corpus scores **45/45 (100%)** after fixes. Accuracy by expected tier is simple 12/12, standard 18/18, and complex 15/15. The initial binary scored 43/45, missing the database migration and data-loss cases. The router now recognizes those cases, common vulnerability terms, and sensitive Git paths at the repository root. Git metadata parsing now preserves Unicode, quotes, newlines, and rename destinations.

An additional 8-prompt security probe caught 7 missed security signals in the initial binary and 0 after the fix. An 8-prompt benign/security-word probe produced 2 conservative security escalations for typo or spelling edits to security-related documentation. These small probes are diagnostic examples, not an independently labeled benchmark. They do not measure coding-task success.

The built-in evaluation took a median 0.195 ms (p95 0.220 ms) over 1,000 in-process runs of the 45-case corpus. Separately, 100 fresh `eval --json` processes had a median wall time of 34.8 ms and p95 of 42.0 ms, including Windows process startup. Thirty sampled CLI runs showed a peak Preagent working set of 4.73 MiB. These measurements use different timing methods and should not be compared directly.

## Local SLM

Ollama was running at loopback and the installed `qwen2.5-coder:1.5b` model was confirmed locally. One cold inference took 3.10 seconds end to end. Three warm samples took 285, 281, and 288 ms. The Ollama API reported about 2.93 seconds model load on the cold request and 225–232 ms per warm generation. During the cold request the Ollama process and model runner peaked at a sampled combined 680 MiB working set. Preagent peaked at 5.18 MiB. Sampling can miss brief memory peaks. The model used 1.17 GB of VRAM according to Ollama, and ran on the GPU.

Across the same 45 corpus prompts, the SLM alone matched 31/45 expected tiers: simple 2/12, standard 18/18, complex 11/15. The hybrid recommendation matched 35/45: simple 2/12, standard 18/18, complex 15/15. The SLM suggested `standard` for the authentication and payment security cases. The rules-based complex tier remained the recommendation. There were no malformed model responses in those 45 calls. The corpus labels are subjective expectations, not evidence of downstream coding success.

Loopback mocks verified rejection of malformed, truncated, nested, duplicate, incomplete, oversized, and ambiguously framed replies. Missing models, invalid model identifiers, unavailable Ollama, malformed ports, and a stalled service all returned the deterministic tier. The stalled response hit the 60-second timeout and exited normally with fallback output. Routing, evaluation, and dry-run commands made zero connections to a configured loopback test port when SLM was not requested. SLM behavior with the service stopped was tested using an unreachable loopback port.

## Changes and outstanding checks

Three fixes were pushed to `main`: deterministic security and migration routing, safe Git status parsing, and strict bounded Ollama response handling with a total timeout. Agent argument handling, effort labels, Windows native executable detection, full Windows exit-code propagation, and integration tests were also corrected. The validation report is in this file.

Native CLI execution remains untested for Codex, Claude Code, and OpenCode. Claude Code and OpenCode are absent. Codex launch from within Codex was deliberately avoided. No external network tracing tool was used, but source inspection and a loopback sentinel verified that default routing/evaluation do not initiate a request. The model benchmark uses only one model and this hand-labeled smoke corpus.

The CI run for commit `81d633d` exposed a macOS-only test-fixture collision because parallel tests could generate the same temporary directory name. Commit `43e135a` adds a process-local atomic suffix. The new cross-platform CI run is pending at report time. Once it passes, the next engineering milestone is a held-out, independently labeled task set evaluated against unchanged agent defaults, rules-only routing, and SLM-assisted routing, with task correctness graded separately from tier agreement.
