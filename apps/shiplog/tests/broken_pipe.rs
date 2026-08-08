//! A reader that stops early must not turn shiplog into a panic.
//!
//! `shiplog status | head -1` and `shiplog doctor --setup | less` are ordinary
//! usage. Rust's `println!` panics on `EPIPE`, so before the stdio panic hook
//! these exited 101 with a Rust panic dump on stderr.

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use tempfile::TempDir;

fn shiplog_bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_shiplog"))
}

/// Set up a minimal local scaffold so the exercised commands have real output.
fn scaffold(dir: &Path) {
    let status = shiplog_bin()
        .current_dir(dir)
        .args(["start", "--yes"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("failed to run shiplog start");
    assert!(status.success(), "shiplog start --yes failed: {status:?}");
}

/// Run `args` in `dir`, read a single line, then close the read end and report
/// what the child did with the rest of its output.
///
/// This is what `head -1` does to a producer that is still writing.
fn read_one_line_then_close(dir: &Path, args: &[&str]) -> (Option<i32>, String) {
    let mut child = shiplog_bin()
        .current_dir(dir)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn shiplog");

    let stdout = child.stdout.take().expect("child stdout was not piped");
    let mut reader = BufReader::new(stdout);
    let mut first_line = String::new();
    let _ = reader.read_line(&mut first_line);
    // Dropping the reader closes our end of the pipe, so every later write in
    // the child fails with EPIPE.
    drop(reader);

    let output = child
        .wait_with_output()
        .expect("failed to wait for shiplog");
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.code(), stderr)
}

#[test]
fn status_survives_a_reader_that_stops_after_one_line() {
    let tmp = TempDir::new().expect("tempdir");
    scaffold(tmp.path());

    // Repeat: whether the child reaches a post-closure write is a race, and a
    // single attempt could pass without exercising the closure at all.
    for attempt in 0..5 {
        let (code, stderr) = read_one_line_then_close(tmp.path(), &["status", "--latest"]);
        assert!(
            !stderr.contains("panicked"),
            "attempt {attempt}: closing the pipe panicked shiplog:\n{stderr}"
        );
        assert_eq!(
            code,
            Some(0),
            "attempt {attempt}: closing the pipe did not end shiplog successfully"
        );
    }
}

#[test]
fn doctor_setup_survives_a_reader_that_stops_after_one_line() {
    let tmp = TempDir::new().expect("tempdir");
    scaffold(tmp.path());

    for attempt in 0..5 {
        let (code, stderr) = read_one_line_then_close(tmp.path(), &["doctor", "--setup"]);
        assert!(
            !stderr.contains("panicked"),
            "attempt {attempt}: closing the pipe panicked shiplog:\n{stderr}"
        );
        assert_eq!(
            code,
            Some(0),
            "attempt {attempt}: closing the pipe did not end shiplog successfully"
        );
    }
}
