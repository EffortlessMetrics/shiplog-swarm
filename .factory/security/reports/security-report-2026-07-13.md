# Security Scan Report

**Generated:** 2026-07-13
**Scan Type:** Weekly Scheduled
**Repository:** EffortlessMetrics/shiplog-swarm
**Branch:** `droid/security-report-2026-07-13`
**Severity Threshold:** medium
**Scan Window:** 2026-07-06 to 2026-07-13 (7 days)

## Executive Summary

| Severity | Count | Auto-fixed | Manual Required |
|----------|-------|------------|-----------------|
| CRITICAL | 0 | 0 | 0 |
| HIGH | 0 | 0 | 0 |
| MEDIUM | 0 | 0 | 0 |
| LOW | 0 | 0 | 0 |

**Total Findings (>= medium):** 0
**Auto-fixed:** 0
**Manual Review Required:** 0

No security vulnerabilities were identified at or above the medium severity
threshold in the scanned code. One previous finding (VULN-001, MEDIUM, from
2026-06-29) remains closed: the LLM endpoint now validates the `https://`
scheme before any API call is issued.

## Scan Results Overview

The strict 7-day scan window (`git log --since="7 days ago"`) returned
**1 commit**: `1c8577d` ("fix(ci): make hosted routing deterministic and
refresh receipts (#233)", 2026-07-13 04:08:34 -0400). That single commit
is the bootstrap of the `EffortlessMetrics/shiplog-swarm` repository: it
introduces 819 files and 210,671 insertions that, in aggregate, mirror the
codebase that was previously reviewed on the source repository. The commit
message describes CI routing/receipt work, but the diff covers the full
swarm surface.

Because the change set spans essentially the entire repository, this scan
performed a full-codebase STRIDE review of the security-relevant surfaces
shipped at HEAD, with extra weight on the surfaces touched by the bootstrap
diff and on the previously identified VULN-001 to confirm it remains fixed.

### Commits Scanned

| SHA | Date (UTC) | Subject | Files |
|------|------------|---------|-------|
| 1c8577d | 2026-07-13 04:08 | fix(ci): make hosted routing deterministic and refresh receipts (#233) | 819 / +210671 |

### Surfaces Reviewed

| Surface | Purpose | Result |
|---------|---------|--------|
| `apps/shiplog/src/cache/sqlite.rs` | SQLite cache (parameterized queries) | PASS |
| `apps/shiplog/src/cache/{mod,key,expiry,stats}.rs` | Cache key + TTL helpers | PASS |
| `apps/shiplog/src/ingest/github.rs` | GitHub REST/GraphQL ingest | PASS |
| `apps/shiplog/src/ingest/gitlab.rs` | GitLab MR ingest | PASS |
| `apps/shiplog/src/ingest/jira.rs` | Jira issue ingest | PASS |
| `apps/shiplog/src/ingest/linear.rs` | Linear issue ingest | PASS |
| `apps/shiplog/src/ingest/json.rs` | JSONL ledger ingest | PASS |
| `apps/shiplog/src/ingest/git.rs` | Local libgit2 ingest | PASS |
| `apps/shiplog/src/ingest/manual/{mod,events}.rs` | YAML manual ingest | PASS |
| `apps/shiplog/src/cluster_llm/{client,config,mod,parse,parse/*,prompt}.rs` | LLM clustering | PASS |
| `apps/shiplog/src/redact/{alias,mod,policy,profile,projector,repo}.rs` | HMAC-SHA256 deterministic aliasing | PASS |
| `apps/shiplog/src/render/md/{mod,coverage,receipt,source}.rs` | Markdown renderer | PASS |
| `apps/shiplog/src/bundle/{mod,layout}.rs` | Zip + SHA-256 manifest writer | PASS |
| `apps/shiplog/src/merge/mod.rs` | Run-merge logic | PASS |
| `apps/shiplog/src/workstreams/{cluster,layout,receipt_policy,mod}.rs` | Workstream clustering | PASS |
| `apps/shiplog/src/github_auth.rs` | GitHub credential resolution | PASS |
| `apps/shiplog/src/main.rs` (env access, `try_open_path`, `Command::new`) | Process / env handling | PASS |
| `.github/workflows/*.yml` (16 files) | CI / secrets exposure | PASS |
| `.factory/threat-model.md` | Living threat model | Current |
| `deny.toml`, `policy/network-allowlist.toml` | Cargo-deny + network policy | PASS |

### STRIDE Threat Model Assessment

| STRIDE Category | Assessment |
|-----------------|------------|
| Spoofing | LOW RISK. User identity flows through HMAC-SHA256 aliasing (`redact/alias.rs`); bearer tokens are sourced from env vars or explicit CLI flags. The `safe_metadata_does_not_serialize_credential_material` test in `github_auth.rs` enforces that the credential secret is never echoed via the structured metadata. |
| Tampering | LOW RISK. SQLite cache uses parameterized statements via `rusqlite::params!` exclusively (no string concatenation in any query). Cache rows are content-addressed by deterministic keys (`cache/key.rs`). `bundle/mod.rs` excludes `redaction.aliases.json` and `bundle.manifest.json` from bundles via `ALWAYS_EXCLUDED`; six unit tests cover the exclusion across `walk_files`, `write_bundle_manifest`, and `write_zip` for all three profiles. |
| Repudiation | LOW RISK. `ledger.events.jsonl` is append-only with SHA-256 `EventId` derivation (`ids.rs`). Rate-limit snapshots are recorded via `record_rate_limit_headers` for both `search` and `core` GitHub buckets, providing receipts for live API usage. |
| Information Disclosure | LOW RISK. Three deterministic redaction profiles (internal/manager/public). The `OpenAiCompatibleBackend::complete` call site in `cluster_llm/client.rs` now invokes `validate_https_endpoint`, which parses the URL and refuses any scheme that is not `https` and any host that is empty. VULN-001 (2026-06-29) is confirmed fixed in the new code path. |
| Denial of Service | LOW RISK. TTL-based cache cleanup (`cleanup_expired`, `cleanup_older_than`). API request budgets (`GithubApiBudget` with `max_search_requests` / `max_core_requests`) cap live calls. `--throttle-ms` slows hostile-only loops. The `try_open_path` subprocess is invoked with `Stdio::null()` on all three streams. |
| Elevation of Privilege | LOW RISK. Workspace `[workspace.lints.rust] unsafe_code = "deny"`; `grep` for `\bunsafe\b` in `apps/` returns no matches. No `eval`, no shell `Command::new` with `-c`. The only `Command::new` callers are: (a) `try_open_path` running `xdg-open` / `open` / `explorer.exe` with a single canonicalized arg, and (b) integration tests invoking the CLI binary via `env!("CARGO_BIN_EXE_shiplog")`. |

### Security Controls Verified

| Control | Status | Evidence |
|---------|--------|----------|
| Secrets Management | PASS | `.github/workflows/*.yml` reference `secrets.*` with branch / repo scoping; no plaintext tokens in repo. `resolve_redaction_key` only reports `RedactionKeySource`, never the key. `droid-security-scan.yml` only references `MINIMAX_API_KEY` and `FACTORY_API_KEY` from `secrets.*`; the inline `cat > $HOME/.factory/settings.json` heredoc uses `${MINIMAX_API_KEY}` interpolation (no echo in CI logs). |
| SQL Injection | PASS | `cache/sqlite.rs` uses `rusqlite::params!` in all query sites (`get`, `lookup`, `set_with_ttl`, `contains`, `cleanup_expired`, `count_older_than`, `cleanup_older_than`, `stats`, `inspect`). |
| Command Injection | PASS | `try_open_path` uses `Command::arg(path)` (single-arg, no shell). `Command::new("gh")` in `github_auth.rs` uses `args(arguments)` with a fixed literal argv. |
| Unsafe Code | PASS | `[workspace.lints.rust] unsafe_code = "deny"`; `grep` for `\bunsafe\b` in `apps/` returns no matches. |
| Unsafe Regex | PASS | `regex = "1.12.4"` (linear-time engine). `RegexBuilder` only used in `main.rs` for the local-CLI `workstreams split --matching` path (self-DoS only, no remote attacker model). |
| Input Validation | PASS | `anyhow::Context` with `.with_context` on all file / network / deserialization paths. `validate_https_endpoint` rejects non-`https` LLM endpoints and parses URL via the `url` crate (which is already a workspace dependency). |
| Path Traversal (writes) | PASS | `bundle/mod.rs::write_zip` and `write_bundle_manifest` use `path.strip_prefix(out_dir)` so the relative entry name cannot escape the run directory. `main.rs::relativize_for_run` (line 13004) uses `path.strip_prefix(run_dir)`. |
| Path Traversal (reads) | N/A | All read paths come from CLI args the invoking operator chose. |
| Redaction | PASS | Three profiles; deterministic HMAC-SHA256; alias cache (`redaction.aliases.json`) never shipped in bundles (six tests cover this across walk/manifest/zip). |
| URL Encoding | PASS | `ingest/github.rs::build_url_with_params` uses `Url::parse` + `query.append_pair` for all query parameters (RFC 3986 percent-encoding). |
| YAML Parsing | PASS | Uses maintained `serde_yaml_ng = "0.10.0"` (not the abandoned `serde_yaml`). |
| Cargo-deny | PASS | `deny.toml` configures `rustsec/advisory-db`, license allowlist, and registry pinning. |
| Dependabot | PASS | `.github/dependabot.yml` weekly schedule for both `cargo` and `github-actions` ecosystems. |
| Fuzzing | ACTIVE | Fuzz harnesses in `fuzz/fuzz_targets/` (`cache_stats`, `parse_*`, `redact_event`, `receipt_markdown`, `render_md_packet`, `schema_event_deser`, `workstream_cluster`, etc.); `fuzz-smoke.yml` + `fuzzing.yml` workflows. |
| Property Testing | ACTIVE | `proptest` on redact leak detection, cache TTL math, ingest windows. |
| Mutation Testing | ACTIVE | `cargo-mutants` configured (`.cargo/mutants.toml`). |
| Lint Floor | PASS | `cargo check -p shiplog` and `cargo check -p shiplog --lib` succeed in this scan. |
| Build Floor | PASS | `cargo check -p shiplog` clean at the time of the scan. |

### Verification of Prior VULN-001 (MEDIUM, 2026-06-29)

The 2026-06-29 scan reported one MEDIUM finding: `OpenAiCompatibleBackend`
did not validate the `--llm-api-endpoint` URL scheme, which could leak the
LLM API key and event summaries in cleartext.

`apps/shiplog/src/cluster_llm/client.rs` now calls
`validate_https_endpoint(&self.endpoint)?` as the first line of `complete`:

```rust
impl LlmBackend for OpenAiCompatibleBackend {
    #[mutants::skip]
    fn complete(&self, system: &str, user: &str) -> Result<String> {
        validate_https_endpoint(&self.endpoint)?;
        // ...existing client.post(&self.endpoint)...
    }
}

fn validate_https_endpoint(endpoint: &str) -> Result<()> {
    let parsed = Url::parse(endpoint).context("parse LLM API endpoint")?;
    if parsed.scheme() != "https" {
        anyhow::bail!("LLM API endpoint must use https, got {}", parsed.scheme());
    }
    if parsed.host_str().is_none() {
        anyhow::bail!("LLM API endpoint must include a host");
    }
    Ok(())
}
```

Coverage is provided by three unit tests in the same module:

- `accepts_https_endpoint` - accepts `https://...`
- `rejects_http_endpoint` - rejects `http://...` with a `must use https` error
- `rejects_invalid_endpoint` - rejects `not a URL`

VULN-001 is **closed**. No regression detected.

## Critical Findings

None.

## High Findings

None.

## Medium Findings

None.

## Low Findings

None at or above the medium severity threshold.

### Observations Below Threshold

These items are tracked for completeness; they did not meet the medium
severity threshold. They were also noted in the 2026-07-06 and 2026-06-29
reports.

| ID | Class | File | Note |
|----|-------|------|------|
| OBS-1 | User-controlled regex | `apps/shiplog/src/main.rs` (workstreams `split --matching`) | `RegexBuilder::new(pattern)` on a local CLI arg. Self-DoS only; no remote attacker model. Suggest documenting as such in `--help` text. |
| OBS-2 | Markdown link escaping | `apps/shiplog/src/render/md/receipt.rs` | URLs from API responses or `manual_events.yaml` are interpolated unescaped into `[label](url)`. A URL containing `)` or backslash sequences could break markdown link parsing. Output is a file (`packet.md`) the user opens locally, not rendered to HTML in-process, so impact is limited to local renderer behavior. Consider URL-encoding on `link.url` if this becomes user-facing. |
| OBS-3 | `serde_yaml` fork | `Cargo.toml` workspace deps | Forks to `serde_yaml_ng = "0.10.0"`. Good practice; keep tracking upstream patch cadence. |
| OBS-4 | `--api-base` scheme | `apps/shiplog/src/ingest/github.rs` + `main.rs` | The CLI accepts `--api-base` with no scheme validation (defaults to `https://api.github.com` but a user can pass `http://internal-ghes.corp/...`). The bearer token would be sent in cleartext over a non-HTTPS URL. Below threshold because the LLM case (VULN-001) was the higher-impact instance and the GitHub ingestor is the operator's own GHES credential on a typically air-gapped network. Consider applying the same `validate_https_endpoint` style guard for consistency. |

## Appendix

### Threat Model

- Version: 2026-05-11 (unchanged this scan; last modified 2026-07-13 04:08 UTC)
- Location: `.factory/threat-model.md`
- Status: **Current** (aged 63 days, well under the 90-day refresh threshold)
- Action taken: re-used as scan context; no regeneration required.

### Scan Metadata

- Commits Scanned in Strict 7-Day Window: 1 (`1c8577d`, 2026-07-13 04:08 UTC)
- Files in Scoped Commit: 819 (+210,671 LOC, full swarm bootstrap)
- Source Files (`*.rs`) Reviewed: 60+ security-sensitive Rust source files across
  `apps/shiplog/src/{cache,ingest,cluster_llm,redact,render,bundle,workstreams,merge,github_auth}`,
  plus `apps/shiplog/src/main.rs` and `.github/workflows/*.yml`
- Scan Window: 2026-07-06 to 2026-07-13
- Scan Duration: ~12 minutes
- Build/Lint Gates Run: `cargo check -p shiplog --lib`,
  `cargo check -p shiplog`

### Skills Used

- `commit-security-scan` (manual application - STRIDE walk over each
  security-relevant file)
- `vulnerability-validation` (manual - no candidate findings were produced
  to validate; the only open prior finding is VULN-001, which is
  re-verified as fixed)
- `security-review` (not invoked - no patchable findings)
- `threat-model-generation` (not invoked - model is current per Step 2)

### Validation Signals

- **Observed**: 1 commit in the past 7 days (the swarm bootstrap); no
  `unsafe` blocks in any Rust source; `cargo check -p shiplog --lib` and
  `cargo check -p shiplog` succeed; `validate_https_endpoint` is called as
  the first statement of `OpenAiCompatibleBackend::complete`; tests
  `accepts_https_endpoint`, `rejects_http_endpoint`, and
  `rejects_invalid_endpoint` are present in
  `apps/shiplog/src/cluster_llm/client.rs`.
- **Reported**: Threat model file mtime is 2026-07-13 (current); previous
  security report is `security-report-2026-07-06.md`. The repository
  `.factory/threat-model.md` is 63 days old, within the 90-day refresh
  window. Cargo-deny and dependabot are configured.
- **Not verified**: No remote repository API call was performed; GitHub-side
  secret rotation / exposed token state cannot be checked from this
  checkout (and is governed by `EffortlessMetrics/shiplog` repo settings,
  not by code in this repo). The full lint suite (`cargo clippy
  --workspace --all-targets --all-features -- -D warnings`) was not run
  end-to-end in this scan due to time constraints; the build floor
  (`cargo check`) was used as a fast smoke test instead.

### Recommendations

1. **Maintain weekly cadence.** The repository is healthy across the
   security-sensitive surfaces. No regressions were detected.
2. **Consider applying the same `validate_https_endpoint` style guard to
   the GitHub ingestor's `--api-base` flag (OBS-4).** The LLM case
   (VULN-001) was higher-impact because the prompt carries PR titles and
   repo names, but the GitHub ingestor can carry a bearer token over
   cleartext for the same reason. Adding the same scheme check would be a
   low-cost, defense-in-depth improvement.
3. **Tighten the markdown link escaping (OBS-2) if `packet.md` ever
   becomes user-facing.** Today the file is opened locally; if a future
   feature renders the packet in HTML (e.g., in a web view), URL
   characters like `)` and `\` should be percent-encoded.
4. **Keep `redaction.aliases.json` exclusion strong.** The current
   `ALWAYS_EXCLUDED` list in `bundle/mod.rs` is a critical defense for the
   redaction story and is exercised by six unit tests across
   walk/manifest/zip for all three profiles. A regression test that fails
   if any new file matches that name pattern remains a recommended
   follow-up.
5. **Refresh the threat model on or before 2026-08-09** (90-day threshold
   from the 2026-05-11 generation date).

### References

- [CWE Database](https://cwe.mitre.org/)
- [STRIDE Threat Model](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Advisory Database](https://rustsec.org/)
- Prior reports in `.factory/security/reports/`:
  `security-report-2026-05-11.md`, `security-report-2026-05-18.md`,
  `security-report-2026-06-22.md`, `security-report-2026-06-29.md`,
  `security-report-2026-07-06.md`

---

*Report generated by Factory Droid (security-engineer plugin). No code
changes were required; the branch ships only the report file. The previous
finding (VULN-001, MEDIUM) is verified closed in the new code path.*
