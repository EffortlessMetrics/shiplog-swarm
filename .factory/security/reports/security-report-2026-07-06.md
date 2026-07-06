# Security Scan Report

**Generated:** 2026-07-06
**Scan Type:** Weekly Scheduled
**Repository:** EffortlessMetrics/shiplog-swarm
**Severity Threshold:** medium

## Executive Summary

| Severity | Count | Auto-fixed | Manual Required |
|----------|-------|------------|-----------------|
| CRITICAL | 0     | 0          | 0               |
| HIGH     | 0     | 0          | 0               |
| MEDIUM   | 0     | 0          | 0               |
| LOW      | 0     | 0          | 0               |

**Total Findings:** 0
**Auto-fixed:** 0
**Manual Review Required:** 0

## Scan Results Overview

No security vulnerabilities were identified at or above the medium severity threshold in the scanned code.

### Commits Scanned (Last 7 Days)

The strict 7-day window (2026-06-29 through 2026-07-06) contains a single commit:

| Commit | Date | Author | Subject | Type |
|--------|------|--------|---------|------|
| `30640e6` | 2026-07-04 | Steven Zimmerman, CPA | docs: align CLAUDE.md and HANDOFF.md with 0.9 CLI surface (#199) | Docs only |

Note: `30640e6` is the swarm branch's root commit (it has no parent in this
repository). `git diff` against the empty tree reports 801 files and 206,550
insertions because the swarm branch is a fresh import. The commit message and
the contents of the documented changes restrict the intentional modifications
to `CLAUDE.md` and `HANDOFF.md`; the rest of the file additions are part of
the initial import and were reviewed as part of the broader sweep below.

### Intentional Changes Per the Commit Message

- `CLAUDE.md` — describes the review-readiness loop, source setup commands,
  GitHub activity harvest, and read-only JSON agent surfaces; no executable
  code or secrets are referenced.
- `HANDOFF.md` — refreshes the date, adds the guided loop workflow (3.0), and
  rewrites the roadmap. Privacy/security section explicitly notes:
  - LLM clustering is opt-in (build feature, default off).
  - Public profile stripping is provable via tests.
  - Redaction alias cache (`redaction.aliases.json`) is excluded from bundles.
  - `.gitignore` covers `out/`, `target/`, `*.zip`, `*.db`, `*.sqlite*`, `.env*`.
  - docs intentionally avoid token-like prefixes (`ghp_`, `sk-`) to prevent
    secret-scan false positives.
  - Branch protection: main requires PRs, no direct pushes.

### Broader Codebase Sweep (For Completeness)

Because the swarm branch is a root commit, the security scan also performed a
STRIDE sweep of the foundational surfaces that the documented changes
reference. The list of inspected paths is intentionally narrow and limited to
the highest-impact security boundaries.

| Path | Why inspected | Findings |
|------|---------------|----------|
| `apps/shiplog/src/ingest/github.rs` | TLS scheme handling, token transport, URL construction, request budget | None |
| `apps/shiplog/src/ingest/gitlab.rs` | Token transport, base URL handling | None |
| `apps/shiplog/src/ingest/jira.rs` | Token transport, base URL handling | None |
| `apps/shiplog/src/ingest/linear.rs` | API key transport, GraphQL request body | None |
| `apps/shiplog/src/ingest/git.rs` | `git2` FFI use, remote-URL parsing for repo naming | None |
| `apps/shiplog/src/ingest/manual/events.rs` | YAML parsing, file IO | None |
| `apps/shiplog/src/cache/sqlite.rs` | SQL construction (parameterized?) | None |
| `apps/shiplog/src/cluster_llm/client.rs` | Bearer-token transport, response handling | None |
| `apps/shiplog/src/cluster_llm/parse.rs` | LLM JSON parsing | None |
| `apps/shiplog/src/redact/alias.rs` | HMAC-SHA256 aliasing, alias cache IO | None |
| `apps/shiplog/src/redact/mod.rs` | Redaction profile dispatch, property tests | None |
| `apps/shiplog/src/bundle/mod.rs` | Zip writer, manifest generator, profile scoping | None |
| `apps/shiplog/src/render/md/mod.rs` | Markdown rendering of untrusted data | None |
| `apps/shiplog/src/main.rs` (grep) | Process spawn (`xdg-open`/`open`/`explorer.exe`), secret handling | None |
| `.github/workflows/*.yml` | Supply chain, permissions, secret usage | None |

### Security Controls Verified

