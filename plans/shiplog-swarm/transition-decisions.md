# Bounded Transition Decisions

This record captured the reviewed decisions for the bounded promotion of
`shiplog-swarm` into `shiplog` that landed as source PR #682. It is historical
evidence, not a source-authority policy, and it does not authorize a future
promotion by itself.

## Closeout

The decisions in this ledger were bound to exact tree entries and consumed by
`EffortlessMetrics/shiplog#682`, regular-merge source commit
`2f69daf921d74fb9472584c2a0df31857cfa2384`. The post-merge routed proof is
run `30806322336`. The current bounded state is recorded in
[`promotion-state.toml`](promotion-state.toml); these entries remain here so
their evidence and review reasoning are not lost.

## Decision boundary

The source target reviewed here is
`b31d5f6d9700698b463d8f2b71b9d48a191f433c`. The swarm target used to inspect
the entries is `e94cce7087780fce6f9ef09d9c1480f12c15061b`, which includes the
merged transition-evidence PR #316. The later manifest update must re-read and
bind the exact entries at its own promotion target.

For the eight paths below, the decision is:

> Take the exact swarm tree entry for this bounded promotion and discard the
> source-side transition copy.

This is an exceptional, consumptive resolution. It does not make swarm
permanently authoritative for any path and does not change
`policy/source-only-paths.toml`. The later manifest entry must use
`resolution = "discard_source"`, the exact source and swarm tree entries, the
merged PR containing this decision as `decision_receipt`, its actual merge SHA
as `decision_merge_sha`, and a `consumed_by` promotion checkpoint.

## Source #666: direct-command contract

Source PR `EffortlessMetrics/shiplog#666` merged as
`d88d59a8afd7eee445c3214c5f89ca7ccd50e4de` with the stated purpose of
removing RTK from live agent guidance, proof commands, support-tier proofs,
validators, and regression fixtures. Swarm PR
`EffortlessMetrics/shiplog-swarm#274` merged as
`1ca35e97ba506062376e6f78b6633003e25db963` carried that contract on the
development trunk first.

The source-side copies are transition-era ports of the swarm control-plane
contract, not independent release behavior. Taking the current swarm entries
preserves the active goal, agent, specification, support-tier, command, and
promotion-runbook state as one coherent set. Historical RTK receipts remain
historical evidence and are not being rewritten.

| Path | Source target entry | Swarm target entry | Decision reason |
|---|---|---|---|
| `.codex/goals/active.toml` | `100644/blob/a8992784bb0da816d3506aa392310a1555adcfe3` | `100644/blob/e26282c9fa2e9d5102f2fb918be3494cbf40d46d` | The swarm active-goal commands are the current operational contract. |
| `AGENTS.md` | `100644/blob/5d24413ccc6b77e293d7c035be16440c82934322` | `100644/blob/85abfd3f0b7fe11d1b5c2a98bd64f3e1159521b9` | The swarm agent instructions are the current development-lane contract. |
| `docs/specs/SHIPLOG-SPEC-0011-shiplog-swarm-cutover-contract.md` | `100644/blob/f7d992c257002e21a812ba06a64866cb0e318ebe` | `100644/blob/5e1ce6c156a072ce62b73aae644e0a44704e9516` | The swarm specification carries the reviewed direct-command contract. |
| `docs/status/SUPPORT_TIERS.md` | `100644/blob/b9360f08d0468d7e7d1b5b79b98d69e0bf8fd0a2` | `100644/blob/301913b6034d03b6fce6b1adb4ad48b22a89bac7` | The swarm support claims must point at the commands users and agents can run. |
| `docs/xtask.md` | `100644/blob/55a4b4ee289575f2c3478e9bc9ac06b2ac382b93` | `100644/blob/6fcb4394cfaa1323a355cb627c100fd429bed27c` | The swarm command reference is the maintained implementation-facing guide. |
| `plans/shiplog-swarm/promotion-runbook.md` | `100644/blob/b56d190691c1c674918da9989d79d112b7d45e92` | `100644/blob/da7873d45b82bce0192eb959d47478c41ee891aa` | The swarm runbook is the maintained promotion procedure and receipt boundary. |

## Source #676: residual promotion guidance

Source PR `EffortlessMetrics/shiplog#676` merged as
`b31d5f6d9700698b463d8f2b71b9d48a191f433c` to remove residual RTK generator
guidance and anchor the current-promotion summary to the real runbook. Swarm
PR `EffortlessMetrics/shiplog-swarm#282` merged as
`dcbf5b3d8fe94dbc57e816ac737677b1caed4ee4` carried the reviewed change first.

