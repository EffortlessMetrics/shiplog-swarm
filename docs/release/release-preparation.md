# Shiplog Release Preparation and Execution

**Status:** active living procedure  
**Authority:** `shiplog-swarm` prepares and proves; `shiplog` promotes, tags,
publishes, and records public release state.

This is the canonical procedure for a new Shiplog release. Versioned readiness,
decision, and handoff files are evidence for one completed release; they are not
standing instructions.

## Operating invariants

1. Normal product, documentation, test, CI, policy, and cleanup work lands in
   `EffortlessMetrics/shiplog-swarm`.
2. `EffortlessMetrics/shiplog` remains the public source and release authority.
3. Source receives a proven exact swarm tree through a regular merge-commit
   promotion PR. Never squash a source promotion.
4. Release-specific source work starts only after the candidate swarm head has
   been promoted and closed out.
5. A defect found during source release prep returns to swarm, is proved there,
   and is promoted again. Do not turn release prep into a second product lane.
6. The release tag is immutable. Never force-move or reuse a failed release
   tag; fix the defect and cut the next patch version.
7. crates.io publication and a public GitHub Release are irreversible public
   boundaries. Keep the GitHub release as a draft until exact-tag proof passes.
8. A skipped or unavailable optional check is not claimed as executed proof.
9. Release claims come from merged behavior and observed artifacts, not commit
   subjects, PR-body assertions, or historical command blocks.

## Phase 0 — Define the candidate

Choose the release version and theme from the user-visible change set, not from
control-plane activity alone.

Before implementation freeze:

- identify the exact user-facing reason for the release;
- decide the intended semantic-version increment;
- list the included and explicitly deferred user-visible changes;
- identify any installer, package-channel, signing, or platform-trust impact;
- create the version-specific release decision from
  [`templates/release-decision.md`](templates/release-decision.md);
- create the version-specific readiness ledger from
  [`templates/release-readiness.md`](templates/release-readiness.md); and
- start the execution handoff from
  [`templates/release-handoff.md`](templates/release-handoff.md).

Do not bump versions merely to make a candidate feel real. Finish the product,
README, guides, changelog curation, and release-scope decision first; make the
version bump late in the source release-prep PR.

## Phase 1 — Finish the candidate on swarm

Release preparation begins on current `shiplog-swarm/main`.

### Content and documentation review

- Curate `CHANGELOG.md` under `[Unreleased]`. Include user-visible behavior,
  meaningful compatibility changes, and important fixes; omit internal churn.
- Update README installation, first-use, recurring-use, and command examples
  affected by the release.
- Update guides, configuration reference, support tiers, and claim boundaries
  before freezing release notes.
- Verify each changelog or release-note statement against merged behavior. If a
  command path has an exception, incomplete platform surface, or unproved
  security property, state the narrower truth.
- Remove stale future-tense wording from current docs without rewriting
  historical release receipts.
- Confirm the release still has a coherent user-facing theme after deferrals.

### Queue and repository state

Both repositories may contain deliberately deferred work, but the release
candidate must not be ambiguous.

```bash
git fetch origin --prune
git fetch swarm --prune

gh pr list --repo EffortlessMetrics/shiplog-swarm --state open --limit 50
gh issue list --repo EffortlessMetrics/shiplog-swarm --state open --limit 50
gh pr list --repo EffortlessMetrics/shiplog --state open --limit 50
gh issue list --repo EffortlessMetrics/shiplog --state open --limit 50
```

For every open item, record one of:

- included before candidate freeze;
- explicitly deferred and non-blocking;
- superseded/duplicate and closed; or
- release blocker.

Do not start promotion while a required-check failure, source mutation incident,
or release-blocking PR has an unexplained disposition.

### Swarm proof

Run the normal contributor and repository-contract proof from a clean tracked
checkout. Known protected agent workspace artifacts may remain when the contract
report classifies them explicitly; tracked edits and unknown untracked paths
remain blocking.

```bash
cargo xtask ci-small
cargo xtask check-policy-ledgers
cargo xtask check-doc-artifacts
cargo xtask check-goals
cargo xtask check-support-tiers
cargo xtask check-workflows --mode blocking-allowlist
cargo xtask check-file-policy --mode blocking-allowlist
cargo xtask check-automation-authority --repository-role swarm
cargo xtask promotion-state --check
cargo xtask repo-contract-report
git diff --check
```

