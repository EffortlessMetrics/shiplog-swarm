# Threat Model - shiplog

## System Overview

shiplog is a Rust-based CLI tool (edition 2024, MSRV 1.95) for generating
changelogs and engineering metrics from various data sources (GitHub,
GitLab, Jira, Linear, manual events, local git). The workspace follows
Clean Architecture / ports-and-adapters; `apps/shiplog` is the only public
binary, `shiplog::engine` orchestrates adapters, and the stable foundation
contracts (`shiplog::ids`, `shiplog::schema`, `shiplog::ports`) have no
adapter dependencies.

## Architecture Layers

1. **CLI Layer** (`apps/shiplog`) — clap subcommands + product flow
2. **Engine Layer** (`shiplog::engine`) — orchestration, scope resolution,
   receipt assembly, evidence-binding authority for promotion
3. **Adapter Layer** (`ingest::{github,gitlab,jira,linear,json,manual,git}`)
   — external system integrations
4. **Foundation Layer** (`schema`, `ports`, `ids`, `coverage`, `cache`,
   `redact`, `render`) — stable contracts and shared utilities
5. **Utilities** (`bundle`, `workstreams`, `cluster_llm`, `testkit`,
   `team`)

## Trust Boundaries

1. External APIs (GitHub, GitLab, Jira, Linear) — untrusted input
2. Local filesystem — operator-controlled paths (CLI args, env)
3. Configuration files — operator-controlled
4. Manual event YAML/JSON — operator-controlled
5. `plans/shiplog-swarm/promotion-state.toml` — maintainer-controlled
   evidence ledger
6. `xtask::tasks::transition` evidence chain — verified through
   `derive_authority` / `derive_source_authority` /
   `ensure_evidence_targets`
7. Promotion overlay worktree (`cargo xtask promote`) — synthesized from
   the verified evidence chain and re-derivable by `run_verify_only`
8. GitHub Actions — `pull_request_target` only on canonical source
   (`source-automation-guard.yml`), every other workflow uses
   `pull_request` with same-repo + trusted-actor gates

## STRIDE Analysis

### Spoofing

- **Threat**: Attacker impersonates another user in commit / PR data.
- **Mitigation**: HMAC-SHA256 aliasing for user identities
  (`shiplog::redact::alias`), three profiles (internal / manager /
  public), deterministic per-key.
- **Severity**: Medium

- **Threat**: Attacker hosts a fake `api_base` URL to intercept bearer
  tokens.
- **Mitigation**: `validate_https_api_base` (strict HTTPS, loopback
  carve-out) is invoked from `github_auth::resolve`,
  `discover_github_user`, and `make_github_ingestor` *before* any
  bearer token is materialised. PR #299 closed the token
  exfiltration vector.
