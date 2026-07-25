//! Transition receipts: bounded divergence authority for the swarm cutover.
//!
//! During the cutover, some changes landed directly on the source repository
//! instead of flowing through a swarm promotion. A transition receipt records
//! one such source PR and, per path, what the swarm side did about it. That
//! evidence is what lets `promote` accept a difference between the two trees
//! that `policy/source-only-paths.toml` does not permanently allow.
//!
//! Three properties keep this from becoming a standing bypass:
//!
//! * **Per-path disposition.** A source PR is rarely uniform. The port that
//!   landed as source #666 / swarm #274 left seven paths byte-identical and six
//!   intentionally different, so one status over one path list would have been
//!   false for half of it.
//! * **Bounded lifetime.** An entry grants nothing once `consumed_by` names the
//!   promotion that reconciled it. Entries are kept as history, not authority.
//! * **Narrow authority.** Only a resolved receipt (`equivalent`,
//!   `superseded_in_swarm`) grants anything, and only permission for the overlay
//!   to keep swarm content on a path both sides changed, because swarm
//!   demonstrably already carries or supersedes the source change.
//!   `missing_in_swarm` grants nothing and blocks: swarm does not carry the
//!   change, so promoting would revert it. `conflicting` blocks outright.
//!
//! Evidence is checked rather than assumed: the recorded merge SHAs must belong
//! to merged PRs and be reachable from the relevant branch, `equivalent` must
//! agree under `git patch-id --stable` for that path, and
//! `superseded_in_swarm` must present a contiguous version chain starting where
//! the source change started.

use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::promotion_state::{Transition, TransitionDisposition, TransitionPath};

/// Divergence the transition receipts justify, split by the shape of the
/// difference they actually prove.
#[derive(Debug, Default)]
pub struct TransitionAuthority {
    /// Paths both repositories changed, where a receipt proves the two changes
    /// are reconciled, so the overlay may keep the swarm content. Carries the
    /// evidence that earned it, so a decision can state its basis rather than
    /// just its effect.
    pub two_sided: BTreeMap<String, ResolvedReceipt>,
    /// Paths a receipt records as changed on source with no swarm counterpart,
    /// mapped to the receipt that records it.
    ///
    /// These grant nothing. The overlay keeps swarm content for any path the
    /// policy does not mark source-authoritative, so promoting would revert the
    /// source change. They are carried only so the refusal can name the receipt
    /// and say what to do about it.
    pub awaiting_swarm: BTreeMap<String, String>,
    /// Source merge commits an active receipt accounts for. A source commit that
    /// followed the last promotion merge is otherwise unapproved divergence, so
    /// without this the path-level authority above could never be reached.
    pub source_commits: BTreeSet<String>,
}

/// The evidence behind a resolved path grant.
#[derive(Clone, Debug)]
pub struct ResolvedReceipt {
    pub source_pr: String,
    pub swarm_chain: Vec<String>,
    pub disposition: TransitionDisposition,
}

/// Repository access the evidence checks need.
pub trait TransitionPort {
    fn git_output(&self, workspace_root: &Path, args: &[&str]) -> Result<String>;
    fn gh_output(&self, args: &[&str]) -> Result<Vec<u8>>;
    /// `git patch-id --stable` over `patch`, returning the patch id alone.
    fn git_patch_id(&self, patch: &str) -> Result<String>;
}

/// Where a receipt's recorded merge commit must be reachable from.
pub struct TransitionRefs<'a> {
    pub source_repo: &'a str,
    pub swarm_repo: &'a str,
    pub source_ref: &'a str,
    pub swarm_ref: &'a str,
}

/// Check every active transition receipt and return the divergence they earn.
pub fn derive_authority(
    port: &impl TransitionPort,
    workspace_root: &Path,
    refs: &TransitionRefs<'_>,
    transitions: &[Transition],
) -> Result<TransitionAuthority> {
    let mut authority = TransitionAuthority::default();
    for entry in transitions {
        // A consumed receipt is retained as history and skipped. Re-validating it
        // would make a migration record permanently load-bearing, which is the
        // failure `consumed_by` exists to prevent.
        if entry.consumed_by().is_some() {
            continue;
        }
        check_merged_at(
            port,
            workspace_root,
            refs.source_repo,
            &entry.source_pr,
            &entry.source_merge_sha,
            refs.source_ref,
        )
        .with_context(|| format!("transition {}: source merge evidence", entry.source_pr))?;

        // Recorded only after the merge evidence above passed, so an unverified
        // receipt cannot let a source commit through the ancestry walk.
        authority
            .source_commits
            .insert(entry.source_merge_sha.clone());

        let source_patch = pull_request_patch(port, refs.source_repo, &entry.source_pr)?;
        for path in &entry.path {
            check_path(
                port,
                workspace_root,
                refs,
                entry,
                path,
                &source_patch,
                &mut authority,
            )
            .with_context(|| {
                format!(
                    "transition {} path {}: {}",
                    entry.source_pr, path.path, path.disposition
                )
            })?;
        }
    }
    Ok(authority)
}