The exact candidate swarm head must also have a successful routed
`Shiplog Rust Small Result` aggregate. The required aggregate is the authority;
optional smoke lanes are supplementary evidence.

## Phase 2 — Promote the exact swarm candidate

Follow
[`plans/shiplog-swarm/promotion-runbook.md`](../../plans/shiplog-swarm/promotion-runbook.md).
The canonical entrypoint is `cargo xtask promote`; do not hand-build a raw
`swarm/main` source branch.

### Read-only planning

Resolve the current exact head after fetching both remotes:

```bash
git fetch origin --prune
git fetch swarm --prune
swarm_head="$(git rev-parse swarm/main)"

cargo xtask promote --swarm-sha "$swarm_head" --dry-run \
  > target/source-of-truth/promote-plan-1.json
cargo xtask promote --swarm-sha "$swarm_head" --dry-run \
  > target/source-of-truth/promote-plan-2.json
cmp target/source-of-truth/promote-plan-1.json \
    target/source-of-truth/promote-plan-2.json
```

PowerShell equivalent:

```powershell
git fetch origin --prune
git fetch swarm --prune
$swarmHead = (git rev-parse swarm/main).Trim()

cargo xtask promote --swarm-sha $swarmHead --dry-run |
  Set-Content -NoNewline target/source-of-truth/promote-plan-1.json
cargo xtask promote --swarm-sha $swarmHead --dry-run |
  Set-Content -NoNewline target/source-of-truth/promote-plan-2.json

if ((Get-FileHash target/source-of-truth/promote-plan-1.json).Hash -ne
    (Get-FileHash target/source-of-truth/promote-plan-2.json).Hash) {
  throw "promotion dry-runs are not deterministic"
}
```

Review the structured plan, not only the exit code. Confirm:

- `source_head`, `swarm_head`, and merge base are the expected exact commits;
- the routed workflow and terminal aggregate belong to that exact swarm SHA;
- the included swarm PR receipt list is complete;
- every differing path has an exact source/swarm tree entry, effect, and basis;
- every source-retained path is backed by a current bounded source-authority
  decision;
- every exceptional take-swarm path is backed by a current bounded transition
  decision;
- the deterministic overlay SHA and source branch/PR action are explicit; and
- no mutation occurred during either dry-run.

A non-zero dry-run is a repair queue. Do not bypass it with an old
`--source-ref`, a raw branch push, a guessed receipt, or `--allow-historical`.
Historical mode is for explicit historical diagnosis; it is not permission to
prepare a PR against a different current source head.

### Prepare the source promotion

After the exact dry-run is deterministic and fully reviewed:

```bash
cargo xtask promote --swarm-sha "$swarm_head"
```

A real execution must:

- re-resolve current source and swarm state;
- require source merge control, including the exact
  `reject-routine-bot-pr` Source Automation Guard check;
- create or update one deterministic source-local promotion branch and PR;
- refuse an incompatible existing branch or PR;
- preserve source-owned release/governance paths through the exact overlay;
- emit the machine-readable receipt; and
- stop before merge, tag, publication, signing, or release execution.

The source promotion PR must pass source CI at its exact head. Review its actual
file diff and proof sections. The PR body must not claim that release authority,
publication, or tagging has moved to swarm.

### Merge and close out the checkpoint

Merge the source promotion with a regular merge commit. Never squash it.

```bash
cargo xtask promote --swarm-sha "$swarm_head" --verify-only
cargo xtask repo-contract-report
```

Verify:

- the exact overlay landed as a two-parent source checkpoint;
- `Shiplog-Swarm-Head`, `Shiplog-Source-Head`, and the resolution-plan identity
  match the reviewed transaction;
- source post-merge routed CI passed at the exact source merge result;
- source carries no unexplained product commits;
- only current approved source governance differs; and
- the bounded manifest and generated current-promotion view are closed out.

A required post-promotion closeout is evidence for the landed transaction. It is
not the prohibited pattern of repeatedly opening receipt-only PRs merely to keep
a moving pending list cosmetically current.

## Phase 3 — Prepare the release on source