The source `current-promotion.md` is a generated operational summary, so the
swarm copy must remain aligned with the swarm promotion manifest. The source
`repo_contract_report.rs` contains the same residual-command cleanup, while
the swarm copy also contains the current control-plane implementation. The
source-side transition copy does not represent separate release authority.

| Path | Source target entry | Swarm target entry | Decision reason |
|---|---|---|---|
| `plans/shiplog-swarm/current-promotion.md` | `100644/blob/4e28955f45a4aa42a5702ee7f6049d8bb496781a` | `100644/blob/1f0d11fc5fe0b4deec71054a8940d0ba26a43c88` | The generated summary must follow the swarm manifest and current promotion state. |
| `xtask/src/tasks/repo_contract_report.rs` | `100644/blob/af4b3f7193d1a72f5aec735134e29809e9726652` | `100644/blob/fab37f26423e06a97326b521706c7067432a571f` | The swarm implementation includes the source cleanup plus the maintained control-plane behavior. |

## Current source-authoritative release and security surfaces

This is the refreshed decision record for the exact promotion targets now under
review:

- source target: `175e8c8cee8110c4cf4a42d5534f5dffe45bf426`
- swarm target: `3f1ee6966e70d3651492345d6ffa86b0ec61b12d`

The earlier source-authority entries were consumed by source promotion #682 and
must not be reused for these newer trees. The source-only policy remains
narrow, but policy alone cannot silently discard a swarm-side change. For each
path below, the reviewed decision is:

> Retain the exact source tree entry for this bounded promotion and discard the
> swarm-side change. The later manifest-binding PR must record this decision's
> merged PR and merge SHA as `source_authority`, then consume it at the source
> promotion checkpoint.

These paths are release, security, merge-control, or repository-role surfaces.
This decision does not make source content generally authoritative for future
swarm changes; any later target divergence requires a new reviewed decision.

| Path | Source target entry | Swarm target entry | Decision reason |
|---|---|---|---|
| `.github/workflows/droid-review.yml` | `100644/blob/196b2f2316e38c6875e87c76c0969195f45bea50` | `100644/blob/353675f6651afacad517e26d2e793ce571a72a77` | Canonical source review automation may publish reviews but cannot write product contents. |
| `.github/workflows/droid-security-scan.yml` | `100644/blob/8dba098967af002229081dc41e30b8ac39ceb842` | `100644/blob/ce1faad0893020958cc836a24c0339e07e7c8d50` | Canonical source scheduled security verification remains read-only while swarm may originate remediation. |
| `.github/workflows/droid.yml` | `100644/blob/e80ab51682e696202d793fdf175cbe3fd2ab82a3` | `100644/blob/ec4b5257d46a456b1298e7649bcfb3b0921ddb3a` | Canonical source review automation may publish comments but cannot mint identity or write product contents. |
| `.github/workflows/release.yml` | `100644/blob/77c5ff31f0da4c15c4e9b1c3590b66ff34d95e35` | `100644/blob/1c8f97214cf86f0578d4d1678ff92b3f31673d94` | Canonical source retains explicitly authorized release-writer jobs and the source-side explicit-shell fix; swarm remains verification-only. |
| `.github/workflows/security.yml` | `100644/blob/22ca71161234b96b50b835bfc00fa9da64d00b6f` | `100644/blob/8ab39d95d4f579f3d08c2c288053e3a0fc9d4f1f` | Canonical source scheduled security verification declares read-only permissions and the swarm remediation handoff. |
| `.github/workflows/source-automation-guard.yml` | `100644/blob/2689c94267bcf8ce5b162b84003dd6d8f7b4c293` | `100644/blob/981788072211f3d01d24731c4b6e978309fbe581` | Canonical source owns the merge-control guard and its synthetic proof dispatch after source PR #686. |
| `policy/automation-authority.toml` | `100644/blob/43ed9b86371066d32096bfaca551d7ce1c155cc2` | `100644/blob/aa3b2ba0ca50acf4a5540a7b2e8f3eca4c7e26cc` | Canonical source owns the source-role policy from #679 while swarm retains its distinct swarm-role policy. |

## Binding and deliberate exclusions

This PR records the exact decision but does not claim that the promotion is
ready. A later substantive manifest-binding PR must:

- name this record's merged PR and actual merge SHA as the decision receipt;
- copy these exact source/swarm tree entries into active `source_authority`
  entries bound to the current targets;
- prove the receipt is reachable from the exact swarm target;
- consume the entries only at the promotion checkpoint.

This decision does not resolve the nine two-sided control-plane blockers in
issue #349, source branch-protection settings in #294, source #657 `Cargo.lock`,
or any other path not listed above. No source refs, source files, promotion
manifest entries, or repository settings are changed by this record.