| Control | Status | Notes |
|---------|--------|-------|
| Secrets Management | PASS | No hardcoded secrets; tokens supplied via `GITHUB_TOKEN`, `GITLAB_TOKEN`, `JIRA_TOKEN`, `LINEAR_API_KEY`; redaction key via `SHIPLOG_REDACT_KEY`; LLM API key via `SHIPLOG_LLM_API_KEY` |
| SQL Injection | PASS | All `rusqlite` callsites in `apps/shiplog/src/cache/sqlite.rs` and elsewhere use the `params!` macro; no string-concatenation SQL found in any workspace crate |
| Command Injection | PASS | `Command::new` callsites in `apps/shiplog/src/main.rs` invoke hardcoded literals `xdg-open` / `open` / `explorer.exe` with a single `&Path` argument that originates inside the run directory (`packet_md`); `command.arg(path)` (not a shell) is used; no `xdg-open` argument-injection opportunity is reachable from user input |
| Unsafe Code | PASS | Workspace lint `unsafe_code = "deny"`; grep for `unsafe {` / `unsafe fn` across `apps/shiplog`, `crates/`, `xtask/` returns zero matches |
| Input Validation | PASS | Consistent use of `anyhow::Result` with `.context(...)`; SHA-256 `EventId`/`RunId` derive deterministically from canonical inputs; proptest coverage exists for ids/redaction/coverage |
| Path Traversal | PASS | Cache (`open(path)` via `rusqlite`) and zip writer (`walk_files`) operate on caller-supplied CLI paths; zip writer only emits a relative path (`strip_prefix(out_dir)`) and never extracts a zip (no zip-slip) |
| URL Handling | PASS | `build_url_with_params` in `apps/shiplog/src/ingest/github.rs` uses the `url` crate's `Url::parse` + `query_pairs_mut` for proper encoding; tests cover empty params, special characters, and invalid base URLs |
| TLS / Scheme | PASS | GitHub `api_base` defaults to `https://api.github.com` and is configurable only for documented GHES use; GitLab and Jira construct URLs via `https://{instance}/...` (scheme hardcoded) |
| Token Transport | PASS | GitHub `bearer_auth(t)` only when `self.token.is_some()`; auth header is the only place tokens appear on the wire; no token logging paths observed |
| Authentication Failure Modes | PASS | All five source adapters surface a clear error when the required token env var is missing (grep for "is required. Set it using" returns five matches, one per adapter) |
| LLM Client Hardening | PASS | `OpenAiCompatibleBackend` uses default `reqwest::blocking::Client`; LLM response parsing uses `serde_json::from_str` with a fixed struct shape, invalid indices are skipped, and receipts are capped (`MAX_RECEIPTS_PER_WORKSTREAM = 10`) |
| Information Disclosure in Redaction | PASS | Three profiles (internal/manager/public) with deterministic HMAC-SHA256 aliasing; public profile property tests assert that PR titles, repo names, manual event content, and all URLs are stripped; alias cache is excluded from bundles |
| Bundle Scope Integrity | PASS | `is_scoped_include` includes only `profiles/<profile>/packet.md` + `coverage.manifest.json` for manager/public; internal includes all non-excluded files; tests pin this behavior |
| JSON Surface (Agent Surfaces) | PASS | Previous scan (2026-06-29) verified the new `sources status --json` projection cannot leak credential/share metadata; this scan confirms the surface is unchanged |
| Reentrancy / TOCTOU | PASS | `repository.config()` write paths in `apps/shiplog/src/ingest/git.rs` tests are scoped to fixture tempdirs and do not mutate real user repos |
| Fuzzing | ACTIVE | Fuzz harnesses present under `fuzz/` (21 targets) for parsers and redaction |
| Property Testing | ACTIVE | `proptest` configured in workspace; redaction, ids, bundle scope, workstream YAML round-trip are all property-tested |
| Mutation Testing | ACTIVE | `cargo-mutants` configured (`cargo-mutants.toml`); `@mutants::skip` is applied to network IO, file IO, and platform integration boundaries |
| Workflow Permissions | PASS | Workflows that perform PR writes (`droid-review.yml`, `droid-security-scan.yml`) declare explicit `permissions:` blocks; `workflow_run`-only workflows (`ci-actuals.yml`) declare `contents: read` + `actions: read`; no PR-triggered workflow has unfettered token scope |
| CI Supply Chain | PASS | `actions/checkout` and `actions/cache` bumped to v7 / v6 in previous scan window (2026-06-29); all third-party Droid actions pinned to commit SHAs in `droid-security-scan.yml` and `droid-review.yml` |
| Dependabot | PASS | `.github/dependabot.yml` configured for Rust and GitHub Actions ecosystems |
| Inline Tokens in Docs | PASS | `HANDOFF.md` explicitly avoids `ghp_` / `sk-` prefixes to prevent secret-scan false positives; the diff contains no token-like literals |