Start a focused release-prep branch from current `EffortlessMetrics/shiplog/main`
after promotion and closeout are green.

Allowed release-specific changes include:

- workspace/package version and lockfile package-version alignment;
- freezing `[Unreleased]` into the new version/date and opening a fresh empty
  `[Unreleased]` section;
- final README/install/usage text for the release;
- the versioned release decision and readiness ledger;
- the versioned execution handoff with pre-publication placeholders;
- release-note and package-channel metadata; and
- narrowly authorized source-owned release workflow/configuration changes.

If release prep discovers a product, shared documentation, test, or ordinary CI
defect, stop. Fix it in swarm, merge it, promote the new exact head, and restart
source release prep from the new source main.

### Source preflight

Run from the source release-prep branch. On Windows, run the Bash scripts through
WSL or Git Bash; the source Release workflow on GitHub remains the authoritative
Linux execution of those scripts.

```bash
cargo xtask ci-small
cargo xtask check-policy-ledgers
cargo xtask check-doc-artifacts
cargo xtask check-goals
cargo xtask check-support-tiers
cargo xtask check-workflows --mode blocking-allowlist
cargo xtask check-file-policy --mode blocking-allowlist
cargo xtask check-automation-authority --repository-role source
cargo xtask promotion-state --check
cargo xtask repo-contract-report

bash scripts/package-version-audit.sh
bash scripts/package-boundary-audit.sh
bash scripts/package-proof.sh
bash scripts/publish-dry-run.sh
bash scripts/check-release-hold.sh vX.Y.Z

git diff --check
```

Confirm independently:

- every publishable workspace package reports the intended version;
- `policy/publish-allowlist.toml` contains only intended public packages;
- the `publish.release` marker, when updated, is treated as documentation and
  not misrepresented as a gate;
- `cargo package --list` contains no private, generated, or unintended files;
- `cargo publish --dry-run --locked` succeeds for the allowlisted order;
- source automation authority is `source`, not `swarm`;
- source release workflow write permission remains narrowly job-scoped; and
- exact source PR and post-merge required checks are green.

Do not tag from an unmerged PR head. Merge the release-prep PR, then verify the
exact resulting `shiplog/main` commit and its post-merge CI.

## Phase 4 — Stage the exact tagged artifacts

Tag only the exact merged source commit approved for release:

```bash
git switch main
git pull --ff-only origin main
git status --short

git tag -a vX.Y.Z -m "shiplog vX.Y.Z"
git push origin vX.Y.Z
```

The tag push invokes the source `Release` workflow. A manual workflow dispatch
is only for an existing explicit semver tag and requires the owner-approval
input. It does not replace the tag or authorize an untagged build.

The source workflow must keep the GitHub release as a draft while it proves the
exact tag. Require successful completion of the applicable jobs:

- `Release Preflight`;
- package proof and allowlisted publish dry-run;
- Linux x86-64 binary build;
- macOS Intel binary build;
- macOS Apple Silicon binary build;
- Windows x86-64 binary build;
- draft GitHub Release creation;
- asset upload and `SHA256SUMS.txt` generation;
- release validation;
- first-use acceptance on all four supported targets; and
- release-mode integration tests.

Check that every job, artifact, and draft release points at the same immutable
tag SHA. Do not substitute a local rebuild for a failed downloaded-artifact or
first-use test.

If any exact-tag gate fails:

- keep the GitHub release draft;
- do not publish crates.io;
- do not make the GitHub release public;
- do not move or reuse the tag; and
- fix the defect through swarm and cut the next patch version.

## Phase 5 — Publish

Only after the exact tag workflow is green and its artifacts are reviewed:

```bash
cargo publish -p shiplog --locked
gh release edit vX.Y.Z --draft=false --latest
```

Verify public state:

- crates.io shows the intended version and package metadata;
- `cargo install shiplog --version X.Y.Z --locked` installs successfully;
- the GitHub release is public, non-prerelease unless explicitly intended, and
  marked latest when appropriate;
- all four binaries and `SHA256SUMS.txt` are present;
- downloaded binaries report the expected `shiplog X.Y.Z` version;
- versionless installer paths resolve the public assets; and
- Homebrew and Scoop updates use the final public asset hashes and pass native
  package validation.