fn check_path(
    port: &impl TransitionPort,
    workspace_root: &Path,
    refs: &TransitionRefs<'_>,
    entry: &Transition,
    path: &TransitionPath,
    source_patch: &str,
    authority: &mut TransitionAuthority,
) -> Result<()> {
    if path.disposition == TransitionDisposition::Conflicting {
        bail!("marked conflicting; resolve it before promoting");
    }
    // Established before any disposition grants anything. A receipt may only
    // speak for paths its own source PR touched, otherwise one merged PR would
    // authorize divergence on an unrelated file it never changed.
    let source_section = patch_for_path(source_patch, &path.path).with_context(|| {
        format!(
            "source PR {} does not touch this path",
            entry.source_pr.as_str()
        )
    })?;

    if path.disposition == TransitionDisposition::MissingInSwarm {
        if !path.swarm_chain.is_empty() {
            bail!("missing_in_swarm must not name swarm PRs");
        }
        // Deliberately not an authority grant. Swarm does not carry this change,
        // and the overlay keeps swarm content for paths outside
        // `policy/source-only-paths.toml`, so promoting would revert it. Record
        // it so the refusal can explain itself.
        authority
            .awaiting_swarm
            .insert(path.path.clone(), entry.source_pr.clone());
        return Ok(());
    }

    if path.swarm_chain.is_empty() {
        bail!("{} requires at least one swarm PR", path.disposition);
    }
    let mut swarm_patches = Vec::new();
    for swarm_pr in &path.swarm_chain {
        let merge_sha = entry.swarm_merge_sha(swarm_pr).with_context(|| {
            format!("{swarm_pr} is named in a chain but has no recorded swarm_merge_sha")
        })?;
        check_merged_at(
            port,
            workspace_root,
            refs.swarm_repo,
            swarm_pr,
            merge_sha,
            refs.swarm_ref,
        )?;
        swarm_patches.push(pull_request_patch(port, refs.swarm_repo, swarm_pr)?);
    }

    match path.disposition {
        TransitionDisposition::Equivalent => {
            if path.swarm_chain.len() != 1 {
                bail!("equivalent must name exactly one swarm PR, not a chain");
            }
            let swarm_section =
                patch_for_path(&swarm_patches[0], &path.path).with_context(|| {
                    format!("swarm PR {} does not touch this path", path.swarm_chain[0])
                })?;
            let source_id = port.git_patch_id(&source_section)?;
            let swarm_id = port.git_patch_id(&swarm_section)?;
            if source_id != swarm_id {
                bail!(
                    "claimed equivalent but patch ids differ: source {source_id}, swarm {swarm_id}"
                );
            }
        }
        TransitionDisposition::SupersededInSwarm => {
            // The chain must begin by reproducing the source change, not merely
            // by starting at the same version. Without this, a swarm step from
            // the same old version to an unrelated new one satisfied the walk
            // while never incorporating what source landed.
            let first = patch_for_path(&swarm_patches[0], &path.path).with_context(|| {
                format!(
                    "chained swarm PR {} does not touch this path",
                    path.swarm_chain[0]
                )
            })?;
            let reproduces = port.git_patch_id(&source_section)? == port.git_patch_id(&first)?
                || reaches_source_result(&source_section, &first);
            if !reproduces {
                bail!(
                    "chain does not start by reproducing the source change: {} must be equivalent to {} for this path, or land the same resulting versions",
                    path.swarm_chain[0],
                    entry.source_pr
                );
            }
            check_supersession_chain(&source_section, &swarm_patches, &path.path)?;
        }
        _ => unreachable!("handled above"),
    }
    // A resolved receipt reconciles a path both sides touched. It deliberately
    // does not grant one-sided source authority: only
    // `policy/source-only-paths.toml` may select source content, so a settled
    // migration can never manufacture temporary source authority.
    authority.two_sided.insert(
        path.path.clone(),
        ResolvedReceipt {
            source_pr: entry.source_pr.clone(),
            swarm_chain: path.swarm_chain.clone(),
            disposition: path.disposition,
        },
    );
    Ok(())
}

