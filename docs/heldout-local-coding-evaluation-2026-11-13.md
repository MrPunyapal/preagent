# Local coding-task evaluation: 13 November 2026

## Findings

The Windows workstation has Ollama and three installed Qwen2.5 Coder models,
but no separate coding-agent runtime besides Codex CLI. Codex CLI 0.154.0 exposes
an OSS Ollama mode (`codex exec --oss --local-provider ollama`); this session is
already running inside Codex, and the task explicitly forbids starting a nested
Codex session. Claude Code, OpenCode, Aider and Continue were not installed.

To gather evidence without launching another agent or installing software, I
implemented a small standard-library Node runner for constrained patch
generation. It is **not an autonomous coding agent**: it gives Ollama one source
excerpt and the frozen task prompt, then treats the response as untrusted text.
It cannot inspect files or call tools, and it never executes generated commands.

All three tested models returned the exact abstention `NO_PATCH` on all four
tasks (**12/12 abstentions**). No patch was applied and no hidden acceptance
tests were run for these attempts. This records a failure to produce a patch
under the frozen prompt procedure; it does **not** establish that any candidate
code passed or failed the acceptance tests. Since the prompt explicitly gave
the model a safe abstention option, the result also does not prove the model
could never produce a patch with another prompt. I did not revise the prompt or
retry to chase a passing result.

No evidence here shows that Preagent improves coding-task outcomes. Keep rules
as the default, keep any SLM advisory and opt-in, and do not add automatic model
selection based on the earlier four-tier-label score.

## Environment and available local execution

- Evaluation date: 2026-11-13; repository main at
  `6e694c1536d2d9789b6870cfe66d352edeae68b5` before these changes.
- Windows 11 build 26300 x86_64; Intel Core i7-12700H (20 logical processors),
  15.63 GiB RAM; NVIDIA RTX 3050 Laptop GPU with 4 GiB VRAM.