Signing, notarization, SmartScreen reputation, or other platform-trust proof is
claimed only when the corresponding release lane actually exists and passed.
Checksums alone are not a signing claim.

## Phase 6 — Close out the release

Finalize the versioned readiness ledger and execution handoff with observed
identities:

- exact tag and source commit;
- source Release workflow run;
- successful platform jobs;
- public crates.io version;
- public GitHub Release and asset list;
- first-use acceptance results;
- installer and package-channel results;
- known limitations and deferred work; and
- rollback/yank decision, if any.

Set the release decision and readiness status to shipped only after public-state
verification. Keep a new `[Unreleased]` section open for subsequent work.

Any release-execution documentation or workflow adjustment committed directly
to source must be ported back to swarm before normal promotion resumes. A
source-only release-governance exception does not authorize permanent source
product or shared-documentation drift.

## Abort and rollback rules

### Before the tag

Fix the candidate in swarm, rerun proof, and promote again. Source release prep
may be abandoned or rebuilt from the new source main.

### After the tag but before crates.io publication

Keep the GitHub release draft. Do not force-move the tag. Fix through swarm and
cut the next patch tag.

### After crates.io publication

Published crate bytes cannot be replaced. Use a new patch release. Yank only
when the published version should no longer be selected automatically, and
record the reason in the release handoff.

### Incorrect source promotion

Revert the regular merge commit in source and pause release work until the
source/swarm divergence is understood. Never rewrite `shiplog/main` history.

## Copyable release checklist

### Candidate on swarm

- [ ] Release version and user-facing theme are decided.
- [ ] Included and deferred scope is explicit.
- [ ] `[Unreleased]` is curated from merged behavior.
- [ ] README, guides, config docs, support tiers, and examples are current.
- [ ] Open PRs/issues are included, deferred, closed, or blocking explicitly.
- [ ] `cargo xtask ci-small` passes.
- [ ] Policy, docs, goals, support-tier, workflow, file-policy, and authority
      checks pass.
- [ ] `promotion-state --check` and `repo-contract-report` pass.
- [ ] Exact swarm `Shiplog Rust Small Result` is green.

### Promotion

- [ ] Two current-head dry-runs are byte-identical.
- [ ] Exact source/swarm/merge-base and required CI evidence are correct.
- [ ] Included PR receipts are complete.
- [ ] Every path decision has exact entries and a current basis.
- [ ] Real `cargo xtask promote` creates or reuses one compatible source PR.
- [ ] Source promotion PR exact-head CI is green.
- [ ] Source promotion is regular-merged, never squashed.
- [ ] `--verify-only` and source post-merge `repo-contract-report` pass.
- [ ] Promotion manifest/current view and consumptive decisions are closed out.

### Source release-prep PR

- [ ] Version and lockfile package versions are aligned.
- [ ] Changelog version/date is frozen and a new `[Unreleased]` exists.
- [ ] Release decision, readiness ledger, and handoff are prepared.
- [ ] README/install/release-note text matches the candidate.
- [ ] Source-role automation and release authority remain correct.
- [ ] Package version/boundary audits pass.
- [ ] Package proof and allowlisted publish dry-run pass.
- [ ] Release-hold guard passes for the exact tag.
- [ ] Source PR and post-merge required checks are green.

### Tag and draft artifacts

- [ ] Annotated semver tag points at exact merged source main.
- [ ] Tag has not been moved or reused.
- [ ] Source Release workflow targets the exact tag SHA.
- [ ] Four platform builds pass.
- [ ] Checksums and draft release assets are complete.
- [ ] Release validation passes.
- [ ] Four-platform first-use acceptance passes.
- [ ] Release-mode integration tests pass.

### Publication and closeout

- [ ] crates.io publication succeeds and public metadata is verified.
- [ ] GitHub release is made public only after exact-tag proof.
- [ ] Public assets, checksums, installers, and `--version` are verified.
- [ ] Homebrew and Scoop updates use final public hashes and pass validation.
- [ ] Versioned readiness and handoff record observed run IDs and public state.
- [ ] Deferred work and limitations are explicit.
- [ ] Source-only release docs/workflow changes are ported back to swarm.
- [ ] New `[Unreleased]` work can proceed without unresolved release drift.