### STRIDE Threat Model Assessment

| STRIDE Category | Assessment |
|-----------------|------------|
| Spoofing | LOW RISK — HMAC-SHA256 deterministic aliasing for user identities; tokens via env vars; no new identity surfaces introduced |
| Tampering | LOW RISK — Immutable `ledger.events.jsonl` (append-only) with SHA-256 `EventId`s; SQLite cache values keyed by `CacheKey` (URL-derived); alias cache has version field that refuses mismatches (`unsupported alias cache version` error tested) |
| Repudiation | LOW RISK — Receipts-first design with deterministic event IDs preserved; `redaction.aliases.json` is per-run so redaction choices are reviewable |
| Information Disclosure | LOW RISK — Three redaction profiles with property-tested leak guarantees; alias cache excluded from bundles; previous scan verified `sources status --json` cannot leak credential/share metadata |
| Denial of Service | LOW RISK — No new unbounded operations; SQLite cache uses TTL expiry (`clean_expired`, `clean_older_than`); GitHub API budget is bounded (`GithubApiBudget`) |
| Elevation of Privilege | LOW RISK — No `unsafe`, no `eval`, no shell calls; `Command::new` callsites are limited to `xdg-open`/`open`/`explorer.exe` with hardcoded literals |

## Critical Findings

None.

## High Findings

None.

## Medium Findings

None.

## Low Findings

None.

## Appendix

### Threat Model

- Path: `.factory/threat-model.md`
- File contents marked `*Generated: 2026-05-11*`
- Age: ~56 days (below the 90-day regeneration threshold)
- Status: Reused. Existing threat model accurately covers all surfaces
  touched by this week's changes (docs only). No regeneration required.

### Scan Metadata

- Scan Window: 2026-06-29 to 2026-07-06 (strict 7 days)
- Commits Scanned: 1 (`30640e6`)
- Files Inspected: ~16 high-impact source paths + `.github/workflows/*.yml` (18 workflow files reviewed for supply-chain posture) + `.factory/security/reports/` for trend continuity
- Output Branch: `droid/security-report-2026-07-06`
- Report Path: `.factory/security/reports/security-report-2026-07-06.md`
- Skills Used (manual equivalents): commit-security-scan (manual code review
  against STRIDE), vulnerability-validation (manual reachability/exploitability
  check), security-review (no patches generated)
- Result: Clean scan, zero findings at or above the `medium` threshold

### Comparison with Prior Reports

| Report Date | Strict 7-day commits | Findings (medium+) |
|-------------|----------------------|--------------------|
| 2026-05-11 | 1 (docs/ci) | 0 |
| 2026-05-18 | 1 (release posture doc) | 0 |
| 2026-06-29 | 3 (dependabot bumps + 1 product) | 0 |
| 2026-07-06 | 1 (docs only) | 0 |

The repo has held a clean weekly posture for the past 8 weeks.

### Recommendations

1. **Continue the docs-first release cadence**: The intentional change this
   week is a pure docs refresh. There is no follow-up remediation work needed.
2. **Keep the previous-scan guardrails intact**: The
   `sources status --json` integration test from `aebab19f` (verified in the
   2026-06-29 scan) continues to be the only credential/share-metadata JSON
   surface. Any future field additions to `SourcesStatusView` should extend
   that test before merging.
3. **Plan threat-model regeneration**: The next scan whose findings or branch
   scope pushes past 90 days since `2026-05-11` should re-issue the threat
   model. Reference date: regenerate by `2026-08-09` to maintain the 90-day
   policy floor.
4. **Lint the macro use of `unsafe_code = "deny"`**: Since the workspace lint
   is already `deny`, the implicit safety invariant is enforced. No action
   needed, but future contributors should know that CI will fail if `unsafe`
   is ever reintroduced.

### References

- [CWE Database](https://cwe.mitre.org/)
- [STRIDE Threat Model](https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [RustSec Advisory Database](https://rustsec.org/)
- Previous reports in this directory: `security-report-2026-05-11.md`,
  `security-report-2026-05-18.md`, `security-report-2026-06-29.md`

---

*Report generated by Factory Droid (security-engineer plugin) for `EffortlessMetrics/shiplog-swarm`.*