- Node.js 24.22.2; Ollama 0.32.5; Codex CLI 0.154.0.
- Installed coder models: `qwen2.5-coder:0.5b` (494M, Q4_K_M, Apache-2.0),
  `qwen2.5-coder:1.5b` (1.5B, Q4_K_M, Apache-2.0), and
  `qwen2.5-coder:3b` (3.1B, Q4_K_M, Qwen Research License). Ollama reports
  tool capability for these models, but this experiment used text completion,
  not tool calls. The 3B model was used only for non-commercial evaluation;
  [its license](https://huggingface.co/Qwen/Qwen2.5-Coder-3B-Instruct/blob/main/LICENSE)
  limits the grant to non-commercial purposes.
- The 7B model was not used: its installed artifact is 4.7 GB on a GPU with
  4 GiB VRAM, and available RAM varied from about 1.8 GiB to 1.1 GiB during
  this experiment.
- RAM free before the model groups was approximately 1.8 GiB (1.5B), 1.6 GiB
  (3B), and 1.15 GiB (0.5B). Ollama had no model loaded before each group and
  was idle after each request. No weights were downloaded.

## Procedure and isolation

Each model/task pair received exactly one attempt. Every attempt began from a
fresh filesystem copy of the matching seeded fixture, whose Git HEAD matched
the frozen commit. The candidate copy matched the seeded source snapshot. The
model received the original task wording plus a source excerpt centered on the
seed hunk; no seed diff or hidden acceptance-test source was given to the model.

The exact system and user prompts, raw completion, model digest, timing fields,
and task metadata are retained in
[`heldout-local-patch-correctness-2026-11-13.json`](../evals/results/heldout-local-patch-correctness-2026-11-13.json).
The fixed generation options were temperature 0, seed 42, context 4096, maximum
1024 output tokens, non-streaming, and `keep_alive: 0`. Ollama's documented
`/api/generate` timing fields and unload behavior are described in the
[Ollama API reference](https://docs.ollama.com/api/generate).

Before any model call, the runner verifies that the exact model is already
installed and refuses to proceed if another model is loaded. It limits patches
to the single source file identified by the checked-in seed patch, rejects
traversal, multi-file, binary, mode, rename and delete changes, validates with
`git apply --check`, and applies with a fixed argument vector and no shell. It
never runs model-provided commands. The existing `preagent heldout grade`
command is called only after an applicable patch is applied. The grader remains
offline and works on its own snapshot. Invalid patches and abstentions are
recorded without retry.

This separation matters: a text completion that happens to contain code is not
equivalent to an agent that can inspect a repository, choose edits, run checks
and iterate.

## Results

All attempts returned `NO_PATCH` (three output tokens). No patch application or
acceptance grade occurred.

| Model | Attempts | Abstentions | Median Ollama total duration | Median model-load duration | Max sampled Ollama host working set |
| --- | ---: | ---: | ---: | ---: | ---: |
| `qwen2.5-coder:0.5b` | 4 | 4 | 2.63 s | 2.38 s | 356 MiB |
| `qwen2.5-coder:1.5b` | 4 | 4 | 2.75 s | 2.44 s | 283 MiB |
| `qwen2.5-coder:3b` | 4 | 4 | 4.34 s | 3.73 s | 344 MiB |

Total process wall-time medians were 2.97 s, 3.10 s and 4.84 s respectively;
they include runner setup and response handling. The host-working-set values
are combined Ollama process samples polled at about 75 ms, not model VRAM or
guaranteed peaks. GPU utilization was **not sampled during generation**. An
idle `nvidia-smi` snapshot before and after showed 0% utilization and 3,962 MiB
free; it cannot be used as evidence of per-attempt GPU use. Ollama was unloaded
between attempts, and its response supplied per-request load and generation
timings.

| Task | 0.5B | 1.5B | 3B | Acceptance result |
| --- | --- | --- | --- | --- |
| `itoa-decimal-pairs` | `NO_PATCH` | `NO_PATCH` | `NO_PATCH` | Not run |
| `ryu-negative-infinity` | `NO_PATCH` | `NO_PATCH` | `NO_PATCH` | Not run |
| `byteorder-endian-read` | `NO_PATCH` | `NO_PATCH` | `NO_PATCH` | Not run |
| `byteorder-signed-width` | `NO_PATCH` | `NO_PATCH` | `NO_PATCH` | Not run |

The local model completion endpoint was available and returned valid JSON with
timing metadata. The models did not emit patches in this fixed protocol. No
acceptance failure or correctness score is inferred from that refusal.

## Routing results are separate

The previous held-out routing report measured 2/4 deterministic tier agreement,
1/4 SLM-only agreement for 0.5B, 3/4 for 1.5B and 1/4 for 3B. Those are tier
label agreements, not code repairs. This patch-generation experiment did not
run Preagent routing to select a different executing model or effort; therefore
it is not a rules-versus-SLM coding outcome comparison.

## Runner checks and remaining gaps

The isolated runner uses only Node built-ins and the existing Git, Ollama and
Preagent grader. Five Node tests passed, covering valid one-file patch
application, path traversal, non-allowlisted paths, multiple files, unsafe
file-operation headers and seed-derived allowlists. No model-generated shell
command was executed. No production router or frozen task file changed.

The Codex OSS local-agent path remains untested because nested Codex execution
is prohibited in this session. Claude Code and OpenCode are absent. The patch
runner has not demonstrated a model repair, so its successful-application and
post-patch-grade path is unit-tested but has not been exercised with a model
patch. GPU utilization during attempts was not collected. No hidden tests were
exposed to the models.

## Recommended next milestone

Use a separately initiated, permitted local Codex OSS session (or another
already-installed local agent) to run the four frozen tasks with identical
permissions and prompts. Capture the agent's actions, patch, duration and
resource use; grade once after each attempt. Compare with the unchanged agent
baseline only if routing changes the actual executing model or effort.

For a broader benchmark, independently sample and freeze approximately 40–60
licensed tasks across bug fixes, refactors, test improvements, security work,
multi-file changes and architecture tasks. Have a second reviewer define
acceptance criteria before runs, keep graders hidden from agents, and repeat
each condition a predeclared number of times. Keep task correctness, tier-label
agreement and routing overhead as separate measurements. Do not tune routing
against either existing held-out set.
