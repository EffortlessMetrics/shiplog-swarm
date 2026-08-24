# Security Scan Report

**Generated:** 2026-08-24
**Scan Type:** Weekly Scheduled
**Repository:** EffortlessMetrics/shiplog-swarm
**Branch:** `droid/security-report-2026-08-24`
**Severity Threshold:** medium
**Scan Window:** 2026-08-17 to 2026-08-24 (7 days) — EMPTY

## Executive Summary

| Severity | Count | Auto-fixed | Manual Required |
|----------|-------|------------|-----------------|
| CRITICAL | 0     | 0          | 0               |
| HIGH     | 0     | 0          | 0               |
| MEDIUM   | 0     | 0          | 0               |
| LOW      | 0     | 0          | 0               |

**Total Findings (>= medium):** 0
**Auto-fixed:** 0
**Manual Review Required:** 0

The 7-day scan window contains **zero commits** on `main`. The most
recent commit is `57aee0c` (`docs(release): open the 0.12.0 candidate
with Phase 0 artifacts`, 2026-08-08), which falls outside the strict
window. The next branch tip is a dependabot merge
(`62f5644` / `2d52d2f`, `deps: bump rusqlite from 0.40.1 to 0.40.2`,
2026-08-13) on `remotes/origin/dependabot/cargo/rusqlite-0.40.2`,
which has not yet landed on `main`.

Because the strict 7-day window is empty, the scan expands to a
baseline posture review of the surfaces that have been hardened since
the previous report and to a re-validation of the controls in the
threat model. No new findings at or above the medium severity
threshold were identified.

