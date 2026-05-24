# Codex CI-Efficiency Compatibility (Hard Invariants)

This section is a hard compatibility contract for Codex-authored CI-efficiency
PRs in EffortlessMetrics repos.

> Do not optimize CI by blindly canceling active work or by routing metadata
> edits through Rust. Optimize by classifying changes correctly, keeping one
> active run plus one pending replacement slot, and making default PR paths
> tiny.

## 1) Concurrency semantics (heavy/core PR workflows)

- Do **not** set `cancel-in-progress: true` on heavy/core Rust workflows unless
  the repository explicitly documents that cancellation is safe.
- Required behavior is **single active run + single pending replacement slot**:
  - active run continues;
  - a newer queued run replaces any older pending run;
  - active run is not killed near completion.
- Required pattern:

```yaml
concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.ref }}
  cancel-in-progress: false
```

## 2) Change classification

- Do not treat all changed files as Rust input.
- Metadata/control-plane paths must route to docs/policy/light lanes when not
  mixed with real code changes.
- Typical light-only examples:
  - `docs/**`, `*.md`, `README*`, `CHANGELOG*`, `SECURITY*`, `CONTRIBUTING*`
  - `policy/**`, `plans/**`, `badges/**`, `AGENTS.md`
  - `.github/CODEOWNERS`, `.github/dependabot.yml`,
    `.github/pull_request_template.md`, `.github/PULL_REQUEST_TEMPLATE/**`
  - `.codex/campaigns/**`, `docs/tracking/**`, `ci/hardware/**` receipts
  - `.rails/**`, `.uselesskey/**`
- `.github/workflows/**` remains special: route workflow edits to minimal hosted
  workflow validation/safety, not normal docs-light and not full Rust by
  default.

## 3) Default PR policy

Classify first, then route to the cheapest truthful lane:

- docs/control-plane only -> no Rust compile
- workflow-only -> hosted YAML/workflow validation only
- Rust source/build/test touched -> Rust-small
- hardware/GPU/receipt-only -> syntax/receipt validation only
- unknown or mixed -> Rust-small (not full CI)

Full CI should require an explicit trigger (for example label, manual dispatch,
main push, release, schedule, or merge queue).

## 4) Hosted fallback policy

- Do not silently replace a self-hosted Rust-small lane with a full
  GitHub-hosted Rust equivalent.
- Fork PRs may use a tiny hosted safe lane.
- Runner readiness/token/idle-runner failures must not auto-trigger a 75–120
  minute hosted equivalent.
- Expensive hosted fallback requires explicit controls (for example `full-ci`,
  `allow-github-hosted`, `ci-budget-ack`).

## 5) Artifacts

- Do not upload receipts/JUnit/log artifacts with `if: always()` on default PR
  paths unless required by merge policy.
- Prefer upload-on-failure and short retention (3–7 days).
- Keep policy receipts small; avoid uploads on docs/control-plane-only paths.

## 6) Required tests for CI-only PRs

Every CI-efficiency PR should include:

- `git diff --check`
- YAML parse validation for edited workflow files
- classification dry-run / unit coverage for:
  - docs-only
  - `.rails/**`
  - `.uselesskey/**`
  - workflow-only change
  - Rust-file change
  - mixed docs + Rust
- verification that heavy/core workflow concurrency did not change away from
  no-cancel semantics unless intentionally documented.

## Reviewer rejection checklist

Reject CI-efficiency PRs unless they answer yes to all of:

1. Heavy/core CI preserves `cancel-in-progress: false` (unless explicit,
   documented exception).
2. Metadata/control-plane-only edits avoid Rust CI.
3. Workflow changes are kept out of docs-light routing.
4. No silent expensive hosted fallback was introduced.
5. Actual billable work is reduced (not merely shifted).
