# Contributing

Preagent's core must remain free, open source, offline-first, and usable on
Windows, Linux, and macOS. No paid API key or proprietary service is allowed
in automated tests or required for routing.

Check changes with:

    cargo fmt --all --check
    cargo clippy --all-targets -- -D warnings
    cargo test --all-targets --locked

Routing regressions can be inspected with:

    cargo run -- eval --json

The labeled corpus is a smoke suite, not evidence of real coding-agent
success. New routing claims require measured end-to-end evaluation.

The maintainer's current workflow uses tested atomic commits directly on
main, without feature PRs. Discuss substantial changes through repository
issues before implementation.
