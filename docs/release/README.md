# Shiplog Release Documentation

This directory separates the **current release procedure** from historical
release evidence.

## Current procedure

Use [`release-preparation.md`](release-preparation.md) for every new release.
It owns the current order of operations, stop conditions, proof commands,
publication boundary, rollback rules, and copyable checklists.

Promotion from the development trunk into the public source repository is a
separate prerequisite. Use
[`plans/shiplog-swarm/promotion-runbook.md`](../../plans/shiplog-swarm/promotion-runbook.md)
for that exact-source/exact-swarm transaction.

## Artifact roles

| Surface | Role |
| --- | --- |
| `release-preparation.md` | Living preparation, staging, publication, and closeout procedure. |
| `templates/` | Starting points for a new versioned readiness ledger and execution handoff. |
| `<version>-release-decision.md` | Version-specific scope and release/no-release decision. |
| `<version>-readiness.md` | Version-specific preflight and observed release evidence. |
| `../../RELEASE_HANDOFF_<version>.md` | Version-specific execution and public-state handoff. |
| `../../CHANGELOG.md` | User-visible release contents and version history. |

Versioned files and root `RELEASE_HANDOFF_*` files are historical receipts.
They may contain commands, workflow names, tools, versions, or repository state
that were correct for that release but are no longer current. **Do not copy a
historical command block into a new release.** Start from the living procedure
and current templates, then record the commands and evidence actually used.

## Authority boundary

- `EffortlessMetrics/shiplog-swarm` owns normal product, documentation, test,
  CI, policy, and release-preparation work.
- `EffortlessMetrics/shiplog` owns the public source lineage, release tags,
  crates.io publication, GitHub Releases, signing, package-channel publication,
  and security-sensitive release credentials.
- A release candidate must be promoted from a proven exact swarm head into
  source through a regular merge-commit promotion before release-specific
  source work begins.
- A product or documentation defect discovered during source release prep goes
  back to swarm first. Do not patch around it directly on source.

## Quick route

1. Finish and prove the release contents on `shiplog-swarm/main`.
2. Promote the exact green swarm head with `cargo xtask promote`.
3. Verify and close out the regular-merge source checkpoint.
4. Prepare the version, changelog freeze, release decision, readiness ledger,
   and handoff on current `shiplog/main`.
5. Merge the release-prep PR with source CI green.
6. Push one immutable annotated tag from the exact merged source commit.
7. Require the source Release workflow to produce and validate the exact four
   platform candidates while the GitHub release remains a draft.
8. Publish crates.io and make the GitHub release public only after the exact tag
   workflow is green.
9. Verify public assets and package channels, then finalize the release
   evidence and back-port any source-only release documentation to swarm.