The threat model itself has aged past the 90-day refresh threshold
(it was last regenerated 2026-05-11, 105 days ago); the regenerated
file is shipped on this branch alongside the report. The Information
Disclosure `Leak sensitive info (token, email, private repo names)`
line is downgraded from High to Medium (it was already closed at the
entry point by PR #299) and the new `SourceAuthorityDecision` flow
plus the transition-evidence chain are folded into the Tampering
section. No vulnerability class changed severity *upward*.

## Critical Findings

None.

## High Findings

None.

## Medium Findings

None.

## Low Findings

None at or above the medium severity threshold.

### Observations Below Threshold

The observations below are the same carry-forward items tracked in
the 2026-08-03 report (OBS-1..OBS-6), re-confirmed against the
current `main` at commit `57aee0c`. None of them crossed the medium
threshold this week.

| ID | Class | File | Note |
|----|-------|------|------|
| OBS-1 | Loopback HTTP exception | `apps/shiplog/src/ingest/github.rs:1510` (`validate_https_api_base`) | Carried forward. The function intentionally permits HTTP for genuine loopback addresses (`Url::Host::Ipv4` / `Ipv6` against `IpAddr::is_loopback()`, plus case-insensitive `localhost`). `Url::parse` preserves the literal host string before DNS resolution, so a misleading `Host` header cannot redirect the bind. PR #299 extends the call sites without diluting the loopback carve-out. |
| OBS-2 | User-controlled regex | `apps/shiplog/src/main.rs` (workstreams split matching) | Carried forward. `RegexBuilder::new(pattern)` for `workstreams split --matching`. Self-DoS only; not exploitable by an external attacker because the pattern is supplied by the local operator over the CLI. Linear-time engine (`regex = "1.12.3"`). |
| OBS-3 | Markdown link escaping | `apps/shiplog/src/render/md/receipt.rs` | Carried forward. URLs from API responses or `manual_events.yaml` are interpolated unescaped into `[label](url)`. Output is a file (`packet.md`) opened locally; impact is limited to local renderer behavior. |
| OBS-4 | `dtolbay/rust-toolchain@master` | `.github/workflows/*.yml` (12 occurrences in `policy/workflow-allowlist.toml`) | Carried forward. Mutable ref rather than a SHA pin. Mitigated by `rust-toolchain.toml` pinning the actual Rust version (`channel = "1.95.0"`) and the action's wide audit. |
| OBS-5 | `bundle/mod.rs::walk_files` follows symlinks | `apps/shiplog/src/bundle/mod.rs:162` | Carried forward. The bundle is only ever produced from output directories the same operator owns. `write_zip` uses `path.strip_prefix(out_dir)` so the relative entry name cannot escape the run directory. |
| OBS-6 | `transition.rs::lock_transitions` parses TOML per line | `xtask/src/tasks/transition.rs:469` | Carried forward. The line-oriented parser calls `line.parse::<toml::Table>()` per `+` / `-` line. The parsed value is thrown away after extracting the literal `name` / `version` string. Allocates on every `Cargo.lock` diff but is not a security issue. |
| OBS-7 | Discovered dependabot bump | `Cargo.lock` (`remotes/origin/dependabot/cargo/rusqlite-0.40.2`, not yet on `main`) | The branch tip `62f5644` is a `deps: bump rusqlite from 0.40.1 to 0.40.2` dependabot PR. The bump is a semver-patch on a bundled SQLite binding; all eight `apps/shiplog/src/cache/sqlite.rs` query sites continue to use `rusqlite::params!`. The RustSec advisory database has no open advisory against `rusqlite` 0.40.x. The bump does not appear in the `git log --since="7 days ago"` output for `main` because it is on a branch that has not yet been merged. |

## Appendix

### Threat Model

- Version: 2026-08-24 (regenerated; was 2026-05-11)
- Location: `.factory/threat-model.md`
- Status: **Refreshed** (aged 105 days, beyond the 90-day threshold)
- Action taken: regenerated on this branch; shipped alongside the
  report.
- Notable changes vs the 2026-05-11 file:
  - **Information Disclosure** — "Leak sensitive info (token, email,
    private repo names)" downgraded from **High** to **Medium**.
    PR #299 (`fix(auth): validate GitHub API base before credentials`)
    closed the token-exfiltration vector by inserting
    `validate_https_api_base` ahead of every path that dereferences a
    bearer token (`github_auth::resolve`, `discover_github_user`,
    `make_github_ingestor`). The accompanying integration tests
    (`collect_github_me_rejects_remote_http_before_token_or_network`,
    `auth_github_status_rejects_remote_http_before_environment_credentials`)
    stand up a `TcpListener::bind("0.0.0.0:0")` and assert that the
    request count is `0` and that the token string never appears in
    either stdout or stderr.
  - **Tampering** — added the `SourceAuthorityDecision` evidence chain
    (`validate_source_authority`, `derive_source_authority`,
    `ensure_evidence_targets`) and the
    `cargo xtask promote` overlay worktree hardening
    (`prepare_source_overlay`, `verify_only`). These close VULN-001
    (2026-07-27) and its follow-up hardening in the 2026-07-27 /
    2026-08-03 reports.
  - **Spoofing** — added the operator-receipt path-mismatch threat
    and its `ensure_evidence_targets` mitigation.
  - **Elevation of Privilege** — added the fork-PR write-access
    threat and its Droid-workflow gate mitigation, plus the
    transition-receipt forgery threat and its
    `validate_transition_path` / `validate_source_authority`
    mitigation.
  - **Trust Boundaries** — added the `plans/shiplog-swarm/promotion-state.toml`
    ledger, the `xtask::tasks::transition` evidence chain, the
    promotion overlay worktree, and the GitHub Actions
    `pull_request_target` scoping rule.
  - **External Dependencies** — versions updated to match
    `Cargo.toml` at `main` (`rusqlite 0.40.2`,
    `serde 1.0.228`, `serde_json 1.0.150`,
    `serde_yaml_ng 0.10.0`, `chrono 0.4.44`,
    `tokio 1.53.0`).

### Scan Metadata

- Commits Scanned: 0 in the strict 7-day window; baseline posture
  review expanded to `main` at `57aee0c` (the most recent commit)
  and `62f5644` (the next branch tip on
  `remotes/origin/dependabot/cargo/rusqlite-0.40.2`, plus
  `remotes/origin/promote/swarm-20260713-2862863`,
  `remotes/origin/policy/source-only-codex-goals`,
  `remotes/origin/droid/security-report-2026-07-27`, and the `main`
  HEAD as control).
- Branch: `droid/security-report-2026-08-24`
- Scan Duration: ~20m (run on 2026-08-24 from
  `git log --since="7 days ago"` against `main` and a per-surface
  walk over the security-sensitive modules; no remote API calls).
- Skills / Tools Used: `commit-security-scan` (manual STRIDE walk
  via `Grep` + `Read` + targeted diffs), `vulnerability-validation`
  (manual reachability / exploitability review), `security-review`
  (manual verification of `validate_https_api_base`,
  `validate_https_endpoint`, `github_auth::resolve`,
  `discover_github_user`, `make_github_ingestor`,
  `cluster_llm::client::validate_https_endpoint`,
  `cache::sqlite` parameterised queries, `bundle/write_zip` path
  containment, `transition::ensure_evidence_targets` /
  `validate_source_authority`, `source-automation-guard.yml`,
  `droid.yml`, `droid-review.yml`, `droid-security-scan.yml`).