The intended claim is narrow: these seven current source-authoritative paths
have a named, reviewable source-retention decision for one future promotion;
the planner must remain fail-closed until that decision is bound and consumed.

## Current two-sided control-plane reconciliation

This is the refreshed decision record for the nine control-plane paths that
both repositories changed after the last completed source promotion:

- historical source target: `175e8c8cee8110c4cf4a42d5534f5dffe45bf426`
- promotion decision target: `541ef085f68711e3382c2ed3ddef7febf004a783`
- historical source receipt: `EffortlessMetrics/shiplog#682` at
  `2f69daf921d74fb9472584c2a0df31857cfa2384`

The source promotion receipt is historical evidence of the source-side copy;
it is not the decision to abandon that copy. For each path below, the reviewed
promotion decision is:

> Take the exact swarm tree entry for this bounded promotion and discard the
> source-side transition copy. Bind the decision as `resolution =
> "discard_source"` under the source #682 transition, using this record's
> merged PR and merge SHA as the decision receipt, then consume it at the
> promotion checkpoint.

This is a bounded take-swarm decision, not permanent swarm authority. The
current swarm copies carry the reviewed control-plane behavior: synthetic
source-guard proof, exact workflow-shell policy, current branch-protection
observation, and the current promotion ledger. Future divergence requires a
new exact decision.

| Path | Source target entry | Swarm target entry | Decision reason |
|---|---|---|---|
| `docs/governance/automation-authority.md` | `100644/blob/323cc19cbbc5f875a9e6384380db17acdab4726c` | `100644/blob/0492fc378c453095c109f17d12b253793b6b233a` | Swarm carries the current synthetic actor-proof and fail-closed automation contract. |
| `docs/status/SUPPORT_TIERS.md` | `100644/blob/301913b6034d03b6fce6b1adb4ad48b22a89bac7` | `100644/blob/00d79b3d9d6f40f5ed2532eddc9119e2fb06a98a` | Swarm carries the current observational branch-protection support claim and proof boundary. |
| `plans/shiplog-swarm/current-promotion.md` | `100644/blob/2cf46f2668123bb939bd5c92beb02f8e375a5664` | `100644/blob/3425fd5cb0eb3d2e13c3d53d19e44eb180bce09d` | The generated promotion view must follow the current swarm manifest and decision ledger. |
| `plans/shiplog-swarm/promotion-state.toml` | `100644/blob/bf3cbaa556a3b3249f952742bb2c6b28847f3876` | `100644/blob/3eae48353c759d5936dd1fcbc3cce21b5a1debef` | The swarm manifest contains the exact active source-authority receipts and current pending range. |
| `plans/shiplog-swarm/transition-decisions.md` | `100644/blob/bbe7516331ab7e132aea0026049d61cb1040accd` | `100644/blob/a2fa27582813c5bc8c560a29000113a7ab4325c5` | The swarm ledger is the maintained record of current transition evidence and bounded decisions. |
| `policy/source-only-paths.toml` | `100644/blob/979f4d444bfab4d4a40c541551253d490f1ac3b6` | `100644/blob/31defb7dea7b915ae5cf3c84d6f330e66e379de3` | Swarm carries the reviewed source-guard allowlist required by the current governance receipt. |
| `xtask/src/tasks/automation_authority.rs` | `100644/blob/60184651df83b939abc0a1202f54410da4d6d71e` | `100644/blob/7916833ea3f286c3eb4f64d7ceee492b15541871` | Swarm carries the fail-closed static validator for synthetic actor proof and guard shape. |
| `xtask/src/tasks/file_policy.rs` | `100644/blob/a6cb804a9c2fa5bb7d2e9d6f9a7aaad8367a5e59` | `100644/blob/9c337a7ecac7f7acb049b15ecdd9f3b2987ef2f7` | Swarm carries the regression policy rejecting unsupported matrix context in workflow shells. |
| `xtask/src/tasks/repo_contract_report.rs` | `100644/blob/fab37f26423e06a97326b521706c7067432a571f` | `100644/blob/583b32574ca60f3b60517bf1724c76c9d1041644` | Swarm carries the current exact promotion, branch-protection, and source-of-truth report behavior. |

## Binding and deliberate exclusions

This record does not change the promotion manifest. A later substantive
manifest-binding PR must add one source #682 transition entry with these nine
paths, exact source/swarm tree entries, `resolution = "discard_source"`, this
record's merged PR and actual merge SHA, and an explicit `consumed_by`
checkpoint. The planner must remain fail-closed until that binding exists.

The decision does not resolve source branch-protection settings in #294, the
source `Cargo.lock` dependency gap, or any path outside these nine. No source
refs, source files, repository settings, or real promotion are changed.
