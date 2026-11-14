# Held-out v2 blind complexity review packet

This packet contains 36 **candidate** task descriptions selected without using
Preagent router or model outcomes. The candidate benchmark is NOT frozen.

Reviewers: assess task complexity **independently** without viewing other reviewers'
answers, SLM results, routing output or seed patches. Read only the frozen proposed
task descriptions and acceptance behavior below. You may inspect the referenced
upstream repositories at their pinned revisions, but **do not edit** prompts or
criteria as part of a review. Record uncertain judgments and concerns explicitly.

Use one tier per task: `simple`, `standard` or `complex`.
The tiers reflect expected engineering effort, not model success:
- simple: small localized change with predictable acceptance behavior.
- standard: needs investigation, several edge cases or moderate code changes.
- complex: significant cross-module interactions, invariants or difficult validation.

Return a completed `submission.template.json` with your actual reviewer ID, date,
tier and concise rationale for every case. A second reviewer must complete the
same packet separately. Record disagreement; do not negotiate labels to
produce artificially high agreement with any routing model.

**No seeded defects, hidden executable tests, or proposed model labels are
included in this reviewer packet.** Technical maintainers should also examine
implementation feasibility and source licensing in a separate process.

## Task 01 | tokio-timeout-deadline

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Repair a planted timeout boundary regression so a future that has completed before its deadline is not reported as elapsed.

**Acceptance behavior:**
- Under paused time, a ready future yields its value at the deadline boundary; an actually pending future expires.
- No spurious extra wake or completion is observed when the task is polled repeatedly.

**Reviewer judgment:** _leave blank until independent review_

## Task 02 | tokio-oneshot-cleanup

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Remove a planted duplication in the oneshot closed-state cleanup path while preserving cancellation, notification and ownership semantics.

**Acceptance behavior:**
- Dropping the sender wakes the receiver and returns its closed result; dropping the receiver notifies the sender.
- No double delivery or lost wake appears under repeated cancellation and send/receive orders.

**Reviewer judgment:** _leave blank until independent review_

## Task 03 | tokio-bounded-backpressure-tests

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Restore missing bounded-channel regression coverage for capacity-one backpressure and sender shutdown.

**Acceptance behavior:**
- A restored regression test asserts that a second send waits until capacity frees.
- A separate assertion checks receiver closure and pending sender resolution after the last receiver drops.

**Reviewer judgment:** _leave blank until independent review_

## Task 04 | tokio-process-tree-cleanup

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Repair a seeded child-process cleanup regression so opt-in kill-on-drop cannot leave an unintended child process running.

**Acceptance behavior:**
- On the supported test platform, dropping a configured child requests termination and the process exits.
- The default behavior for child processes without kill-on-drop remains unchanged.

**Reviewer judgment:** _leave blank until independent review_

## Task 05 | tokio-time-interval-coordination

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Repair coordinated interval and sleep scheduling after a planted change that mishandles missed ticks under paused time.

**Acceptance behavior:**
- MissedTickBehavior::Skip and Burst remain observably distinct after advancing a paused clock.
- Sleeping and interval consumers still wake in the expected order across a large time jump.

**Reviewer judgment:** _leave blank until independent review_

## Task 06 | tokio-scheduler-cancellation-boundary