/// Confirm the PR is merged, the recorded SHA is that merge, and the merge is
/// reachable from the branch that repository promotes from.
///
/// Recording a SHA is not evidence on its own: a well-formed but wrong SHA is
/// indistinguishable from a correct one without asking the forge which commit
/// the PR actually produced.
fn check_merged_at(
    port: &impl TransitionPort,
    workspace_root: &Path,
    repo: &str,
    receipt: &str,
    merge_sha: &str,
    reachable_from: &str,
) -> Result<()> {
    let number = receipt_number(receipt, repo)?;
    let raw = port.gh_output(&[
        "pr",
        "view",
        &number,
        "--repo",
        repo,
        "--json",
        "state,mergeCommit",
    ])?;
    let view: serde_json::Value =
        serde_json::from_slice(&raw).with_context(|| format!("parse {receipt} PR view"))?;
    let state = view.get("state").and_then(|value| value.as_str());
    if state != Some("MERGED") {
        bail!("{receipt} is {}, not MERGED", state.unwrap_or("unknown"));
    }
    let merged = view
        .get("mergeCommit")
        .and_then(|commit| commit.get("oid"))
        .and_then(|oid| oid.as_str())
        .with_context(|| format!("{receipt} reports no merge commit"))?;
    // Compared in full rather than by prefix. This is a fail-closed authority
    // boundary, so an abbreviation that merely happens to share a prefix must not
    // pass as evidence.
    if merged != merge_sha {
        bail!("{receipt} merged as {merged}, but the receipt records {merge_sha}");
    }
    // Reachability is checked locally: the forge can say what a PR merged as,
    // but only ancestry proves that commit is still part of the promoted line
    // rather than something later rewritten away.
    port.git_output(
        workspace_root,
        &["merge-base", "--is-ancestor", merge_sha, reachable_from],
    )
    .with_context(|| {
        format!("{receipt} merge {merge_sha} is not reachable from {reachable_from}")
    })?;
    Ok(())
}

/// Require the swarm steps to form an unbroken version chain that starts where
/// the source change started.
///
/// Proving the versions merely differ is not supersession: an unrelated
/// downgrade also differs. The chain has to begin at the source change's own
/// starting version so the swarm side demonstrably continues that history.
fn check_supersession_chain(
    source_section: &str,
    swarm_patches: &[String],
    path: &str,
) -> Result<()> {
    let source = lock_transitions(source_section);
    if source.is_empty() {
        bail!("no Cargo.lock package versions found in the source change for {path}");
    }
    let mut steps = Vec::new();
    for patch in swarm_patches {
        let section = patch_for_path(patch, path)
            .with_context(|| format!("a chained swarm PR does not touch {path}"))?;
        steps.push(lock_transitions(&section));
    }
    let mut proven = false;
    for (package, (source_from, source_to)) in &source {
        let mut cursor = match source_from {
            Some(from) => from.clone(),
            None => continue,
        };
        let mut advanced = false;
        for step in &steps {
            let Some((step_from, step_to)) = step.get(package) else {
                continue;
            };
            let (Some(step_from), Some(step_to)) = (step_from, step_to) else {
                continue;
            };
            if step_from != &cursor {
                bail!(
                    "chain for {package} is not contiguous: expected a step starting at {cursor}, found one starting at {step_from}"
                );
            }
            cursor = step_to.clone();
            advanced = true;
        }
        if !advanced {
            continue;
        }
        if let Some(source_to) = source_to
            && &cursor == source_to
        {
            bail!(
                "chain for {package} ends at {cursor}, the same version source landed; that is equivalent, not superseded"
            );
        }
        proven = true;
    }
    if !proven {
        bail!("no chained swarm change continues the source version history for {path}");
    }
    Ok(())
}

/// `name`/`version` pairs a Cargo.lock diff moves, as `package -> (from, to)`.
fn lock_transitions(section: &str) -> BTreeMap<String, (Option<String>, Option<String>)> {
    let mut transitions: BTreeMap<String, (Option<String>, Option<String>)> = BTreeMap::new();
    let mut package: Option<String> = None;
    for line in section.lines() {
        if line.starts_with("+++") || line.starts_with("---") || line.starts_with("diff --git") {
            continue;
        }
        let Some(content) = line.strip_prefix('+').or_else(|| line.strip_prefix('-')) else {
            if let Some(rest) = line.strip_prefix(' ')
                && let Some(name) = keyed_value(rest, "name")
            {
                package = Some(name);
            }
            continue;
        };
        if let Some(name) = keyed_value(content, "name") {
            package = Some(name);
            continue;
        }
        if let Some(version) = keyed_value(content, "version")
            && let Some(name) = package.clone()
        {
            let entry = transitions.entry(name).or_insert((None, None));
            if line.starts_with('-') {
                entry.0 = Some(version);
            } else {
                entry.1 = Some(version);
            }
        }
    }
    transitions
}

