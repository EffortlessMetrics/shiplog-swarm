# Current shiplog-swarm Promotion

**Status:** source promotion in progress
**Swarm head before receipt refresh:** `2862863b38ce5fe078ac440085648c4198a460be`
**Source base:** `a3a15edcdc03d667f6e0152b86455c067c5d6509`
**Source promotion:** `EffortlessMetrics/shiplog#649`

## Included work

- `EffortlessMetrics/shiplog-swarm#225` — canonicalize unknown source-system names to lowercase.
- `EffortlessMetrics/shiplog-swarm#226` — enforce half-open Git and Jira date windows.
- `EffortlessMetrics/shiplog-swarm#233` — refresh the current promotion receipts and compact the active manifest.

## Proof

### Swarm proof

- `Shiplog Rust Small Result` passed for `shiplog-swarm/main` at `2862863b38ce5fe078ac440085648c4198a460be` in run `29218878179`.
- Branch protection is strict and requires only `Shiplog Rust Small Result`.
- The stale pre-reconciliation security report PR `EffortlessMetrics/shiplog-swarm#200` was closed unmerged.

### Source proof

- `EffortlessMetrics/shiplog#649` is the regular-merge promotion PR.
- Source `Shiplog Rust Small Result` passed at the pre-receipt head in run `29230991924`.
- Source proof must rerun and pass after the promotion branch advances to the final receipt-refresh head.
- The source-only regex Dependabot PR `EffortlessMetrics/shiplog#632` was closed; dependency work remains on the swarm trunk.

## Merge boundary

- Swarm PRs use squash merge.
- Source promotion uses a regular merge commit; do not squash.
- Release authority, tags, publishing, signing, and release workflows remain in `EffortlessMetrics/shiplog`.

## Verification

```bash
rtk git fetch origin --prune
rtk git fetch swarm --prune
rtk git merge-base origin/main swarm/main
rtk git log --oneline origin/main..swarm/main
rtk gh run list --repo EffortlessMetrics/shiplog-swarm --branch main --limit 10
rtk gh run list --repo EffortlessMetrics/shiplog --branch main --limit 10
rtk cargo xtask repo-contract-report
rtk git diff --check
```

## Next action

Merge this receipt refresh after `Shiplog Rust Small Result` passes, fast-forward the source promotion branch to the resulting swarm head, require green source proof, and merge `EffortlessMetrics/shiplog#649` with a regular merge commit.