**Source:** [tokio-rs/tokio](https://github.com/tokio-rs/tokio/tree/ab0814fc8c62b3d8be40aad068e2fed6562cfeb0) at `ab0814fc8c62b3d8be40aad068e2fed6562cfeb0`

**Request:** Restore a scheduler cancellation invariant shared by current-thread and multi-thread paths without changing the public runtime API.

**Acceptance behavior:**
- Cancelled tasks cannot complete a user-visible action after cancellation is observed.
- Both scheduler modes progress unrelated ready work under a bounded deterministic workload.

**Reviewer judgment:** _leave blank until independent review_

## Task 07 | serde-numeric-range

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Repair a seeded numeric deserialization regression that accepts an integer outside the destination type's range.

**Acceptance behavior:**
- The destination type rejects both over-maximum and under-minimum integer inputs.
- Boundary integer values still deserialize correctly without truncation or wrapping.

**Reviewer judgment:** _leave blank until independent review_

## Task 08 | serde-serialize-container-sharing

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Consolidate planted duplicated sequence-serialization control flow while retaining all emitted element and end markers.

**Acceptance behavior:**
- A recording serializer sees the same element order and sequence length for arrays and slices.
- Serializer errors propagate unchanged and serialization remains compatible with no_std where supported.

**Reviewer judgment:** _leave blank until independent review_

## Task 09 | serde-derive-alias-coverage

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Restore test coverage for a derived field alias and its default-value interaction.

**Acceptance behavior:**
- A regression test verifies the alias deserializes into the intended field without changing the canonical field name.
- A second assertion verifies the configured default applies only when both canonical name and alias are absent.

**Reviewer judgment:** _leave blank until independent review_

## Task 10 | serde-deny-unknown-fields

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Repair a planted rejection-bypass in a derived type using deny_unknown_fields, without rejecting valid known fields.

**Acceptance behavior:**
- Unexpected fields in adversarial input are rejected rather than silently accepted.
- Known fields and configured aliases continue to deserialize normally.

**Reviewer judgment:** _leave blank until independent review_

## Task 11 | serde-derive-renames-roundtrip

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Repair a cross-direction rename regression so derived Serialize and Deserialize agree on a renamed variant and field.

**Acceptance behavior:**
- Serializing then deserializing the test enum preserves the intended variant and field value.
- The external field/variant names remain identical in both directions; legacy aliases work where configured.

**Reviewer judgment:** _leave blank until independent review_

## Task 12 | serde-core-derive-visitor-boundary

**Source:** [serde-rs/serde](https://github.com/serde-rs/serde/tree/aa7913ad0211fef960df68e7812a6211145a888c) at `aa7913ad0211fef960df68e7812a6211145a888c`

**Request:** Restore the boundary between generated Deserialize code and Serde's visitor contract after a planted architectural regression.

**Acceptance behavior:**
- A custom visitor receives exactly one completed visit for valid struct input and handles errors correctly.
- The generated deserializer does not bypass visitor error handling or change the public trait contract.

**Reviewer judgment:** _leave blank until independent review_

## Task 13 | ripgrep-escaped-glob-literal

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Repair a planted glob-escape regression so escaped metacharacters match literally.

**Acceptance behavior:**
- Escaped brace, bracket and question-mark patterns match their literal target paths.
- Unescaped wildcard and brace expansion patterns continue to match according to glob syntax.

**Reviewer judgment:** _leave blank until independent review_

## Task 14 | ripgrep-ignore-precedence

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Consolidate a deliberately duplicated ignore-rule precedence path without changing parent/child override behavior.

**Acceptance behavior:**
- Negated child rules override applicable parent ignore patterns exactly where allowed.
- Directory-only patterns and nested ignore files preserve documented precedence.

**Reviewer judgment:** _leave blank until independent review_

## Task 15 | ripgrep-option-terminator-tests

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Restore CLI regression assertions that filenames following -- are never treated as options.

**Acceptance behavior:**
- A restored test passes a filename starting with a hyphen after -- and verifies it remains an operand.
- A separate test checks ordinary flags before -- still apply.

**Reviewer judgment:** _leave blank until independent review_

## Task 16 | ripgrep-preprocessor-argv-safety

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Repair a planted unsafe argument-handling regression for an explicitly configured preprocessor command.

**Acceptance behavior:**
- Metacharacters inside an argument are passed literally, without executing an unexpected shell command.
- A benign configured preprocessor still receives the intended filename and exits normally.

**Reviewer judgment:** _leave blank until independent review_

## Task 17 | ripgrep-json-printer-summary

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Restore consistent match counts and event ordering across JSON output and summary statistics after a two-file regression.

**Acceptance behavior:**
- A fixed corpus yields the same match count in JSON and summary modes.
- Multiple files with and without matches preserve per-file event boundaries and nonnegative counters.

**Reviewer judgment:** _leave blank until independent review_

## Task 18 | ripgrep-walker-policy-isolation

**Source:** [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep/tree/178cc027e283fd965cb72d0a7395b219eada170c) at `178cc027e283fd965cb72d0a7395b219eada170c`

**Request:** Repair separation between directory traversal and gitignore policy after a planted cross-module regression.

**Acceptance behavior:**
- Walking with ignore enabled and disabled yields the expected independent file sets.
- Symlinks and nested .gitignore policy do not cause traversal outside the selected root.

**Reviewer judgment:** _leave blank until independent review_

## Task 19 | regex-escaped-class-range

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Repair a seeded character-class parser bug that misinterprets a literal hyphen or escaped bracket as a range delimiter.

**Acceptance behavior:**
- Escaped hyphen and bracket syntax compiles to the intended character set.
- Invalid descending ranges still produce a syntax error, not a silent match.

**Reviewer judgment:** _leave blank until independent review_

## Task 20 | regex-meta-search-consistency

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Remove a planted duplicated search-result handling path while preserving find and captures behavior.

**Acceptance behavior:**
- Find and captures agree on the first match span for the same pattern and haystack.
- Zero-width matches and no-match results retain their original semantics.

**Reviewer judgment:** _leave blank until independent review_

## Task 21 | regex-utf8-boundary-tests

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Restore regression coverage for UTF-8 boundaries and zero-width matching in the lightweight engine.

**Acceptance behavior:**
- A restored test checks a zero-width match at a valid Unicode boundary.
- A separate assertion verifies that a match cannot split a multi-byte UTF-8 codepoint in string mode.

**Reviewer judgment:** _leave blank until independent review_

## Task 22 | regex-regex-size-limit

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Restore enforcement of a configured automaton size limit for adversarially large regex patterns.

**Acceptance behavior:**
- An oversized expression is rejected using the explicitly configured low limit.
- A modest valid expression remains accepted under a reasonable limit.

**Reviewer judgment:** _leave blank until independent review_

## Task 23 | regex-unicode-flag-crossmodule

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Repair a planted mismatch in Unicode-flag handling between syntax translation and regex execution.

**Acceptance behavior:**
- Unicode-aware matching behaves as specified when the flag is enabled.
- Disabling Unicode produces the documented ASCII-oriented behavior on the same input.

**Reviewer judgment:** _leave blank until independent review_

## Task 24 | regex-engine-selection-contract

**Source:** [rust-lang/regex](https://github.com/rust-lang/regex/tree/3d79fe92666a9f191d7677ebc123ac165eac200c) at `3d79fe92666a9f191d7677ebc123ac165eac200c`

**Request:** Restore an engine-selection invariant so optimized and fallback regex strategies produce the same observable captures.

**Acceptance behavior:**
- The same test corpus yields identical match spans across enabled search strategies.
- A strategy fallback preserves match order and never silently drops a capture.

**Reviewer judgment:** _leave blank until independent review_

## Task 25 | clap-option-terminator

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Repair a planted end-of-options regression so a value after -- is never parsed as an option.

**Acceptance behavior:**
- An argument named -value is accepted as a positional value after --.
- Options before -- still parse normally and required positionals remain enforced.

**Reviewer judgment:** _leave blank until independent review_

## Task 26 | clap-help-layout-refactor

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Remove duplicated help-row rendering logic without changing ordering, wrapping or styles.

**Acceptance behavior:**
- Help text remains identical for commands with short/long flags and subcommands.
- Colored and plain help formats preserve their expected line structure.

**Reviewer judgment:** _leave blank until independent review_

## Task 27 | clap-alias-validation-tests

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Restore regression coverage for a required argument group and a subcommand alias.

**Acceptance behavior:**
- A restored test fails parsing when all members of a required group are absent.
- A separate assertion verifies alias resolution without accepting an unrelated alias.

**Reviewer judgment:** _leave blank until independent review_

## Task 28 | clap-argv-literal-safety

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Repair a planted parser regression that interprets shell-like bytes after -- as control syntax rather than literal user input.

**Acceptance behavior:**
- Tokens containing shell metacharacters remain literal positional values with no process execution.
- Unknown options before -- still return a normal parser error.

**Reviewer judgment:** _leave blank until independent review_

## Task 29 | clap-usage-help-agreement

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Repair a planted inconsistency between required-argument usage output and full help text.

**Acceptance behavior:**
- Required positional arguments appear consistently in both usage and full help.
- Optional and mutually exclusive arguments remain correctly marked in both outputs.

**Reviewer judgment:** _leave blank until independent review_

## Task 30 | clap-builder-parser-separation

**Source:** [clap-rs/clap](https://github.com/clap-rs/clap/tree/ec93e71255c0b833a34f50662a3fdb22f53f6ded) at `ec93e71255c0b833a34f50662a3fdb22f53f6ded`

**Request:** Restore separation of immutable command configuration from per-invocation parser state.

**Acceptance behavior:**
- Parsing the same command twice produces equivalent values without state leaking between runs.
- Subcommand configuration remains stable after an earlier parse error.

**Reviewer judgment:** _leave blank until independent review_

## Task 31 | hyper-chunked-extensions

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Repair a planted chunked transfer decoder regression that mishandles valid chunk extensions.

**Acceptance behavior:**
- A valid chunk extension is parsed without altering the decoded body bytes.
- Malformed chunk sizes still produce an error and do not expose truncated body content.

**Reviewer judgment:** _leave blank until independent review_

## Task 32 | hyper-header-parse-refactor

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Remove duplicated header validation logic while retaining HTTP/1 response and request parsing semantics.

**Acceptance behavior:**
- Accepted requests and responses preserve existing parsed header values.
- Malformed header syntax still fails with the same error class.

**Reviewer judgment:** _leave blank until independent review_

## Task 33 | hyper-pipelining-regression-tests

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Restore regression tests for two pipelined HTTP/1 requests on a persistent connection.

**Acceptance behavior:**
- A restored test verifies both requests receive responses in the original order.
- An explicit connection-close case is tested separately so buffered bytes are not lost.

**Reviewer judgment:** _leave blank until independent review_

## Task 34 | hyper-ambiguous-framing-rejection

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Repair a planted HTTP request-framing ambiguity that could enable request smuggling.

**Acceptance behavior:**
- Conflicting Content-Length headers and conflicting transfer framing are rejected.
- Unambiguous valid Content-Length and chunked messages remain accepted.

**Reviewer judgment:** _leave blank until independent review_

## Task 35 | hyper-eof-coordination

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Repair a planted early-EOF mismatch between HTTP/1 client and server connection handling.

**Acceptance behavior:**
- Premature body EOF is reported as incomplete instead of successful.
- A correctly terminated body is received intact on both client and server paths.

**Reviewer judgment:** _leave blank until independent review_

## Task 36 | hyper-conn-dispatch-boundary

**Source:** [hyperium/hyper](https://github.com/hyperium/hyper/tree/378f8479b8b703d098b46de1cec9ea648f12ea69) at `378f8479b8b703d098b46de1cec9ea648f12ea69`

**Request:** Restore an invariant between connection state and dispatch lifecycle after a planted architectural regression.

**Acceptance behavior:**
- Graceful shutdown allows an in-flight response to finish without accepting a new request.
- Dispatch cannot report successful completion while buffered response bytes remain unwritten.

**Reviewer judgment:** _leave blank until independent review_