/// Parse a `key = "value"` line through the TOML parser, so escapes and
/// malformed values are rejected rather than mis-sliced.
fn keyed_value(line: &str, key: &str) -> Option<String> {
    let line = line.trim();
    if !line.strip_prefix(key)?.trim_start().starts_with('=') {
        return None;
    }
    let table = line.parse::<toml::Table>().ok()?;
    let value = table.get(key)?.as_str()?;
    (!value.is_empty()).then(|| value.to_string())
}

/// The `diff --git` section for exactly `path`.
///
/// Identity is computed per section so it carries the file it belongs to. A
/// whole-patch digest over added and removed lines would treat the same edit
/// applied to a different file as the same change.
fn patch_for_path(patch: &str, path: &str) -> Option<String> {
    let mut section: Option<Vec<&str>> = None;
    for line in patch.lines() {
        if let Some(rest) = line.strip_prefix("diff --git ") {
            let matches = rest
                .split_whitespace()
                .nth(1)
                .and_then(|value| value.strip_prefix("b/"))
                .is_some_and(|value| value == path);
            if matches {
                section = Some(vec![line]);
                continue;
            }
            if section.is_some() {
                break;
            }
            continue;
        }
        if let Some(lines) = section.as_mut() {
            lines.push(line);
        }
    }
    let lines = section?;
    let mut text = lines.join("\n");
    text.push('\n');
    Some(text)
}

/// Stable patch identity via git itself, rather than a hand-rolled digest.
///
/// `--stable` ignores line numbers but keeps file identity, so the same edit
/// applied to a different file is a different id.
pub fn system_patch_id(patch: &str) -> Result<String> {
    let stdout = run_git_with_stdin(&["patch-id", "--stable"], patch)?;
    // Output is "<patch-id> <commit-id>"; an empty patch yields nothing at all,
    // which must not silently compare equal to another empty result.
    stdout
        .split_whitespace()
        .next()
        .map(str::to_string)
        .context("transition: git patch-id produced no id for the supplied patch")
}

/// `git hash-object` over arbitrary content.
///
/// Gives a stable content id using git's own hashing rather than taking on a
/// digest dependency for one identifier.
pub fn git_hash_object(content: &str) -> Result<String> {
    run_git_with_stdin(&["hash-object", "--stdin"], content)?
        .split_whitespace()
        .next()
        .map(str::to_string)
        .context("transition: git hash-object produced no id")
}

fn run_git_with_stdin(args: &[&str], stdin: &str) -> Result<String> {
    use std::io::Write as _;
    use std::process::{Command, Stdio};

    let mut child = Command::new("git")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("transition: run git {}", args.join(" ")))?;
    child
        .stdin
        .take()
        .context("transition: git stdin unavailable")?
        .write_all(stdin.as_bytes())
        .context("transition: write to git stdin")?;
    let output = child
        .wait_with_output()
        .context("transition: collect git output")?;
    if !output.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn pull_request_patch(port: &impl TransitionPort, repo: &str, receipt: &str) -> Result<String> {
    let number = receipt_number(receipt, repo)?;
    let raw = port
        .gh_output(&["pr", "diff", &number, "--repo", repo, "--patch"])
        .with_context(|| format!("fetch patch for {receipt}"))?;
    Ok(String::from_utf8_lossy(&raw).into_owned())
}

/// PR number from `owner/repo#number`, rejecting a receipt that names a
/// different repository than the one about to be queried.
///
/// Discarding the repository component would let a manifest record
/// `OtherOrg/OtherRepo#657` while the verifier answered from the expected
/// repository's #657 instead.
fn receipt_number(receipt: &str, expected_repo: &str) -> Result<String> {
    let (repo, number) = receipt
        .split_once('#')
        .with_context(|| format!("receipt {receipt:?} must be owner/repo#number"))?;
    if repo != expected_repo {
        bail!("receipt {receipt:?} names repository {repo:?}, expected {expected_repo:?}");
    }
    if number.is_empty() || !number.chars().all(|c| c.is_ascii_digit()) {
        bail!("receipt {receipt:?} must end with a numeric PR id");
    }
    Ok(number.to_string())
}

