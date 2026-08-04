use anyhow::{Context, Result, bail, ensure};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const SOURCE_SHA: &str = "1111111111111111111111111111111111111111";
const RELEASE_ASSETS: [&str; 4] = [
    "shiplog-x86_64-unknown-linux-gnu/shiplog-x86_64-unknown-linux-gnu",
    "shiplog-x86_64-apple-darwin/shiplog-x86_64-apple-darwin",
    "shiplog-aarch64-apple-darwin/shiplog-aarch64-apple-darwin",
    "shiplog-x86_64-pc-windows-msvc/shiplog-x86_64-pc-windows-msvc.exe",
];

struct CandidateFixture {
    _root: TempDir,
    candidate_dir: PathBuf,
    smoke_dir: PathBuf,
    asset_path: PathBuf,
    asset_relative: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn current_release_asset() -> Result<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok(RELEASE_ASSETS[0]),
        ("macos", "x86_64") => Ok(RELEASE_ASSETS[1]),
        ("macos", "aarch64") => Ok(RELEASE_ASSETS[2]),
        ("windows", "x86_64") => Ok(RELEASE_ASSETS[3]),
        (os, arch) => bail!("unsupported release candidate test platform: {os}/{arch}"),
    }
}

fn sha256(path: &Path) -> Result<String> {
    let bytes = fs::read(path).with_context(|| format!("read {}", path.display()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn write_candidate_metadata(fixture: &CandidateFixture) -> Result<()> {
    let sums_path = fixture.candidate_dir.join("SHA256SUMS.txt");
    let mut sums = String::new();
    for relative in RELEASE_ASSETS {
        let path = fixture.candidate_dir.join(relative);
        sums.push_str(&format!("{}  {relative}\n", sha256(&path)?));
    }
    fs::write(&sums_path, sums).with_context(|| format!("write {}", sums_path.display()))?;

    let manifest_path = fixture.candidate_dir.join("RELEASE_CANDIDATE.txt");
    fs::write(
        &manifest_path,
        format!(
            concat!(
                "schema_version=1\n",
                "release_tag=v{}\n",
                "source_sha={}\n",
                "repository=EffortlessMetrics/shiplog-swarm\n",
                "workflow_run_id=fixture\n",
                "workflow_run_attempt=1\n",
                "asset_count=4\n",
                "checksum_manifest_sha256={}\n"
            ),
            env!("CARGO_PKG_VERSION"),
            SOURCE_SHA,
            sha256(&sums_path)?
        ),
    )
    .with_context(|| format!("write {}", manifest_path.display()))?;
    Ok(())
}

fn candidate_fixture() -> Result<CandidateFixture> {
    let root = TempDir::new().context("create release candidate fixture")?;
    let candidate_dir = root.path().join("candidate");
    let smoke_dir = root.path().join("smoke");
    let asset_relative = current_release_asset()?;
    let asset_path = candidate_dir.join(asset_relative);

    for relative in RELEASE_ASSETS {
        let path = candidate_dir.join(relative);
        let parent = path
            .parent()
            .context("candidate asset should have a parent")?;
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
        if relative == asset_relative {
            fs::copy(env!("CARGO_BIN_EXE_shiplog"), &path).with_context(|| {
                format!(
                    "copy test binary {} to {}",
                    env!("CARGO_BIN_EXE_shiplog"),
                    path.display()
                )
            })?;
        } else {
            fs::write(&path, format!("fixture placeholder for {relative}\n"))
                .with_context(|| format!("write {}", path.display()))?;
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&asset_path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&asset_path, permissions)?;
    }

    let fixture = CandidateFixture {
        _root: root,
        candidate_dir,
        smoke_dir,
        asset_path,
        asset_relative: asset_relative.to_string(),
    };
    write_candidate_metadata(&fixture)?;
    Ok(fixture)
}

fn run_candidate_smoke(fixture: &CandidateFixture) -> Result<Output> {
    let root = repo_root();
    let version = env!("CARGO_PKG_VERSION");

    #[cfg(windows)]
    let mut command = {
        let script_path = root.join("scripts/release-install-smoke.ps1");
        let script = script_path
            .to_str()
            .context("PowerShell smoke path is not UTF-8")?;
        let mut command = Command::new("pwsh");
        command.args(["-NoProfile", "-File", script, version]);
        command
    };

    #[cfg(not(windows))]
    let mut command = {
        let script_path = root.join("scripts/release-install-smoke.sh");
        let script = script_path
            .to_str()
            .context("Bash smoke path is not UTF-8")?;
        let mut command = Command::new("bash");
        command.args([script, version]);
        command
    };

    command
        .current_dir(&root)
        .env("SHIPLOG_RELEASE_CANDIDATE_DIR", &fixture.candidate_dir)
        .env("SHIPLOG_RELEASE_SOURCE_SHA", SOURCE_SHA)
        .env("SHIPLOG_RELEASE_SMOKE_DIR", &fixture.smoke_dir)
        .env_remove("GITHUB_TOKEN")
        .env_remove("GH_TOKEN")
        .env_remove("GITLAB_TOKEN")
        .env_remove("JIRA_TOKEN")
        .env_remove("LINEAR_API_KEY")
        .env_remove("SHIPLOG_REDACT_KEY")
        .output()
        .context("run staged release candidate smoke")
}

fn combined_output(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn staged_candidate_smoke_uses_local_bundle_and_emits_receipts() -> Result<()> {
    let fixture = candidate_fixture()?;
    let output = run_candidate_smoke(&fixture)?;
    ensure!(output.status.success(), "{}", combined_output(&output));

    let text = combined_output(&output);
    ensure!(
        text.contains("staged release candidate smoke passed"),
        "candidate mode should report its exact proof boundary: {text}"
    );
    ensure!(
        fixture
            .smoke_dir
            .join("cold-start/status.latest.json")
            .is_file(),
        "candidate smoke should retain the status receipt"
    );
    ensure!(
        fixture
            .smoke_dir
            .join("demo-review-rescue.stdout")
            .is_file(),
        "candidate smoke should retain the no-network rescue transcript"
    );
    Ok(())
}

#[test]
fn staged_candidate_smoke_rejects_checksum_mismatch_before_execution() -> Result<()> {
    let fixture = candidate_fixture()?;
    let mut bytes = fs::read(&fixture.asset_path)?;
    bytes.extend_from_slice(b"corrupt");
    fs::write(&fixture.asset_path, bytes)?;

    let output = run_candidate_smoke(&fixture)?;
    ensure!(
        !output.status.success(),
        "corrupted candidate unexpectedly passed"
    );
    ensure!(
        combined_output(&output).contains("checksum mismatch"),
        "corruption should fail at the checksum boundary: {}",
        combined_output(&output)
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn staged_candidate_smoke_rejects_broken_command_with_matching_checksum() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let fixture = candidate_fixture()?;
    fs::write(&fixture.asset_path, "#!/usr/bin/env bash\nexit 42\n")?;
    let mut permissions = fs::metadata(&fixture.asset_path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fixture.asset_path, permissions)?;
    write_candidate_metadata(&fixture)?;

    let output = run_candidate_smoke(&fixture)?;
    ensure!(
        !output.status.success(),
        "broken candidate unexpectedly passed"
    );
    ensure!(
        combined_output(&output).contains("candidate binary failed --version"),
        "matching checksums must not rescue a broken executable: {}",
        combined_output(&output)
    );
    Ok(())
}
