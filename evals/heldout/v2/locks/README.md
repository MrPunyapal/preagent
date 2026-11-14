# Pinned v2 dependency snapshots

These bytes come from the six GitHub Actions artifacts produced by
[declared-license inventory run 43581601670](https://github.com/MrPunyapal/preagent/actions/runs/43581601670).
The `catalog.json` records exact pinned upstream revisions, artifacts
and SHA-256 digests for each resolved Cargo.lock and dependency inventory.

The one-shot workflow `heldout-v2-pin-locks.yml` retrieves those artifacts
through GitHub, validates SHA-256 and repository/package metadata, and
commits lock and inventory snapshots directly to main only if main has not
advanced. There is no external download of coding models, no local machine
requirement, no branch or PR, and no force push.

Run `node evals/heldout/v2/locks/lock-snapshots.cjs check` to verify
materialized files. A missing snapshot is a **blocker**, not proof of a
bad package or a successful license audit.

The retained `licenses/*.json` are **declared-license inventories only**.
They do not provide legal approval or verified transitive license signoffs.
