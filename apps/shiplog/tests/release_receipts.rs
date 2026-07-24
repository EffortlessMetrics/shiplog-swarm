//! Structural receipt validation for release first-use acceptance (issue #245).
//!
//! `shiplog report validate --receipts` must structurally validate a run's
//! durable receipts — `packet.md`, `ledger.events.jsonl`,
//! `coverage.manifest.json`, `bundle.manifest.json` — not merely check that
//! they exist. Release first-use acceptance drives the published binary against
//! an empty directory and then runs this command, so a broken receipt fails
//! acceptance instead of shipping.
//!
//! These tests drive the same cargo-built binary a release download would
//! provide (via `CARGO_BIN_EXE_shiplog`), generate a real cold-start run, and
//! assert both the healthy path and that each individual receipt corruption is
//! caught. Each test generates its own run and corrupts a receipt in place —
//! the intake report records its own absolute path, so a run must not be moved.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn shiplog_cmd(cwd: &Path) -> Command {
    let mut cmd = Command::from_std(std::process::Command::new(env!("CARGO_BIN_EXE_shiplog")));
    cmd.current_dir(cwd)
        .env_remove("GITHUB_TOKEN")
        .env_remove("GITLAB_TOKEN")
        .env_remove("JIRA_TOKEN")
        .env_remove("LINEAR_API_KEY")
        .env_remove("SHIPLOG_REDACT_KEY");
    cmd
}

/// Produce a real cold-start run and return (tempdir, run_dir). The tempdir
/// must outlive use of `run_dir`.
fn cold_start_run() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("tempdir");
    let out = tmp.path().join("out");
    shiplog_cmd(tmp.path())
        .args([
            "intake",
            "--last-6-months",
            "--out",
            out.to_str().expect("utf-8 out path"),
            "--no-open",
        ])
        .assert()
        .success();
    let run_dir = fs::read_dir(&out)
        .expect("read out")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| path.is_dir() && path.join("intake.report.json").exists())
        .expect("cold-start run directory with intake.report.json");
    (tmp, run_dir)
}

/// `shiplog report validate --path <run>/intake.report.json [--receipts]`.
fn validate(run: &Path, receipts: bool) -> Command {
    let report = run.join("intake.report.json");
    let mut cmd = Command::from_std(std::process::Command::new(env!("CARGO_BIN_EXE_shiplog")));
    cmd.arg("report")
        .arg("validate")
        .arg("--path")
        .arg(report.to_str().expect("utf-8 report path"));
    if receipts {
        cmd.arg("--receipts");
    }
    cmd
}

/// Assert `--receipts` validation fails AND that it failed for the expected
/// receipt-specific reason, so a future regression that fails at the same call
/// site for an unrelated reason cannot pass silently.
fn assert_receipts_reject(run: &Path, expected_error: &str) {
    let assert = validate(run, true).assert().failure();
    let stderr = String::from_utf8_lossy(&assert.get_output().stderr);
    assert!(
        stderr.contains(expected_error),
        "expected receipts failure to mention {expected_error:?}. stderr:\n{stderr}"
    );
}

#[test]
fn receipts_validation_accepts_a_healthy_cold_start_run() {
    let (_tmp, run) = cold_start_run();
    let assert = validate(&run, true).assert().success();
    let stdout = String::from_utf8_lossy(&assert.get_output().stdout);
    assert!(
        stdout.contains("Receipts: 4 structurally validated"),
        "expected receipts summary line. stdout:\n{stdout}"
    );
}

#[test]
fn receipts_validation_rejects_malformed_coverage_manifest() {
    let (_tmp, run) = cold_start_run();
    fs::write(run.join("coverage.manifest.json"), "not valid json{").expect("corrupt coverage");
    assert_receipts_reject(&run, "coverage receipt");
}

#[test]
fn receipts_validation_rejects_malformed_bundle_manifest() {
    let (_tmp, run) = cold_start_run();
    // Valid JSON, but missing every required BundleManifest field.
    fs::write(run.join("bundle.manifest.json"), "{}").expect("corrupt bundle");
    assert_receipts_reject(&run, "bundle receipt");
}

#[test]
fn receipts_validation_rejects_malformed_ledger_line() {
    let (_tmp, run) = cold_start_run();
    let ledger = run.join("ledger.events.jsonl");
    let pristine = fs::read_to_string(&ledger).expect("read ledger");
    fs::write(
        &ledger,
        format!("{pristine}\nthis line is not a json event\n"),
    )
    .expect("corrupt ledger");
    assert_receipts_reject(&run, "is not a well-formed event record");
}

#[test]
fn receipts_validation_rejects_packet_missing_required_sections() {
    let (_tmp, run) = cold_start_run();
    fs::write(
        run.join("packet.md"),
        "# Something Else\n\nno required sections here\n",
    )
    .expect("corrupt packet");
    assert_receipts_reject(&run, "missing required section");
}

#[test]
fn receipts_flag_is_opt_in_and_does_not_change_default_validate() {
    // Without --receipts, a broken coverage manifest still passes: the flag is
    // the only thing that adds structural receipt validation.
    let (_tmp, run) = cold_start_run();
    fs::write(run.join("coverage.manifest.json"), "not valid json{").expect("corrupt coverage");
    validate(&run, false).assert().success();
    // ...and with the flag, the same corruption is rejected.
    validate(&run, true).assert().failure();
}