/// Whether a swarm path patch lands the same resulting versions as the source
/// patch, which is equivalence in effect even when the patch text differs
/// because the two repositories started from different content.
fn reaches_source_result(source_section: &str, swarm_section: &str) -> bool {
    let source = lock_transitions(source_section);
    let swarm = lock_transitions(swarm_section);
    if source.is_empty() {
        return false;
    }
    source.iter().all(|(package, (_, source_to))| {
        swarm
            .get(package)
            .is_some_and(|(_, swarm_to)| swarm_to == source_to)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    const CARGO_LOCK: &str = "Cargo.lock";
    const SOURCE: &str = "EffortlessMetrics/shiplog";
    const SWARM: &str = "EffortlessMetrics/shiplog-swarm";

    struct StubPort {
        /// `repo#n` to the PR view JSON the forge would return.
        views: BTreeMap<String, String>,
        /// `repo#n` to that PR's patch.
        patches: BTreeMap<String, String>,
        /// SHAs that are reachable from the ref they are checked against.
        reachable: BTreeSet<String>,
        ancestry_checks: RefCell<Vec<String>>,
    }

    impl StubPort {
        fn new() -> Self {
            Self {
                views: BTreeMap::new(),
                patches: BTreeMap::new(),
                reachable: BTreeSet::new(),
                ancestry_checks: RefCell::new(Vec::new()),
            }
        }

        fn merged(mut self, receipt: &str, merge_sha: &str, patch: &str) -> Self {
            let key = key_for(receipt);
            self.views.insert(
                key.clone(),
                format!("{{\"state\":\"MERGED\",\"mergeCommit\":{{\"oid\":\"{merge_sha}\"}}}}"),
            );
            self.patches.insert(key, patch.to_string());
            self.reachable.insert(merge_sha.to_string());
            self
        }

        fn open(mut self, receipt: &str, patch: &str) -> Self {
            let key = key_for(receipt);
            self.views.insert(
                key.clone(),
                "{\"state\":\"OPEN\",\"mergeCommit\":null}".to_string(),
            );
            self.patches.insert(key, patch.to_string());
            self
        }

        fn unreachable(mut self, merge_sha: &str) -> Self {
            self.reachable.remove(merge_sha);
            self
        }
    }

    fn key_for(receipt: &str) -> String {
        receipt.to_string()
    }

    impl TransitionPort for StubPort {
        fn git_output(&self, _workspace_root: &Path, args: &[&str]) -> Result<String> {
            if args.first() == Some(&"merge-base") {
                let sha = args
                    .get(2)
                    .context("stub: merge-base without a sha")?
                    .to_string();
                self.ancestry_checks.borrow_mut().push(sha.clone());
                if !self.reachable.contains(&sha) {
                    bail!("stub: {sha} is not an ancestor");
                }
                return Ok(String::new());
            }
            bail!("stub: unexpected git {args:?}")
        }

        fn gh_output(&self, args: &[&str]) -> Result<Vec<u8>> {
            let number = args.get(2).context("stub: gh call without a number")?;
            let repo = args
                .iter()
                .position(|arg| *arg == "--repo")
                .and_then(|index| args.get(index + 1))
                .context("stub: gh call without a repo")?;
            let key = format!("{repo}#{number}");
            match args.get(1) {
                Some(&"view") => {
                    let view = self
                        .views
                        .get(&key)
                        .with_context(|| format!("stub: no view for {key}"))?;
                    Ok(view.clone().into_bytes())
                }
                Some(&"diff") => {
                    let patch = self
                        .patches
                        .get(&key)
                        .with_context(|| format!("stub: no patch for {key}"))?;
                    Ok(patch.clone().into_bytes())
                }
                other => bail!("stub: unexpected gh pr {other:?}"),
            }
        }

        fn git_patch_id(&self, patch: &str) -> Result<String> {
            system_patch_id(patch)
        }
    }

    fn refs() -> TransitionRefs<'static> {
        TransitionRefs {
            source_repo: SOURCE,
            swarm_repo: SWARM,
            source_ref: "origin/main",
            swarm_ref: "swarm/main",
        }
    }

    fn entry(disposition: TransitionDisposition, chain: &[(&str, &str)]) -> Transition {
        let mut swarm_merge_sha = BTreeMap::new();
        for (receipt, sha) in chain {
            swarm_merge_sha.insert((*receipt).to_string(), (*sha).to_string());
        }
        Transition {
            source_pr: format!("{SOURCE}#657"),
            source_merge_sha: "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf".to_string(),
            consumed_by: String::new(),
            swarm_merge_sha,
            path: vec![TransitionPath {
                path: CARGO_LOCK.to_string(),
                disposition,
                swarm_chain: chain
                    .iter()
                    .map(|(receipt, _)| (*receipt).to_string())
                    .collect(),
            }],
        }
    }

    /// A resolved receipt reconciles a path both sides changed. It must not also
    /// grant one-sided source authority, or a settled migration would approve
    /// unrelated future source-only drift on the same path.
    #[test]
    fn resolved_receipt_grants_two_sided_only() -> Result<()> {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let swarm = format!("{SWARM}#269");
        let port = StubPort::new()
            .merged(
                &format!("{SOURCE}#657"),
                "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
                &patch,
            )
            .merged(&swarm, "ba4aeaf78cb17f980f1da05d24d9638033b95f68", &patch);
        let entries = vec![entry(
            TransitionDisposition::Equivalent,
            &[(swarm.as_str(), "ba4aeaf78cb17f980f1da05d24d9638033b95f68")],
        )];

        let authority = derive_authority(&port, Path::new("."), &refs(), &entries)?;

        assert!(authority.two_sided.contains_key(CARGO_LOCK));
        assert!(
            authority.awaiting_swarm.is_empty(),
            "a resolved receipt has nothing awaiting swarm: {:?}",
            authority.awaiting_swarm
        );
        Ok(())
    }

    /// The source merge a receipt accounts for must be reported, so the ancestry
    /// walk can step over it. Without this the path-level authority is
    /// unreachable: a source commit following the promotion merge is rejected as
    /// unapproved divergence before alignment is ever consulted.
    #[test]
    fn active_receipt_accounts_for_its_source_merge_commit() -> Result<()> {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let sha = "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf";
        let port = StubPort::new().merged(&format!("{SOURCE}#657"), sha, &patch);
        let mut entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];

        let active = derive_authority(&port, Path::new("."), &refs(), &entries)?;
        assert!(
            active.source_commits.contains(sha),
            "an active receipt must account for its source merge"
        );

        // A consumed receipt stops accounting for it, so the commit becomes
        // unapproved divergence again rather than staying permanently waved through.
        entries[0].consumed_by = format!("{SOURCE}#655");
        let consumed = derive_authority(&port, Path::new("."), &refs(), &entries)?;
        assert!(consumed.source_commits.is_empty());
        Ok(())
    }

    /// Evidence must gate the commit-level allowance too, not just the paths.
    #[test]
    fn unverified_receipt_accounts_for_nothing() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().open(&format!("{SOURCE}#657"), &patch);
        let entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];
        assert!(
            derive_authority(&port, Path::new("."), &refs(), &entries).is_err(),
            "an unmerged receipt must not account for its source commit"
        );
    }

    /// `missing_in_swarm` grants nothing. It records that swarm lacks the change
    /// so the refusal can name the receipt, and stops recording it once consumed.
    #[test]
    fn missing_in_swarm_records_but_never_grants() -> Result<()> {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().merged(
            &format!("{SOURCE}#657"),
            "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
            &patch,
        );
        let mut entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];

        let active = derive_authority(&port, Path::new("."), &refs(), &entries)?;
        assert_eq!(
            active.awaiting_swarm.get(CARGO_LOCK).map(String::as_str),
            Some(format!("{SOURCE}#657").as_str())
        );
        assert!(
            active.two_sided.is_empty(),
            "missing_in_swarm must not grant reconciliation authority"
        );

        // Once consumed the receipt is history and grants nothing, so the
        // migration mechanism cannot become a permanent bypass.
        entries[0].consumed_by = format!("{SOURCE}#655");
        let consumed = derive_authority(&port, Path::new("."), &refs(), &entries)?;
        assert!(
            consumed.awaiting_swarm.is_empty() && consumed.two_sided.is_empty(),
            "a consumed receipt still granted authority"
        );
        Ok(())
    }

    /// Recording a SHA is not evidence. An unmerged PR must be rejected even
    /// though the receipt is well formed.
    #[test]
    fn unmerged_source_pr_is_rejected() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().open(&format!("{SOURCE}#657"), &patch);
        let entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("an unmerged PR must not supply evidence");
        assert!(error.to_string().contains("source merge evidence"));
        assert!(
            format!("{error:#}").contains("not MERGED"),
            "unexpected error: {error:#}"
        );
    }

    /// A well-formed SHA that is not the PR's merge commit must be rejected.
    #[test]
    fn wrong_recorded_merge_sha_is_rejected() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().merged(
            &format!("{SOURCE}#657"),
            "1111111111111111111111111111111111111111",
            &patch,
        );
        let entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("a mismatched merge sha must be rejected");
        assert!(
            format!("{error:#}").contains("but the receipt records"),
            "unexpected error: {error:#}"
        );
    }

    /// The merge must still be part of the promoted line, not something later
    /// rewritten away, which only local ancestry can establish.
    #[test]
    fn unreachable_merge_sha_is_rejected() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let sha = "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf";
        let port = StubPort::new()
            .merged(&format!("{SOURCE}#657"), sha, &patch)
            .unreachable(sha);
        let entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("an unreachable merge must be rejected");
        assert!(
            format!("{error:#}").contains("not reachable from origin/main"),
            "unexpected error: {error:#}"
        );
    }

    /// Equivalence is decided by `git patch-id --stable` on the path's own
    /// section, so the same edit to a different file is a different identity.
    #[test]
    fn equivalent_requires_matching_patch_id_for_that_path() -> Result<()> {
        let source_patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let divergent = section(CARGO_LOCK, "1.52.3", "9.9.9", "tokio");
        let swarm = format!("{SWARM}#269");
        let swarm_sha = "ba4aeaf78cb17f980f1da05d24d9638033b95f68";
        let port = StubPort::new()
            .merged(
                &format!("{SOURCE}#657"),
                "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
                &source_patch,
            )
            .merged(&swarm, swarm_sha, &divergent);
        let entries = vec![entry(
            TransitionDisposition::Equivalent,
            &[(swarm.as_str(), swarm_sha)],
        )];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("differing patches must not be accepted as equivalent");
        assert!(
            format!("{error:#}").contains("patch ids differ"),
            "unexpected error: {error:#}"
        );

        // The same content under a different file name is a different change.
        let elsewhere = section("other/Cargo.lock", "1.52.3", "1.53.0", "tokio");
        assert_ne!(
            system_patch_id(&source_patch)?,
            system_patch_id(&elsewhere)?,
            "patch identity must include the file it applies to"
        );
        Ok(())
    }

    /// A receipt may only speak for paths its own source PR touched. Granting
    /// `missing_in_swarm` before checking that let one merged PR authorize
    /// divergence on an unrelated file it never changed.
    #[test]
    fn missing_in_swarm_requires_the_source_pr_to_touch_the_path() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().merged(
            &format!("{SOURCE}#657"),
            "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
            &patch,
        );
        let mut entries = vec![entry(TransitionDisposition::MissingInSwarm, &[])];
        entries[0].path[0].path = "some-unrelated-source-file".to_string();
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("a path the source PR never touched must not be authorized");
        assert!(
            format!("{error:#}").contains("does not touch this path"),
            "unexpected error: {error:#}"
        );
    }

    /// A receipt naming a different repository must not be answered from the
    /// expected repository's PR of the same number.
    #[test]
    fn receipt_repository_identity_is_enforced() {
        assert!(receipt_number(&format!("{SOURCE}#657"), SOURCE).is_ok());
        let error = receipt_number("OtherOrg/OtherRepo#657", SOURCE)
            .expect_err("a foreign repository receipt must be rejected");
        assert!(
            error.to_string().contains("names repository"),
            "unexpected error: {error}"
        );
    }

    /// Starting at the right version is not enough. A swarm step from the same
    /// old version to an unrelated new one advanced the walk while never
    /// incorporating what source landed.
    #[test]
    fn supersession_rejects_a_divergent_step_from_the_source_start() {
        let source_patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        // Same starting version, unrelated destination: contiguous, but not a
        // continuation of the source change.
        let divergent = section(CARGO_LOCK, "1.52.3", "9.9.9", "tokio");
        let swarm = format!("{SWARM}#269");
        let swarm_sha = "ba4aeaf78cb17f980f1da05d24d9638033b95f68";
        let port = StubPort::new()
            .merged(
                &format!("{SOURCE}#657"),
                "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
                &source_patch,
            )
            .merged(&swarm, swarm_sha, &divergent);
        let entries = vec![entry(
            TransitionDisposition::SupersededInSwarm,
            &[(swarm.as_str(), swarm_sha)],
        )];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("a divergent first step must not prove supersession");
        assert!(
            format!("{error:#}").contains("does not start by reproducing the source change"),
            "unexpected error: {error:#}"
        );
    }

    /// The Tokio chain the manifest actually needs: swarm reproduces the source
    /// bump, then continues past it.
    #[test]
    fn supersession_accepts_a_chain_that_reproduces_then_advances() -> Result<()> {
        let source_patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let reproduce = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let advance = section(CARGO_LOCK, "1.53.0", "1.53.1", "tokio");
        let first = format!("{SWARM}#269");
        let second = format!("{SWARM}#265");
        let first_sha = "ba4aeaf78cb17f980f1da05d24d9638033b95f68";
        let second_sha = "97239104cb2923e389749ac733755a955e4c6cc5";
        let port = StubPort::new()
            .merged(
                &format!("{SOURCE}#657"),
                "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
                &source_patch,
            )
            .merged(&first, first_sha, &reproduce)
            .merged(&second, second_sha, &advance);
        let entries = vec![entry(
            TransitionDisposition::SupersededInSwarm,
            &[(first.as_str(), first_sha), (second.as_str(), second_sha)],
        )];

        let authority = derive_authority(&port, Path::new("."), &refs(), &entries)?;
        assert!(authority.two_sided.contains_key(CARGO_LOCK));
        assert!(authority.awaiting_swarm.is_empty());
        Ok(())
    }

    /// `conflicting` blocks rather than granting anything.
    #[test]
    fn conflicting_disposition_blocks() {
        let patch = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let port = StubPort::new().merged(
            &format!("{SOURCE}#657"),
            "e6a72ad11ab22d03b1cf434b237bf0fff9c145bf",
            &patch,
        );
        let entries = vec![entry(TransitionDisposition::Conflicting, &[])];
        let error = derive_authority(&port, Path::new("."), &refs(), &entries)
            .expect_err("conflicting must block");
        assert!(
            format!("{error:#}").contains("resolve it before promoting"),
            "unexpected error: {error:#}"
        );
    }

    fn section(path: &str, from: &str, to: &str, package: &str) -> String {
        format!(
            "diff --git a/{path} b/{path}\nindex 111..222 100644\n--- a/{path}\n+++ b/{path}\n@@ -1,4 +1,4 @@\n [[package]]\n-name = \"{package}\"\n-version = \"{from}\"\n+name = \"{package}\"\n+version = \"{to}\"\n"
        )
    }

    #[test]
    fn patch_for_path_selects_only_the_requested_file() {
        let patch = format!(
            "{}{}",
            section("Cargo.lock", "1.0.0", "1.1.0", "tokio"),
            section("other/Cargo.lock", "9.0.0", "9.1.0", "tokio")
        );
        let selected = patch_for_path(&patch, CARGO_LOCK).expect("section");
        assert!(selected.contains("diff --git a/Cargo.lock b/Cargo.lock"));
        assert!(!selected.contains("other/Cargo.lock"));
        assert!(selected.contains("1.1.0"));
        assert!(!selected.contains("9.1.0"));
        assert!(patch_for_path(&patch, "missing.txt").is_none());
    }

    #[test]
    fn lock_transitions_reads_from_and_to_versions() {
        let parsed = lock_transitions(&section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio"));
        let (from, to) = parsed.get("tokio").expect("tokio");
        assert_eq!(from.as_deref(), Some("1.52.3"));
        assert_eq!(to.as_deref(), Some("1.53.0"));
    }

    #[test]
    fn keyed_value_rejects_malformed_input_without_panicking() {
        for malformed in ["name = \"", "name = \"\"", "name =", "name", "name = 1"] {
            assert_eq!(keyed_value(malformed, "name"), None, "{malformed:?}");
        }
        assert_eq!(
            keyed_value("name = \"tokio\"", "name").as_deref(),
            Some("tokio")
        );
    }

    /// The chain must start where the source change started. Mapping a source
    /// bump straight onto a later swarm bump skips the step that connects them.
    #[test]
    fn supersession_requires_a_contiguous_chain_from_the_source_start() {
        let source = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let later_only = vec![section(CARGO_LOCK, "1.53.0", "1.53.1", "tokio")];
        let error = check_supersession_chain(&source, &later_only, CARGO_LOCK)
            .expect_err("a chain skipping the connecting step must be rejected");
        assert!(
            error.to_string().contains("not contiguous"),
            "unexpected error: {error}"
        );

        let full_chain = vec![
            section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio"),
            section(CARGO_LOCK, "1.53.0", "1.53.1", "tokio"),
        ];
        check_supersession_chain(&source, &full_chain, CARGO_LOCK)
            .expect("the full chain should prove supersession");
    }

    /// An unrelated move that merely differs from source is not supersession.
    #[test]
    fn supersession_rejects_a_divergent_step() {
        let source = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let downgrade = vec![section(CARGO_LOCK, "1.40.0", "1.39.0", "tokio")];
        let error = check_supersession_chain(&source, &downgrade, CARGO_LOCK)
            .expect_err("a divergent step must be rejected");
        assert!(
            error.to_string().contains("not contiguous"),
            "unexpected error: {error}"
        );
    }

    /// Landing on the same version source landed is equivalence, and claiming
    /// supersession would earn authority the evidence does not support.
    #[test]
    fn supersession_rejects_a_chain_that_only_reaches_the_source_version() {
        let source = section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio");
        let same = vec![section(CARGO_LOCK, "1.52.3", "1.53.0", "tokio")];
        let error = check_supersession_chain(&source, &same, CARGO_LOCK)
            .expect_err("reaching only the source version is not supersession");
        assert!(
            error.to_string().contains("equivalent, not superseded"),
            "unexpected error: {error}"
        );
    }
}
