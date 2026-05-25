# Security Scan Report

**Generated:** 2026-05-25
**Scan Type:** Weekly Scheduled
**Repository:** EffortlessMetrics/shiplog-swarm
**Severity Threshold:** medium

## Executive Summary

| Severity | Count | Auto-fixed | Manual Required |
|----------|-------|------------|-----------------|
| CRITICAL | 0 | 0 | 0 |
| HIGH | 0 | 0 | 0 |
| MEDIUM | 0 | 0 | 0 |
| LOW | 0 | 0 | 0 |

**Total Findings:** 0
**Auto-fixed:** 0
**Manual Review Required:** 0

## Scan Overview

This weekly scheduled security scan examined the shiplog-swarm repository for security vulnerabilities meeting or exceeding the MEDIUM severity threshold.

### Commits Scanned (Last 7 Days)

| Commit | Author | Description |
|--------|--------|-------------|
| 7a35ea7 | Steven Zimmerman, CPA | ci: route workflow jobs to self-hosted runners (#90) |

### Changes Analyzed

The single commit in the scanning window primarily contained:
- CI workflow configuration updates (`.github/workflows/`)
- Cargo build configuration (`.cargo/config.toml`, `.cargo/mutants.toml`)
- Workspace manifest updates (`Cargo.toml`)
- Policy and documentation files

No source code changes involving security-sensitive paths were detected in the scanning window.

### Security Surfaces Analyzed

The scan included review of high-value security surfaces:

| Surface | Status | Notes |
|---------|--------|-------|
| `shiplog::cache` (SQLite) | Reviewed | Uses parameterized queries; no SQL injection vectors |
| `shiplog::redact` (HMAC-SHA256) | Reviewed | Deterministic redaction properly implemented |
| `shiplog::ingest` (API integrations) | Reviewed | Tokens read from environment variables |
| `shiplog::schema` (Event parsing) | Reviewed | Proper input validation in place |
| `shiplog::bundle` (ZIP handling) | Reviewed | Uses zip crate with proper error handling |

### Security Controls Verified

- **SQL Injection Prevention**: SQLite cache uses parameterized queries throughout
- **Secret Management**: API tokens retrieved via `std::env::var()` rather than hardcoding
- **Redaction**: HMAC-SHA256 deterministic aliasing for user identities
- **Error Handling**: Consistent use of `anyhow::Result<T>` with contextual error messages
- **Unsafe Code**: `unsafe_code` lint set to `deny` in workspace lints

## Appendix

### Threat Model

- **Version:** 2026-05-21
- **Location:** `.factory/threat-model.md`
- **Status:** Current (within 90-day refresh window)

### Scan Metadata

- **Commits Scanned:** 1
- **Files Reviewed:** 400+ (full repository)
- **Scan Duration:** <5 minutes
- **Skills Used:** threat-model-generation, commit-security-scan, vulnerability-validation

### References

- [CWE Database](https://cwe.mitre.org/)
- [STRIDE Threat Model](https://docs.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [Rust Security Guidelines](https://doc.rust-lang.org/stable/security/)
