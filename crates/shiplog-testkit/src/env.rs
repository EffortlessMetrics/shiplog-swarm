//! Ambient credential isolation for tests that drive the `shiplog` binary.
//!
//! Provider credentials are read from the environment, so a developer's own
//! shell can decide what a test observes. With `GH_TOKEN` exported, shiplog
//! prefers it over the `GITHUB_TOKEN` a test set, and the setup readiness
//! tests reported the wrong source. Tests that assert on a credential-free
//! environment have the mirror-image problem: they pass on CI and quietly stop
//! proving anything on a machine that has a token.
//!
//! Clear every credential up front, then let each test opt back in to exactly
//! the ones it is about.

/// Every environment variable shiplog reads a provider credential from.
///
/// Keep this in step with the lookups in `apps/shiplog/src/github_auth.rs` and
/// the source ingest adapters.
pub const AMBIENT_CREDENTIAL_ENV_VARS: &[&str] = &[
    "GH_TOKEN",
    "GITHUB_TOKEN",
    "GH_ENTERPRISE_TOKEN",
    "GITHUB_ENTERPRISE_TOKEN",
    "GITLAB_TOKEN",
    "JIRA_TOKEN",
    "LINEAR_API_KEY",
    "SHIPLOG_REDACT_KEY",
    "SHIPLOG_LLM_API_KEY",
];

/// Remove every ambient provider credential from `command`.
///
/// Call this while building the command. A later `.env(key, value)` still
/// wins, so a test that needs a specific credential sets it as usual.
pub fn clear_ambient_credentials(
    command: &mut std::process::Command,
) -> &mut std::process::Command {
    for key in AMBIENT_CREDENTIAL_ENV_VARS {
        command.env_remove(key);
    }
    command
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `GH_TOKEN` is the one that regressed: shiplog prefers it over
    /// `GITHUB_TOKEN`, so leaving it ambient silently overrode what tests set.
    #[test]
    fn covers_both_github_token_variables() {
        assert!(AMBIENT_CREDENTIAL_ENV_VARS.contains(&"GH_TOKEN"));
        assert!(AMBIENT_CREDENTIAL_ENV_VARS.contains(&"GITHUB_TOKEN"));
    }

    #[test]
    fn clearing_beats_an_exported_credential_and_yields_to_an_explicit_one() {
        let mut command = std::process::Command::new(env!("CARGO"));
        command.env("GH_TOKEN", "ambient");
        clear_ambient_credentials(&mut command);
        command.env("GITHUB_TOKEN", "chosen-by-the-test");

        let envs: Vec<_> = command.get_envs().collect();
        let gh_token = envs.iter().find(|(key, _)| *key == "GH_TOKEN");
        let github_token = envs.iter().find(|(key, _)| *key == "GITHUB_TOKEN");

        assert_eq!(
            gh_token.map(|(_, value)| *value),
            Some(None),
            "an ambient GH_TOKEN must be removed, not carried through"
        );
        assert_eq!(
            github_token.and_then(|(_, value)| *value),
            Some("chosen-by-the-test".as_ref()),
            "a credential set after clearing must survive"
        );
    }
}