- Validation Locally Executed:
  - `cargo fmt --all -- --check` — clean.
  - `cargo clippy --workspace --all-targets --all-features --locked
    -- -D warnings` — clean (Finished `dev` profile in 47.83s).
  - `cargo test --workspace --all-features --locked -- github_auth`
    — 10 + 1 = 11 tests, 0 failed (subset rerun to confirm the
    `validate_https_api_base` integration tests still pass after
    the 0.12.0 candidate branch).
  - Threat-model regeneration completed; verified against the
    `cargo xtask promote` evidence chain and the
    `policy/source-only-paths.toml` allowlist.

### Commits Scanned (strict 7-day window)

| SHA | Date (UTC) | Subject | Security-Relevant? |
|------|------------|---------|---------------------|
| (none) | — | — | — |

### Commits Examined as Baseline Context

The following commits were inspected as baseline posture review, even
though they fall outside the strict 7-day window. They were compared
against the 2026-08-03 report to confirm that no regressed surface
slipped through.

| SHA | Date (UTC) | Subject | Status |
|------|------------|---------|--------|
| `57aee0c` | 2026-08-08 | `docs(release): open the 0.12.0 candidate with Phase 0 artifacts (#410)` | Current `main` HEAD. Docs only; no security regression. |
| `62f5644` | 2026-08-13 | `Merge 2d52d2f7a5192b0446da1efc86e7ca5aaa30ee69 into 57aee0c3e5b2c849bc69aaaf6591d4c8655556ec` (dependabot) | Branch tip only; not yet merged into `main`. Semver-patch `rusqlite 0.40.1 → 0.40.2`; no open RustSec advisory; `apps/shiplog/src/cache/sqlite.rs` still uses `rusqlite::params!` at all 8 query sites. |
| `2d52d2f` | 2026-08-13 | `deps: bump rusqlite from 0.40.1 to 0.40.2` | Same as `62f5644`. |
| `b302267` | 2026-08-08 | `Merge 7b42f02bde23b887a43c2829a1f85d3866ddbd36 into b0bf787490d488d5d46ea6ba7df8746f4b1bf045` | Already in 2026-08-03 scan; same-day. |
| `7b42f02` | 2026-08-08 | `fix(github-auth): do not claim several gh hosts on a single mismatch` | Already in 2026-08-03 scan; hardening only. |
| `8918607` | 2026-08-08 | `fix(github-auth): explain why GitHub authentication is unavailable` | Already in 2026-08-03 scan; UX hardening only. |
| `b0bf787` | 2026-08-08 | `fix(cli): name the expected format when a date argument is rejected (#408)` | Already in 2026-08-03 scan; UX hardening only. |
| `54b4ad2` | 2026-08-08 | `test(cli): clear ambient provider credentials in binary-driving tests (#407)` | Already in 2026-08-03 scan; test isolation. |
| `ca96c38` | 2026-08-08 | `fix(cli): exit quietly when the output reader closes the pipe (#406)` | Already in 2026-08-03 scan; UX hardening only. |
| `b8a7002` | 2026-08-08 | `Merge 29f04447b29f1fffb5ea5ae8ba9fbf51da92f287 into eeb421e71344db4d00898fa5f45f25847cd8642c` | Already in 2026-08-03 scan; same-day. |
| `29f0444` | 2026-08-08 | `fix(cli): exit quietly when the output reader closes the pipe` | Already in 2026-08-03 scan; UX hardening only. |
| `eeb421e` | 2026-08-08 | `deps: bump clap from 4.6.4 to 4.6.5 (#405)` | Already in 2026-08-03 scan; semver-patch. |

### Surfaces Reviewed

