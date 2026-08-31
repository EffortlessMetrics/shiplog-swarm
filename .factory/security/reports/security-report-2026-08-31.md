# Security Scan Report

**Generated:** 2026-08-31
**Scan Type:** Weekly Scheduled
**Repository:** EffortlessMetrics/shiplog-swarm
**Branch:** `droid/security-report-2026-08-31`
**Severity Threshold:** medium
**Scan Window:** 2026-08-24 to 2026-08-31 (7 days)

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

No security vulnerabilities were identified at or above the medium severity
threshold in the one commit scanned this week. The scan window is dominated
by a single defensive hardening (PR #415, `f134583`) that closes the
regression surface previously identified as the mirror of `RELEASE_HANDOFF_0.12.0.md`'s
audit item: the published-binary smoke scripts and the fresh-clone
contributor-acceptance workflow both asserted they ran "without provider
credentials", but each only cleared six environment variables while
shiplog now reads nine. An exported `GH_ENTERPRISE_TOKEN`,
`GITHUB_ENTERPRISE_TOKEN`, or `SHIPLOG_LLM_API_KEY` stayed visible to the
binary on exactly the machines most likely to have one set, so the
no-token path stopped being proven on those machines.

PR #415 makes `AMBIENT_CREDENTIAL_ENV_VARS` (in
`crates/shiplog-testkit/src/env.rs`) the single source of truth for
which env vars shiplog reads credentials from. The two smoke scripts and
the contributor-acceptance workflow each declare that list once and clear
through it, and `credential_free_lanes_clear_every_ambient_credential`
pins every declaration in both directions: nothing missing, and nothing
stale pretending to be coverage.

This is a hardening of the credential-free-lane contract documented in
the threat model under "Elevation of Privilege" / "no eval, no unsafe
code, sandboxed adapters", not a newly identified vulnerability. No
follow-up patch is required for this branch.

## Critical Findings

None.

## High Findings

None.

## Medium Findings

None.

## Low Findings

None at or above the medium severity threshold.

### Observations Below Threshold