- **Severity**: Low (was High before PR #299)

- **Threat**: Operator records a transition receipt that names a path
  the source PR did not actually change.
- **Mitigation**: `ensure_evidence_targets` (added in PR #303 era,
  hardened in #312/#313/#314) requires the recorded targets to be
  ancestors of the current promotion targets and re-validates the
  tree entries at both pairs. `dependency_equivalent` uses
  `git patch-id --stable` for `Cargo.lock` deltas.
- **Severity**: Low (was Medium before the 2026-07-27 / 2026-08-03
  hardening chain)

### Tampering

- **Threat**: Modify cached API responses to alter output.
- **Mitigation**: SQLite-backed cache with TTL
  (`apps/shiplog/src/cache/sqlite.rs`); all eight query sites are
  parameterised through `rusqlite::params!`; the cache-internals seam
  (`cpf-0005`) keeps the raw `Connection` field private.
- **Severity**: Low

- **Threat**: Source PR gets an overlay that replaces source content
  at any path the manifest names.
- **Mitigation**: VULN-001 (2026-07-27) closed by the
  `ensure_evidence_targets` ancestor + dual tree-entry re-validation
  and `validate_transition_path`. `verify_only` (PR #275 lineage)
  re-derives the resolution plan from persisted source / swarm state
  and refuses any modern overlay whose `Shiplog-Resolution-Plan:`
  trailer cannot be reproduced. `compute_resolution_plan` is the
  single source of truth shared between `run_with_port_to` and
  `run_verify_only`.
- **Severity**: Low

- **Threat**: Routine bot (`dependabot[bot]`, `factory-droid[bot]`)
  opens a PR against canonical source.
- **Mitigation**: `source-automation-guard.yml` is metadata-only (no
  checkout, no `secrets.*`, `contents:read` + `pull-requests:read`
  only, `exit 1` on either routine identity). Guarded by
  `inspect_source_bot_guard` (PR #311). `inspect_workflow` (PR #333)
  keeps legitimate source-side `droid-review.yml` / `droid.yml`
  comment writes from being mis-flagged.
- **Severity**: Low

- **Threat**: Promotion overlay worktree leaks into source.
- **Mitigation**: `prepare_source_overlay` claims the worktree with a
  `pid+time` nonce, bounded to 64 retries; cleanup is `Drop`-driven on
  every exit path. `validate_source_authority` (PR #311) enforces
  full 40-hex `source_target` / `swarm_target` for active decisions.
- **Severity**: Low

### Repudiation

- **Threat**: User denies making changes captured by the tool.
- **Mitigation**: `ledger.events.jsonl` is append-only with SHA-256
  `EventId`s (`shiplog::ids`). `run_verify_only` writes a
  machine-readable receipt alongside the resolution plan id and the
  policy that produced it.
- **Severity**: Low

- **Threat**: Receipt contents drift from the policy that produced
  them.
- **Mitigation**: `validate` (top-level manifest validator) checks
  schema version, status enum, transitions, deferred receipts, and
  `source_authority` decisions. `validate_full_sha` requires 40 hex
  chars. `validate_receipt` checks the leading-SHA format.
- **Severity**: Low

### Information Disclosure

- **Threat**: Exfiltrate bearer tokens to a remote host by setting a
  malicious `api_base` URL.
- **Mitigation**: `validate_https_api_base` is invoked *before* any
  bearer token is materialised in `github_auth::resolve`,
  `discover_github_user`, and `make_github_ingestor`. PR #299 closed
  this vector. The new integration tests
  `collect_github_me_rejects_remote_http_before_token_or_network` and
  `auth_github_status_rejects_remote_http_before_environment_credentials`
  stand up a `TcpListener::bind("0.0.0.0:0")` and assert that the
  request count is `0` and that the token string never appears in
  either stdout or stderr.
- **Severity**: Low (was High before PR #299)

- **Threat**: LLM clustering leaks bearer / OAuth tokens over cleartext.
- **Mitigation**: `cluster_llm::client::validate_https_endpoint`
  refuses any non-`https` scheme; no loopback carve-out.
- **Severity**: Low

- **Threat**: Bundle zip or `redaction.aliases.json` carries the
  redaction key out of the run directory.
- **Mitigation**: `bundle/mod.rs::write_zip` writes to a path provided
  by the operator; `path.strip_prefix(out_dir)` prevents the relative
  entry name from escaping. Alias cache is generated only when the
  `Redactor` has a configured key.
- **Severity**: Low

- **Threat**: Scaffold identity placeholder ("Your Name") leaks into
  a published packet.
- **Mitigation**: `doctor::build_identity_item` (PR #290) reports
  `ReadyWithCaveats` whenever `[user].label` or `[sources.manual].user`
  still holds the `SCAFFOLD_USER_PLACEHOLDER`. The constant is shared
  between the scaffold template and the placeholder detector so the
  two cannot drift.
- **Severity**: Low

### Denial of Service

- **Threat**: Exhaust disk space with unbounded cache.
- **Mitigation**: TTL-based cleanup (`cache::expiry`), `with_max_size`
  configuration, `cleanup_expired` / `cleanup_older_than` exposed via
  `cache` subcommand.
- **Severity**: Low

- **Threat**: Operator-controlled regex blows up the workstream matcher.
- **Mitigation**: `regex = "1.12.3"` (linear-time engine) for
  `workstreams split --matching` and `cluster`. Patterns are
  operator-supplied CLI input, not network-reachable.
- **Severity**: Low

- **Threat**: Promotion overlay worktree spin loop blocks xtask.
- **Mitigation**: Bounded to 64 retries with a `pid+time` nonce;
  `Drop`-driven cleanup on every exit path.
- **Severity**: Low

- **Threat**: PR body growth overwhelms the contributor CI lane.
- **Mitigation**: `contributor-acceptance.yml` is label-gated
  (`contributor-acceptance` / `full-ci`); PR body is parsed via
  `gh pr view --json body` with hardcoded argv.
- **Severity**: Low

### Elevation of Privilege

- **Threat**: Execute arbitrary code via a malicious config or
  template.
- **Mitigation**: `[workspace.lints.rust] unsafe_code = "deny"` (no
  `unsafe` blocks in production code); no `Command::new` with
  operator-supplied argv (`run_gh`, `gh_command`, `try_open_path`
  all use hardcoded argv vectors with `.arg(path)` or
  `.args(arguments)`; no shell). No template engine eval.
- **Severity**: Low

- **Threat**: Run-start / init writes over an existing
  `shiplog.toml` / `manual_events.yaml`.
- **Mitigation**: `run_start` requires `--yes` before any
  filesystem write; `ensure_init_files_available` refuses to
  overwrite unless `--force` is passed.
- **Severity**: Low

- **Threat**: Fork PR gains write access to
  `EffortlessMetrics/shiplog-swarm` via a misconfigured Droid
  workflow.
- **Mitigation**: All Droid workflows (`droid.yml`,
  `droid-review.yml`, `droid-security-scan.yml`) gate on
  `github.event.pull_request.head.repo.full_name == github.repository`
  for `pull_request` triggers; manual `@droid` triggers gate on
  `author_association ∈ {OWNER, MEMBER, COLLABORATOR}`.
  `pull_request_target` only appears in
  `source-automation-guard.yml`, which is metadata-only and gated on
  `github.repository == 'EffortlessMetrics/shiplog'`.
- **Severity**: Low

- **Threat**: Forge a transition receipt that names an attacker-
  controlled path.
- **Mitigation**: `validate_transition_path` mirrors the
  `policy/source-only-paths.toml` allowlist (directory globs
  intentionally unsupported); `validate_source_authority` rejects
  empty paths, duplicate paths, malformed decision receipts,
  malformed decision merge SHAs, empty reasons, and active decisions
  without exact evidence targets. `validate_receipt` checks leading
  SHA format.
- **Severity**: Low

## Key Security Features

1. **Deterministic Redaction**: Same input + key = same alias via
   HMAC-SHA256 (`shiplog::redact::alias`)
2. **Receipts-First**: Every claim traces to fetched evidence;
   `coverage.manifest.json` records what was *not* fetched
3. **Immutable Ledger**: `ledger.events.jsonl` is append-only with
   SHA-256 `EventId`s (`shiplog::ids`)
4. **Cache Expiry**: TTL-based cleanup, parameterised SQL throughout
   `apps/shiplog/src/cache/sqlite.rs`
5. **Input Validation**: `validate_https_api_base`,
   `validate_https_endpoint`, `validate_full_sha`,
   `validate_receipt`, `validate_transition_path`,
   `validate_source_authority`
6. **HTTPS-Only by Default**: All authenticated network calls go
   through `validate_https_*` before secrets are loaded
7. **No unsafe code**: Workspace lint `unsafe_code = "deny"`
8. **No shellout**: All `Command::new` call sites use hardcoded argv

## Data Flow

1. **Ingest**: Events fetched from external APIs or local files
2. **Cache**: API responses cached in SQLite with TTL
3. **Cluster**: Events grouped into workstreams via user-curated YAML
4. **Redact**: Sensitive data masked via HMAC-SHA256 aliasing
5. **Render**: Output generated as markdown or JSON
6. **Bundle**: Run directory zipped with `bundle/write_zip`; alias
   cache emitted to `redaction.aliases.json` only when redaction was
   performed

## External Dependencies

- `rusqlite = "0.40.2"` (bundled, parameterised queries only)
- `reqwest = "0.13.4"` (blocking + json features; no native-tls)
- `serde = "1.0.228"` / `serde_json = "1.0.150"` /
  `serde_yaml = { package = "serde_yaml_ng", version = "0.10.0" }`
- `chrono = "0.4.44"` (date / time handling)
- `tokio = "1.53.0"` (feature-gated, dev-only)

## Security Controls

- **Fuzzing**: 36 fuzz targets in `fuzz/fuzz_targets/`;
  `fuzz-smoke.yml` + `fuzzing.yml` workflows
- **Property-based testing**: `proptest` on redact leak detection,
  cache TTL math, ingest windows
- **Mutation testing**: `cargo-mutants` configured
  (`cargo-mutants.toml`, `.cargo/mutants.toml`)
- **Clippy linting**: `[workspace.lints.clippy]` with strict denials
  for correctness, unsafe code, manual bit math
- **No panic / unwrap in production**: `unwrap()` / `expect()` calls
  are confined to test modules; production code uses `anyhow` /
  `Result`
- **No shellout**: All `Command::new` call sites use hardcoded argv
- **Action pinning**:
  `EffortlessMetrics/droid-action-safe@7c1377ccbacddc95560d1570547a5baa51de01ec`,
  `actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1`
  (v7.0.1)
- **Workflow allowlist**: `policy/workflow-allowlist.toml` and
  `policy/source-only-paths.toml` enforce per-workflow permissions,
  secrets, and external actions; `inspect_workflow` /
  `inspect_source_bot_guard` validate at xtask runtime
- **cargo-deny**: weekly cron (`security.yml`), PR lane
  (`ci.yml#deny`)
- **Droid automation guards**: same-repo gate, trusted-actor gate,
  no `pull_request_target` outside canonical source

---

*Generated: 2026-08-24 (regenerated after 105-day staleness; was
2026-05-11).*