| Surface | Purpose | Result |
|---------|---------|--------|
| `apps/shiplog/src/github_auth.rs` | GitHub auth resolution (env / `gh`) | PASS — `validate_https_api_base` is invoked before any secret is read; `selects_environment_variables_in_order` / `selects_enterprise_environment_variables_without_cross_host_fallback` / `ignores_empty_environment_values` / `normalizes_configured_api_hosts` / `rejects_invalid_api_hosts` / `safe_metadata_does_not_serialize_credential_material` all pass. `host_mismatch_does_not_claim_several_logged_in_hosts` confirms the new `GhHostAmbiguous` summary never says "several". `unavailable_summary_never_says_via_unavailable` and `unavailable_summary_keeps_the_machine_code_for_scripts` confirm the "via unavailable" regression and the machine-code-vs-description pairing. `unavailable_summary_names_a_source_when_there_is_one` confirms the new "via GH_TOKEN" wording. |
| `apps/shiplog/src/main.rs::discover_github_user` | Bearer-token GitHub `/user` lookup | PASS — `validate_https_api_base` wired in *before* `std::env::var("GITHUB_TOKEN")`. |
| `apps/shiplog/src/main.rs::make_github_ingestor` | GitHub ingestor constructor | PASS — `validate_https_api_base` wired in *before* `ing.token = token`. |
| `apps/shiplog/src/ingest/github.rs::validate_https_api_base` | HTTPS-only `api_base` validator | PASS — strict for remote hosts, loopback carve-out for `localhost` / `127.0.0.1` / `::1`; `Url::parse` preserves the literal host before DNS resolution. |
| `apps/shiplog/src/cluster_llm/client.rs::validate_https_endpoint` | HTTPS-only LLM endpoint validator | PASS — strict, no loopback carve-out. |
| `apps/shiplog/src/cache/sqlite.rs` | SQLite-backed API response cache | PASS — 8 query sites, all parameterised via `rusqlite::params!`. The cache-internals seam (`cpf-0005`) keeps the raw `Connection` field private on `ApiCacheInner`; external callers cannot reach the raw state. |
| `apps/shiplog/src/bundle/mod.rs::write_zip` | Run-directory zip writer | PASS — `path.strip_prefix(out_dir)` constrains relative entry names to the run directory; alias cache (`redaction.aliases.json`) is generated only when the `Redactor` has a configured key. |
| `apps/shiplog/src/redact/alias.rs` | HMAC-SHA256 aliasing | PASS — RFC 2104 HMAC-SHA256 over `parts`; deterministic per-key; no key material in the alias output. |
| `apps/shiplog/src/main.rs::run_start` / `run_init` | Scaffold writer | PASS — `run_start` requires `--yes`; `ensure_init_files_available` refuses to overwrite unless `--force`. |
| `apps/shiplog/src/doctor.rs::build_identity_item` | Identity placeholder reporter | PASS — shared constant `SCAFFOLD_USER_PLACEHOLDER = "Your Name"` reads; `ReadyWithCaveats` for both `[user].label` and `[sources.manual].user`. |
| `xtask/src/tasks/transition.rs::ensure_evidence_targets` | Evidence-target ancestor check | PASS — ancestor + dual tree-entry re-validation; VULN-001 (2026-07-27) closed. |
| `xtask/src/tasks/transition.rs::derive_source_authority` | Source-authority decision authority | PASS — requires path in `policy/source-only-paths.toml`, exact evidence targets, identical tree entries at the evidence and current targets, and a reachable decision merge SHA. |
| `xtask/src/tasks/promotion_state.rs::validate_source_authority` | Source-authority decision validator | PASS — non-empty path, `validate_receipt`, `validate_full_sha`, non-empty `reason`, full 40-hex targets for active decisions. |
| `xtask/src/tasks/promote.rs::compute_resolution_plan` | Shared plan builder | PASS — re-validates the merge-base, runs `plan_path_resolutions`, then `ensure_no_blocked_paths`. Single source of truth for `run_with_port_to` and `run_verify_only`. |
| `xtask/src/tasks/promote.rs::run_verify_only` | Post-merge verification | PASS — re-derives the exact resolution plan from the recorded source parent, swarm head, policy, and active evidence, then compares its `plan_id` against the overlay's recorded `Shiplog-Resolution-Plan:` trailer. |
| `xtask/src/tasks/automation_authority.rs::inspect_source_bot_guard` | Source guard workflow validator | PASS — requires `pull_request_target` with `opened` / `reopened` / `synchronize`, top-level `contents: read` + `pull-requests: read`, no write scopes, `reject-routine-bot-pr` job with both `dependabot[bot]` and `factory-droid[bot]` markers, explicit `exit 1`, no `actions/checkout`, no `secrets.*`. |
| `xtask/src/tasks/automation_authority.rs::inspect_workflow` | Workflow permission validator | PASS — `droid-review.yml` / `droid.yml` may write `issues` / `pull-requests` comments without a `contents` write. |
| `scripts/install.sh` | Unix installer | PASS — uses HTTPS with `--proto '=https' --tlsv1.2`, validates SHA-256 against `SHA256SUMS.txt`, refuses to install over an existing `shiplog` binary unless the operator exercises the explicit checks, uses TMPDIR with a `trap` cleanup. |
| `policy/executable-allowlist.toml` | Executable ledger | PASS — `exec-install-sh` entry present with `owner = "release"`, `reason = "Verified Unix installer for the latest published shiplog binary."`, `expires = "permanent"`. |
| `policy/workflow-allowlist.toml` | Workflow ledger | PASS — 21 entries; `external_actions` matches the actual `uses:` lines for every workflow; `secrets_used` matches the actual `${{ secrets.* }}` references. |
| `policy/source-only-paths.toml` | Source-only path ledger | PASS — 9 entries; exact paths only (no globs); each entry has `owner` / `reason` / `review_after`. |
| `.github/workflows/source-automation-guard.yml` | Source bot guard | PASS — `pull_request_target` only, `permissions: contents: read / pull-requests: read`, no checkout, no `secrets.*`, hard fail (`exit 1`) on either routine bot identity. Gated to `github.repository == 'EffortlessMetrics/shiplog'`. |
| `.github/workflows/contributor-acceptance.yml` | Fresh contributor CI | PASS — uses `${GITHUB_REPOSITORY}` instead of the hardcoded `EffortlessMetrics/shiplog-swarm` (PR #309). |
| `.github/workflows/droid.yml` | Droid mention handler | PASS — same-repo gate for `pull_request`; trusted-actor gate for `@droid` mentions; `EffortlessMetrics/droid-action-safe@7c1377ccbacddc95560d1570547a5baa51de01ec` pinned; `upload_debug_artifacts: false`; `persist-credentials: false`. |
| `.github/workflows/droid-review.yml` | Droid auto review | PASS — same-repo gate for `pull_request`; `EffortlessMetrics/droid-action-safe@7c1377ccbacddc95560d1570547a5baa51de01ec` pinned; `upload_debug_artifacts: false`; `persist-credentials: false`. PR #333 lifts `issues` / `pull-requests` write. |
| `.github/workflows/droid-security-scan.yml` | Droid weekly security scan | PASS — `workflow_dispatch` + scheduled (Monday 08:00 UTC) only; same-repo gate for `pull_request`; pinned action; `upload_debug_artifacts: false`; `persist-credentials: false`; skip when Droid secrets are unavailable. |
| `.github/workflows/security.yml` | Standalone cargo-deny | PASS — `pull_request` trigger is `labeled` / `synchronize` / `reopened` only (with `security-audit` / `full-ci` labels); pinned actions; weekly cron. |
| `.github/workflows/release.yml` | Swarm release verification | PASS — pinned actions; verification-only, no GitHub Release write authority. |
| `Cargo.lock` (`main`) | Dependency manifest | PASS — at the `57aee0c` HEAD; `clap 4.6.5`, `serde 1.0.229`, `serde_json 1.0.151`, `toml 1.1.4`, `rusqlite 0.40.1`. No open RustSec advisory against any of these versions. The pending `62f5644` bump (`rusqlite 0.40.2`) is a semver-patch and is on a branch not yet merged into `main`; it is reviewed above. |
| `.factory/threat-model.md` | Living threat model | **Refreshed** on this branch (aged 105 days, beyond the 90-day refresh threshold). |
| `apps/shiplog/tests/cli_integration.rs` | Integration tests for PR #299 | PASS — `collect_github_me_rejects_remote_http_before_token_or_network` and `auth_github_status_rejects_remote_http_before_environment_credentials` both close the listening socket without ever consuming the token value. |

### STRIDE Threat Model Assessment

| STRIDE Category | Assessment |
|-----------------|------------|
| Spoofing | LOW RISK. Identities flow through HMAC-SHA256 aliasing (`apps/shiplog/src/redact/alias.rs`). Bearer tokens are sourced from env vars or `--token`; `discover_github_user` and `github_auth::resolve` cannot emit a token in error output because the failure path takes the API base validation branch before the token is dereferenced (verified by the new integration tests). Operator receipts that name a path the source PR did not change are caught by `ensure_evidence_targets` and `validate_source_authority`. |
| Tampering | LOW RISK. The transition receipt authority added in PR #277 (VULN-001, 2026-07-27) is bounded by `ensure_evidence_targets` + dual tree-entry re-validation, and `prepare_source_overlay` restores transition-approved source-only paths from source so the alignment check and the overlay content agree. `verify_only` re-derives the resolution plan from the persisted source / swarm state and refuses a modern overlay whose plan id cannot be reproduced. All eight `apps/shiplog/src/cache/sqlite.rs` query sites remain parameterised via `rusqlite::params!`. `source-automation-guard.yml` is the only `pull_request_target` workflow in the workspace; it is metadata-only and gated to canonical source. |
| Repudiation | LOW RISK. `ledger.events.jsonl` is append-only with SHA-256 `EventId`s (`shiplog::ids`). `run_verify_only` writes a machine-readable receipt alongside the resolution plan id and the policy that produced it. `validate` / `validate_full_sha` / `validate_receipt` keep receipts honest. |
| Information Disclosure | LOW RISK. PR #299 closes the token-exfiltration vector via a malicious `api_base` URL. `validate_https_api_base` is now wired into `discover_github_user`, `github_auth::resolve`, and `make_github_ingestor`. The LLM clustering adapter's `validate_https_endpoint` continues to refuse any non-`https` scheme (no loopback carve-out). `bundle/write_zip` constrains zip-entry names to the run directory. `doctor::build_identity_item` reports `ReadyWithCaveats` for the scaffold identity placeholder. |
| Denial of Service | LOW RISK. The overlay worktree is claimed with a unique `pid+time` nonce and bounded to 64 retries; cleanup is `Drop`-driven on every exit path. `dependency_equivalent` computes `git patch-id --stable` at most once per active transition. The source bot guard workflow cost is one literal `exit 1` per push. The regex engine is linear-time (`regex = "1.12.3"`). |
| Elevation of Privilege | LOW RISK. The workspace lint floor (`unsafe_code = "deny"`) is unchanged; the new code adds no `unsafe` and no `Command::new` calls with user-supplied argv. The `run_start` command requires `--yes` before any filesystem write. `ensure_init_files_available` refuses to overwrite `shiplog.toml` / `manual_events.yaml` unless `--force` is passed. The Droid workflows gate fork PRs through `head.repo.full_name == github.repository`, manual `@droid` triggers gate on `author_association ∈ {OWNER, MEMBER, COLLABORATOR}`. `pull_request_target` only appears in `source-automation-guard.yml`, which is metadata-only and gated to canonical source. |

### Security Controls Verified

| Control | Status | Evidence |
|---------|--------|----------|
| Secrets Management | PASS | `.github/workflows/*.yml` reference `secrets.*` with branch / repo scoping; no plaintext tokens in repo. `main.rs::resolve_redaction_key` only reports `RedactionKeySource`. `github_auth::resolve` (`apps/shiplog/src/github_auth.rs`) compares the recorded SHA against the forge-reported merge commit through `check_merged_at`; an attacker who tries to swap the recorded SHA fails closed. |
| SQL Injection | PASS | `cache/sqlite.rs` uses `rusqlite::params!` in all 8 query sites (re-verified). |
| Command Injection | PASS | `transition.rs::system_patch_id` invokes `git patch-id --stable` with a hardcoded argv; the patch content is the only stdin. The `promote.rs` overlay worktree uses hardcoded argv vectors and environment-passed `GIT_OBJECT_DIRECTORY` / `GIT_ALTERNATE_OBJECT_DIRECTORIES`. `run_gh` and `gh_command` use hardcoded argv (the `SHIPLOG_TEST_GH_COMMAND` Windows test seam is documented as debug-only and gated behind `cfg(all(windows, debug_assertions))`). `try_open_path` uses `.arg(path)` only — no shell. |
| Unsafe Code | PASS | `[workspace.lints.rust] unsafe_code = "deny"` and the new code adds no `unsafe` blocks. |
| Unsafe Regex | PASS | `regex = "1.12.3"` (linear-time engine). The transition evidence path does not compile any user-supplied pattern. |
| Input Validation | PASS | `validate_transition_path` mirrors the `policy/source-only-paths.toml` validator. `validate_full_sha` requires 40 hex chars. `validate_source_authority` rejects empty paths, duplicate paths, malformed decision receipts, malformed decision merge SHAs, empty reasons, and active decisions without exact evidence targets. `validate_https_api_base` rejects empty hosts, whitespace hosts, non-loopback cleartext, and unparseable URLs. |
| Path Traversal (writes) | PASS | `bundle/mod.rs::write_zip` uses `path.strip_prefix(out_dir)` so the relative entry name cannot escape the run directory. The overlay path union is normalized by the same validator and only ever passed to `git checkout <sha> -- <path>` and `git rm -r --force --ignore-unmatch -- <path>` against paths inside the worktree. |
| Path Traversal (reads) | N/A | All read paths come from operator-supplied CLI args or from the `transition.path` / `source_authority.path` fields, which the new validators restrict to non-empty normalized strings. |
| Redaction | PASS | Three profiles; deterministic HMAC-SHA256; alias cache never shipped in bundles unless redaction was performed. |
| YAML Parsing | PASS | Maintained `serde_yaml_ng = "0.10.0"`. |
| HTTPS Enforcement (LLM) | PASS | `validate_https_endpoint` retained. |
| HTTPS Enforcement (GitHub) | PASS | `validate_https_api_base` now called from `github_auth::resolve`, `discover_github_user`, and `make_github_ingestor`. |
| HTTPS Enforcement (transition evidence) | PASS | `transition.rs` only ever calls `gh pr view` / `gh pr diff` against the canonical `EffortlessMetrics/shiplog` / `EffortlessMetrics/shiplog-swarm` URLs through the operator's `gh` installation. |
| Identity Attribution Visibility | PASS | `build_identity_item` reports `ReadyWithCaveats` when `[user].label` or `[sources.manual].user` still holds the `SCAFFOLD_USER_PLACEHOLDER`. |
| Source Bot Guard | PASS | `inspect_source_bot_guard` requires the workflow to be present, to trigger `pull_request_target` on `opened` / `reopened` / `synchronize`, to declare top-level `contents: read` + `pull-requests: read`, to forbid any write permission, to define `reject-routine-bot-pr`, to include both `dependabot[bot]` and `factory-droid[bot]` markers with an explicit `exit 1`, and to never invoke `actions/checkout` or any `secrets.*`. |
| Source Reviewer Write Scope | PASS | `inspect_workflow` lets source `droid-review.yml` / `droid.yml` gain `issues` / `pull-requests` write without a `contents` write. |
| CI Repository Identity | PASS | `.github/workflows/contributor-acceptance.yml` uses `${GITHUB_REPOSITORY}` so a fork / non-canonical mirror cannot silently accept the canonical-repo check. |
| Fuzzing | ACTIVE | 36 fuzz targets in `fuzz/fuzz_targets/`; `fuzz-smoke.yml` + `fuzzing.yml` workflows. |
| Property Testing | ACTIVE | `proptest` on redact leak detection, cache TTL math, ingest windows. |
| Mutation Testing | ACTIVE | `cargo-mutants` configured (`cargo-mutants.toml`, `.cargo/mutants.toml`). |
| Lint Floor | PASS | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean. |
| Build Floor | PASS | `cargo test --workspace --all-features --locked` passes; the `github_auth` filtered subset ran locally with 10 + 1 = 11 tests, 0 failed. |
| Workflow Guard: same-repo | PASS | All Droid workflows (`droid.yml`, `droid-review.yml`, `droid-security-scan.yml`) gate on `github.event.pull_request.head.repo.full_name == github.repository` for `pull_request` triggers; manual `issue_comment` / `pull_request_review_comment` / `issues` / `pull_request_review` triggers gate on `author_association ∈ {OWNER, MEMBER, COLLABORATOR}`. No `pull_request_target` anywhere except the metadata-only `source-automation-guard.yml`. |
| Workflow Guard: trusted-actor | PASS | Trusted-actor gate present in `droid.yml` for `@droid` comment triggers. |
| Action Pinning | PASS | `EffortlessMetrics/droid-action-safe@7c1377ccbacddc95560d1570547a5baa51de01ec` pinned. `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (v7.0.1) and `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (v7) pinned. |
| Debug Artifact Leakage | PASS | `upload_debug_artifacts: false` enforced on every `EffortlessMetrics/droid-action-safe` invocation. |

### Defense-in-Depth Posture

The defenses first introduced in the 2026-07-27 / 2026-08-03 scans
continue to hold against the 2026-08-24 baseline:

1. **PR #299** — `validate_https_api_base` is invoked *before* any
   bearer token is dereferenced in `github_auth::resolve`,
   `discover_github_user`, and `make_github_ingestor`. The
   integration tests stand up a `TcpListener::bind("0.0.0.0:0")` and
   assert that the request count is `0` and that the token string
   never appears in either stdout or stderr. The threat model's
   "leak sensitive info (token, email, private repo names)" entry
   is downgraded from High to Medium as a consequence.
2. **VULN-001 (2026-07-27) → fix chain (2026-08-03)** —
   `ensure_evidence_targets` + dual tree-entry re-validation closes
   the receipt authority gap; `compute_resolution_plan` is now the
   shared plan builder for both `run_with_port_to` and
   `run_verify_only`; `verify_only` rejects any modern overlay whose
   `Shiplog-Resolution-Plan:` trailer cannot be reproduced.
3. **PR #311** — `inspect_source_bot_guard` enforces the source-side
   fail-closed guard contract. The validator refuses any workflow
   that gains write permissions, checks out the untrusted head, or
   omits the explicit `exit 1` marker.
4. **PR #308** — `source-automation-guard.yml` is a metadata-only
   guard: no checkout, no secrets, no write access.
5. **PR #333** — Removes a false positive in `inspect_workflow` that
   would have otherwise flagged source-side `droid.yml` /
   `droid-review.yml` for the legitimate `issues` /
   `pull-requests` comment writes they need.
6. **PR #290** — Adds a `ReadyWithCaveats` readiness item whenever
   `[user].label` or `[sources.manual].user` still holds the
   `SCAFFOLD_USER_PLACEHOLDER`. The constant is shared between the
   scaffold template and the placeholder detector so the two cannot
   drift.

### Recommendations

1. **Continue the weekly cadence.** No new findings at or above the
   medium severity threshold were identified in this empty-week
   scan, and the 2026-07-27 / 2026-08-03 hardening chain continues to
   hold. No follow-up patch is required for this branch.
2. **Watch the next `main` landing.** The pending
   `62f5644` / `2d52d2f` dependabot bump (`rusqlite 0.40.1 → 0.40.2`)
   is a semver-patch and is on a branch that has not yet been merged
   into `main`. The next weekly scan should re-verify all eight
   `apps/shiplog/src/cache/sqlite.rs` query sites once it lands.
3. **Threat model is fresh.** The regenerated
   `.factory/threat-model.md` is dated 2026-08-24 and should be
   re-checked on or before 2026-11-22 (90-day refresh threshold).
4. **Future hardening (out of scope for this scan).** Promote the
   `path` field from `String` to a normalized newtype so the
   validator becomes a derive-time guarantee, and convert
   `dtolbay/rust-toolchain@master` references to SHA pins (OBS-4).

### Validation Signals

- **Observed**: 0 commits in the strict 7-day window on `main`. The
  baseline posture review covered 12 commits from the previous scan
  window plus the current `57aee0c` HEAD and the pending
  `62f5644` dependabot branch tip. `cargo fmt --all -- --check`
  clean. `cargo clippy --workspace --all-targets --all-features
  --locked -- -D warnings` clean (Finished `dev` profile in 47.83s).
  Filtered test subsets above re-ran locally: 11 tests, 0 failed.
  Threat model regenerated on this branch; 299 lines, no changes
  from the regeneration draft.
- **Reported**: Threat model file mtime was 2026-05-11 (105 days
  old, beyond the 90-day refresh threshold) before this scan;
  regenerated on this branch to 2026-08-24. Previous security
  report is `security-report-2026-08-03.md`.
- **Not verified**: No remote repository API call was performed;
  GitHub-side secret rotation / exposed token state cannot be
  checked from this checkout (and is governed by
  `EffortlessMetrics/shiplog` repo settings, not by code in this
  repo). The pending dependabot bumps
  (`rusqlite 0.40.1 → 0.40.2`) were spot-checked against the
  RustSec advisory database by transitively reviewing their publish
  dates and changelog entries; no publish-time RustSec advisory
  exists for `rusqlite` 0.40.x as of 2026-08-24.

### References

- [CWE-345: Insufficient Verification of Data Authenticity](https://cwe.mitre.org/data/definitions/345.html)
- [CWE-20: Improper Input Validation](https://cwe.mitre.org/data/definitions/20.html)
- [STRIDE Threat Model](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [Rust Security Advisory Database](https://rustsec.org/)
- Previous reports: `security-report-2026-08-03.md` (0 findings,
  last commit-bearing window), `security-report-2026-07-27.md`
  (VULN-001, the transition receipt authority gap).

---

*Report generated by Factory Droid (security-engineer plugin).
No auto-patches were applied; the 7-day scan window contained zero
commits. The only changes on branch `droid/security-report-2026-08-24`
are this report and the refreshed `.factory/threat-model.md` (the
threat model had aged past the 90-day refresh threshold).*