| ID | Class | File | Note |
|----|-------|------|------|
| OBS-1 | Loopback HTTP exception | `apps/shiplog/src/ingest/github.rs:1510` | Carried forward. `validate_https_api_base` intentionally permits HTTP for genuine loopback addresses (match-arm checks `url::Host::Ipv4`/`Ipv6` against `IpAddr::is_loopback()` and treats `localhost` case-insensitively). The exception cannot be reached by a remote attacker because `Url::parse` preserves the literal host string before DNS resolution, so a misleading `Host` header cannot redirect the bind. PR #415 did not modify this carve-out. |
| OBS-2 | User-controlled regex | `apps/shiplog/src/main.rs` (workstreams split) | Carried forward. `RegexBuilder::new(pattern)` for `workstreams split --matching`. Self-DoS only; not exploitable by an external attacker because the pattern is supplied by the local operator over the CLI. |
| OBS-3 | Markdown link escaping | `apps/shiplog/src/render/md/receipt.rs` | Carried forward. URLs from API responses or `manual_events.yaml` are interpolated unescaped into `[label](url)`. Output is a file (`packet.md`) opened locally; impact is limited to local renderer behavior. |
| OBS-4 | `dtolbay/rust-toolchain@master` | `.github/workflows/*.yml` (12 occurrences) | Carried forward. Mutable ref rather than a SHA pin. Mitigated by `rust-toolchain.toml` pinning the actual Rust version and the action's wide audit. PR #415 did not add any new `dtolbay/rust-toolchain` occurrences. |
| OBS-5 | `bundle/mod.rs::walk_files` follows symlinks | `apps/shiplog/src/bundle/mod.rs:162` | Carried forward. The bundle is only ever produced from output directories the same operator owns. |
| OBS-6 | `transition.rs::lock_transitions` parses TOML per line | `xtask/src/tasks/transition.rs:469` | Carried forward. The line-oriented parser calls `line.parse::<toml::Table>()` per `+`/`-` line. The parsed value is thrown away after extracting the literal `name`/`version` string. Not a security issue, but allocates on every `Cargo.lock` diff. |
| OBS-7 | `discover_github_user` body / error leakage | `apps/shiplog/src/main.rs:11965` | Carried forward. `validate_https_api_base` is wired into this function and the `auth_github_status_rejects_remote_http_before_environment_credentials` integration test (`apps/shiplog/tests/cli_integration.rs:18497`) explicitly asserts `!stderr.contains(token)` and `!stdout.contains(token)`. The token is sent only after the API base is verified, so the failure path can never contain the bearer token. Verified. |
| OBS-8 | `cargo xtask promote` overlay worktree ownership | `xtask/src/tasks/promote.rs` (`prepare_source_overlay`) | Carried forward. The overlay worktree is claimed with a `pid+time` nonce and bounded to 64 retries; cleanup is `Drop`-driven on every exit path. Not a privilege issue; the worktree is owned by the invoking operator. |
| OBS-9 | `validate_https_api_base` loopback host comparison | `apps/shiplog/src/ingest/github.rs:1510` | Carried forward. Re-verified: the match-arm is `url::Host::Ipv4(ip)` / `url::Host::Ipv6(ip)` against `IpAddr::is_loopback()`, with `host_str.eq_ignore_ascii_case("localhost")` as the literal-name carve-out. A DNS rebinding attacker cannot reach this branch because `Url::parse` preserves the literal host string before any DNS resolution. |
| OBS-10 | `AMBIENT_CREDENTIAL_ENV_VARS` testkit coverage test (PR #415) | `crates/shiplog-testkit/src/env.rs:33` (`covers_both_github_token_variables`), `apps/shiplog/tests/release_candidate_smoke.rs` (`credential_free_lanes_clear_every_ambient_credential`) | New this week. The testkit pins both `GH_TOKEN` and `GITHUB_TOKEN` in the canonical list. The release-candidate smoke test pins every declaration in both directions: every name in the canonical list must appear in each declared list, and every name in a declared list must appear in the canonical list. This is hardening, not a vulnerability. |
| OBS-11 | `gh_command` Windows debug seam | `apps/shiplog/src/github_auth.rs:404` | Carried forward. `SHIPLOG_TEST_GH_COMMAND` is gated behind `cfg(all(windows, debug_assertions))` so release builds always resolve the real GitHub CLI from PATH. The arg vector passed to `cmd.exe /D /S /C` is hardcoded by the test seam; the user-supplied argv comes from the function caller (`&[&str]`), not from environment input. |
| OBS-12 | `release.yml` `dtolbay/rust-toolchain@master` | `.github/workflows/release.yml:109,164,437,695` | Carried forward. PR #415 did not modify `release.yml`, but the mutable ref remains. Mitigated by `rust-toolchain.toml` pinning the actual toolchain. |

## Appendix

### Threat Model

- Version: 2026-05-11 (unchanged this scan)
- Location: `.factory/threat-model.md`
- Status: **Refresh needed** (aged 112 days, over the 90-day refresh threshold)
- Action taken: re-used as scan context; no regeneration performed in this scan.
- The Information Disclosure line should be downgraded from High to Medium on
  the next refresh — the token-exfiltration vector that justified High was
  closed by PR #299's `validate_https_api_base` wiring and reinforced by
  PR #415's exhaustive credential clearing on every credential-free lane.
- The new `AMBIENT_CREDENTIAL_ENV_VARS` contract should be added to the
  Elevation of Privilege section's "sandboxed adapters" notes.

### Scan Metadata

- Commits Scanned: 1 (strict 7-day window — `f134583`)
- Branch: `droid/security-report-2026-08-31`
- Scan Duration: ~20m (run on 2026-08-31 from
  `git log --since="7 days ago"` and `git cat-file --batch-all-objects`)
- Skills / Tools Used: `commit-security-scan` (manual STRIDE walk via
  `Grep` + `Read` + targeted diffs), `vulnerability-validation` (manual
  reachability / exploitability review), `security-review` (manual
  verification of `clear_ambient_credentials`, `AMBIENT_CREDENTIAL_ENV_VARS`,
  `credential_free_lanes_clear_every_ambient_credential`, `validate_https_api_base`,
  `validate_https_endpoint`, and the SQLite cache query sites).
- Validation Locally Executed:
  - `cargo fmt --all -- --check` — clean.
  - `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` — clean.
  - `cargo test --release -p shiplog --locked --test release_candidate_smoke` — 10 passed, 0 failed.
    This includes the new `credential_free_lanes_clear_every_ambient_credential`
    test which exercises every declaration in both directions across
    `scripts/release-install-smoke.sh`, `scripts/release-install-smoke.ps1`,
    and `.github/workflows/contributor-acceptance.yml`.

### Commits Scanned (strict 7-day window)

| SHA | Date (UTC) | Subject | Security-Relevant? |
|------|------------|---------|---------------------|
| `f134583` | 2026-08-27 | `fix(acceptance): clear every ambient credential in credential-free lanes (#415)` | Yes (hardening) |

### Surfaces Reviewed

| Surface | Purpose | Result |
|---------|---------|--------|
| `crates/shiplog-testkit/src/env.rs` | Canonical ambient-credential env-var list | PASS — `AMBIENT_CREDENTIAL_ENV_VARS` now lists all 9 names. `clear_ambient_credentials` removes every entry. New unit tests `covers_both_github_token_variables` and `clearing_beats_an_exported_credential_and_yields_to_an_explicit_one` pass. |
| `scripts/release-install-smoke.sh` | Unix published-binary smoke | PASS — declares `ambient_credential_env_vars=(...)` with all 9 names and clears through it before every credential-free proof step. |
| `scripts/release-install-smoke.ps1` | Windows published-binary smoke | PASS — declares `$AmbientCredentialEnvVars = @(...)` with all 9 names and clears through it before every credential-free proof step. |
| `.github/workflows/contributor-acceptance.yml` | Fresh-clone contributor CI | PASS — Unix and Windows lanes each declare their list with all 9 names and assert no credential variable survives. |
| `apps/shiplog/tests/release_candidate_smoke.rs` (`credential_free_lanes_clear_every_ambient_credential`) | Bidirectional pin between canonical and declared lists | PASS — verifies both directions for each declared list, across all 4 declaration sites (Unix+Windows in both `.sh`/`.ps1` and the workflow). |
| `apps/shiplog/src/github_auth.rs::resolve` | GitHub auth resolution (env / `gh`) | PASS — `validate_https_api_base` is invoked before any credential lookup; `safe_metadata_does_not_serialize_credential_material` test guards against token leakage in metadata. |
| `apps/shiplog/src/ingest/github.rs::validate_https_api_base` | HTTPS-only `api_base` validator | PASS — loopback carve-out unchanged from 2026-07-27. |
| `apps/shiplog/src/cluster_llm/client.rs::validate_https_endpoint` | HTTPS-only LLM endpoint validator | PASS — strict, no loopback. |
| `apps/shiplog/src/cache/sqlite.rs` | SQLite cache | PASS — all query sites remain parameterized via `rusqlite::params!`. |
| `.github/workflows/source-automation-guard.yml` | Source bot guard | PASS — `pull_request_target` only, `permissions: contents: read / pull-requests: read`, no checkout, no `secrets.*`, hard fail (`exit 1`) on either routine bot identity. |
| `.github/workflows/droid*.yml` | Droid automation | PASS — same-repo guard, trusted-actor guard, no `pull_request_target` outside `source-automation-guard.yml`, action SHA pinning. |
| `.github/workflows/release.yml` | Release pipeline | PASS — exact-tag checkout, four-platform staging, manifest-bound `RELEASE_CANDIDATE.txt`, `release-candidate-ready` terminal gate. |
| `.github/workflows/contributor-acceptance.yml` | Fresh-clone contributor CI | PASS — PR #415 keeps credential clearing exhaustive; no ambient credential survives into `cargo build` or `dev-check`. |
| `.github/workflows/security.yml` | cargo-deny | PASS — scope is `pull_request: labeled/synchronize/reopened` plus push/cron/dispatch; does not duplicate PR-on-every (PR #155 routing). |
| `policy/workflow-allowlist.toml` | Workflow ledger | PASS — all 21 workflow files are accounted for; `external_actions` matches each workflow's actual `uses:` lines. |
| `policy/network-allowlist.toml` | Network allowlist | PASS — outbound destinations from CI/release lanes are receipted (crates.io, rustup, github releases). |
| `policy/automation-authority.toml` | Automation authority contract | PASS — swarm is `repository_role = "swarm"`, source retains `release-execution = "explicitly-authorized"`. |
| `Cargo.lock` | Dependency manifest | PASS — `cargo clippy --workspace --all-features --locked` clean; no `unsafe_code` (denied at workspace lint floor); no hardcoded secrets. |
| `.factory/threat-model.md` | Living threat model | Aged 112 days; refresh recommended at next scan. |

### STRIDE Threat Model Assessment

| STRIDE Category | Assessment |
|-----------------|------------|
| Spoofing | LOW RISK. Identities flow through HMAC-SHA256 aliasing (`apps/shiplog/src/redact/alias.rs`). Bearer tokens are sourced from env vars or `--token`; the `resolve` path in `apps/shiplog/src/github_auth.rs` validates the API base before dereferencing any credential, so a malicious `api_base` URL cannot capture a token. PR #415 reduces the residual exposure surface by ensuring no env-var credential survives into a "credential-free" lane. |
| Tampering | LOW RISK. `ledger.events.jsonl` is append-only with SHA-256 `EventId`s (`shiplog::ids`). The SQLite cache continues to use parameterized queries in all 8 query sites. PR #415 does not modify any cache or ingest surface. |
| Repudiation | LOW RISK. `ledger.events.jsonl` is append-only; `release-candidate-ready.txt` and `RELEASE_CANDIDATE.txt` are workflow-staged, manifest-bound, and `release-preflight` re-checks them on every platform. |
| Information Disclosure | LOW RISK. PR #299 closed the token-exfiltration vector via a malicious `api_base` URL (`validate_https_api_base` wired into `github_auth::resolve`, `discover_github_user`, and `make_github_ingestor`). PR #415 closes the residual credential-leak vector on the published-binary and fresh-clone credential-free lanes by exhaustively clearing every env-var shiplog reads. The LLM clustering adapter's `validate_https_endpoint` continues to refuse any non-`https` scheme. |
| Denial of Service | LOW RISK. The SQLite cache has TTL-based expiry and `cleanup_expired` / `cleanup_older_than` helpers. The `AMBIENT_CREDENTIAL_ENV_VARS` constant is a 9-element `&[&str]` and is iterated only at test/process startup; not an attack surface. |
| Elevation of Privilege | LOW RISK. The workspace lint floor (`unsafe_code = "deny"`) is unchanged; PR #415 adds no `unsafe` and no `Command::new` calls with user-supplied argv. The credential-free lanes only read the env vars they explicitly enumerate and remove. |

### Security Controls Verified

| Control | Status | Evidence |
|---------|--------|----------|
| Secrets Management | PASS | `.github/workflows/*.yml` reference `secrets.*` with branch / repo scoping; no plaintext tokens in repo. PR #415 strengthens the credential-free contract by exhaustive enumeration of `AMBIENT_CREDENTIAL_ENV_VARS`. |
| SQL Injection | PASS | `cache/sqlite.rs` uses `rusqlite::params!` in all 8 query sites (re-verified). |
| Command Injection | PASS | `github_auth::gh_command` invokes `gh` with a hardcoded argv vector and the Windows debug seam is gated behind `cfg(all(windows, debug_assertions))`. |
| Unsafe Code | PASS | `[workspace.lints.rust] unsafe_code = "deny"` and PR #415 adds no `unsafe` blocks. |
| Unsafe Regex | PASS | `regex = "1.12.3"` (linear-time engine). The credential-free lane fix does not compile any user-supplied pattern. |
| Input Validation | PASS | `validate_https_api_base` runs before any credential dereference. `validate_full_sha`, `validate_source_authority`, and the new bidirectional `AMBIENT_CREDENTIAL_ENV_VARS` pin keep the credential-free contract closed. |
| Path Traversal (writes) | PASS | `bundle/mod.rs::write_zip` uses `path.strip_prefix(out_dir)` so the relative entry name cannot escape the run directory. PR #415 does not modify any write path. |
| Redaction | PASS | Three profiles; deterministic HMAC-SHA256; alias cache never shipped in bundles. |
| YAML Parsing | PASS | Maintained `serde_yaml_ng = "0.10.0"`. |
| HTTPS Enforcement (LLM) | PASS | `validate_https_endpoint` retained. |
| HTTPS Enforcement (GitHub) | PASS | `validate_https_api_base` retained. |
| Credential-Free Lane Exhaustiveness | PASS (new this week) | `credential_free_lanes_clear_every_ambient_credential` pins every declaration in both directions against `AMBIENT_CREDENTIAL_ENV_VARS`. |
| Identity Attribution Visibility | PASS | `build_identity_item` reports `ReadyWithCaveats` when `[user].label` or `[sources.manual].user` still holds the `SCAFFOLD_USER_PLACEHOLDER`. |
| Source Bot Guard | PASS | `source-automation-guard.yml` requires `pull_request_target` with `opened`/`reopened`/`synchronize`, top-level `contents: read` + `pull-requests: read`, no write scopes, `reject-routine-bot-pr` job with both `dependabot[bot]` and `factory-droid[bot]` markers, explicit `exit 1`, no `actions/checkout`, no `secrets.`. |
| CI Repository Identity | PASS | `contributor-acceptance.yml` uses `${GITHUB_REPOSITORY}` so forks and other repos see the correct `git remote get-url origin` expectation. |
| Fuzzing | ACTIVE | 36 fuzz targets in `fuzz/fuzz_targets/`; `fuzz-smoke.yml` + `fuzzing.yml` workflows. |
| Property Testing | ACTIVE | `proptest` on redact leak detection, cache TTL math, ingest windows. |
| Mutation Testing | ACTIVE | `cargo-mutants` configured (`cargo-mutants.toml`, `.cargo/mutants.toml`). |
| Lint Floor | PASS | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` clean. |
| Build Floor | PASS | `cargo test --release -p shiplog --locked --test release_candidate_smoke` passes 10 tests, 0 failures. |
| Workflow Guard: same-repo | PASS | All Droid workflows (`droid.yml`, `droid-review.yml`, `droid-security-scan.yml`) gate on `github.event.pull_request.head.repo.full_name == github.repository` for `pull_request` triggers; manual `issue_comment` / `pull_request_review_comment` / `issues` / `pull_request_review` triggers gate on `author_association ∈ {OWNER, MEMBER, COLLABORATOR}`. No `pull_request_target` anywhere except the metadata-only `source-automation-guard.yml`. |
| Workflow Guard: trusted-actor | PASS | Trusted-actor gate present in `droid.yml` for `@droid` comment triggers. |
| Action Pinning | PASS | `EffortlessMetrics/droid-action-safe@7c1377ccbacddc95560d1570547a5baa51de01ec` pinned. `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1` (v7.0.1), `actions/upload-artifact@043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (v7), `actions/download-artifact@3e5f45b2cfb9172054b4087a40e8e0b5a5461e7c` (v8) pinned. |
| Debug Artifact Leakage | PASS | `upload_debug_artifacts: false` enforced on every `EffortlessMetrics/droid-action-safe` invocation. |

### Defense-in-Depth Highlights This Week

1. **PR #415** — `AMBIENT_CREDENTIAL_ENV_VARS` in
   `crates/shiplog-testkit/src/env.rs` is now the single source of truth
   for which env vars shiplog reads credentials from. The published-binary
   smoke scripts (`release-install-smoke.sh` and `.ps1`) and the
   fresh-clone contributor-acceptance workflow each declare the list once
   and clear through it. `credential_free_lanes_clear_every_ambient_credential`
   pins every declaration in both directions: every canonical name must
   appear in every declared list, and every declared name must appear in
   the canonical list. This closes the residual Information Disclosure
   line in the credential-free contract: a `GH_ENTERPRISE_TOKEN`,
   `GITHUB_ENTERPRISE_TOKEN`, or `SHIPLOG_LLM_API_KEY` exported on the
   fresh-clone contributor acceptance or the published-binary smoke
   could previously have stayed visible to the binary, so the no-token
   proof stopped being true exactly on the machines most likely to have
   one set.
2. The fresh-clone contract test no longer pins the literal prefix
   "unset GITHUB_TOKEN GH_TOKEN GITLAB_TOKEN" — that prefix pinned the
   first three names instead of proving coverage, and is how the list
   fell behind in the first place. It now asserts only that both lanes
   clear through the shared list, and points at the test that owns which
   names that list must hold.

### Recommendations

1. **Continue the weekly cadence.** PR #415 hardens the
   credential-free-lane contract; PR #299's `validate_https_api_base`
   wiring and the VULN-001 / 2026-07-27 transition-receipt authority fix
   both continue to hold across this week's small surface change.
2. **Refresh the threat model.** `.factory/threat-model.md` is now 112
   days old (over the 90-day threshold). The next scan should regenerate
   it; the Information Disclosure line should be downgraded from High to
   Medium (PR #299 + PR #415 close the two highest-impact entries) and
   the `AMBIENT_CREDENTIAL_ENV_VARS` contract should be added to the
   Elevation of Privilege section's "sandboxed adapters" notes.
3. **Carry-forward.** OBS-1..OBS-9 remain below the medium threshold and
   should be re-evaluated in next week's scan. OBS-10 is new this week
   and below threshold. OBS-11 and OBS-12 are carried-forward.

### Validation Signals

- **Observed**: 1 commit in the 7-day window. The commit is
  `f134583` (PR #415) and is a defensive hardening of the
  credential-free-lane contract. `cargo fmt --all -- --check` clean.
  `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`
  clean. `cargo test --release -p shiplog --locked --test release_candidate_smoke`
  passes 10/10 tests, including the new
  `credential_free_lanes_clear_every_ambient_credential` test that pins
  every declaration in both directions across all 4 declaration sites.
- **Reported**: Threat model file mtime is 2026-08-31 (today), but the
  content header still reads `Generated: 2026-05-11`. The
  2026-05-11 generation is the authoritative version; the mtime change
  is from another workflow touching the file. Previous security report
  is `security-report-2026-08-03.md` (0 findings).
- **Not verified**: No remote repository API call was performed;
  GitHub-side secret rotation / exposed token state cannot be
  checked from this checkout (and is governed by `EffortlessMetrics/shiplog`
  repo settings, not by code in this repo). cargo-deny is not
  installed in this runner; the dependency manifest was spot-checked
  via `cargo clippy --workspace --all-features --locked` and a manual
  read of the deny.toml policy, not via `cargo deny check advisories`.

### References

- [CWE-345: Insufficient Verification of Data Authenticity](https://cwe.mitre.org/data/definitions/345.html)
- [CWE-20: Improper Input Validation](https://cwe.mitre.org/data/definitions/20.html)
- [CWE-522: Insufficiently Protected Credentials](https://cwe.mitre.org/data/definitions/522.html)
- [STRIDE Threat Model](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [Rust Security Advisory Database](https://rustsec.org/)
- Previous reports: `security-report-2026-08-03.md` (0 findings),
  `security-report-2026-07-27.md` (VULN-001, the transition receipt
  authority gap that `ensure_evidence_targets` + dual tree-entry
  re-validation closed).

---

*Report generated by Factory Droid (security-engineer plugin). No
auto-patches were applied; the one change in this scan window
(PR #415, `f134583`) is a defensive hardening authored by
`EffortlessSteven` and merged through the swarm's normal review flow.
The report itself is the only change on branch
`droid/security-report-2026-08-31`.*
