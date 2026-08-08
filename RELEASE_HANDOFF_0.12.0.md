# shiplog 0.12.0 — Release Execution Handoff

**Release target:** `v0.12.0`
**Status:** preparing
**Release decision:** [`docs/release/0.12.0-release-decision.md`](docs/release/0.12.0-release-decision.md)
**Readiness ledger:** [`docs/release/0.12.0-readiness.md`](docs/release/0.12.0-readiness.md)

Nothing here has been executed. Every identity and receipt is a placeholder,
and this handoff must not be marked closed from intended commands — only from
observed public state.

## Exact release identities

| Identity | Value |
| --- | --- |
| Swarm candidate SHA | `<not frozen>` |
| Source promotion PR / merge | `#<number>` / `<full sha>` |
| Source release-prep PR / merge | `#<number>` / `<full sha>` |
| Release source commit | `<full sha>` |
| Annotated tag | `v0.12.0` |
| Source Release workflow | `<run URL or ID>` |
| Staged candidate manifest | `<artifact/digest>` |
| Terminal candidate-ready aggregate | `<job/result>` |

## Published contents

`<Not published. Write this in user language after publication and link to the
CHANGELOG 0.12.0 section. Intended scope is in the release decision; keep
control-plane receipts in the readiness ledger rather than turning them into
the release story.>`

## Artifact receipt

| Artifact | Observed result |
| --- | --- |
| `shiplog-x86_64-unknown-linux-gnu` | `<asset/checksum/artifact digest>` |
| `shiplog-x86_64-apple-darwin` | `<asset/checksum/artifact digest>` |
| `shiplog-aarch64-apple-darwin` | `<asset/checksum/artifact digest>` |
| `shiplog-x86_64-pc-windows-msvc.exe` | `<asset/checksum/artifact digest>` |
| `SHA256SUMS.txt` | `<asset/checksum/artifact digest>` |

## Validation receipt

- Release preflight: `<result>`
- Release validation: `<result>`
- Four-platform first-use acceptance against one staged set: `<results>`
- Release-mode integration tests: `<result>`
- Terminal `Release Candidate Ready`: `<result>`
- crates.io install and `--version`: `<result>`
- GitHub release public-state verification: `<result>`
- Versionless installers: `<result>`
- Homebrew: `<PR/version/native validation>`
- Scoop: `<PR/version/native validation>`
- Signing/notarization: `not configured`

## Execution record

`<No commands executed. Record only what actually ran, with exact commit and
tag context.>`

The intended final boundary, for reference only:

```text
git tag -a v0.12.0 -m "shiplog v0.12.0"
git push origin v0.12.0
# After exact-tag staged-candidate proof and explicit owner approval:
git checkout --detach v0.12.0
cargo publish -p shiplog --locked
gh release edit v0.12.0 --draft=false --latest
```

`cargo publish` must run from the exact detached tag checkout proven by the
source Release workflow, never from a later moving `main`.

## Exceptions, rollback, or yank

None. No tag has been pushed and nothing has been published.

Rollback rules if that changes: the tag is immutable and published crate bytes
are never replaced. A failed candidate is fixed through swarm, promoted again,
and released as the next patch version — `v0.12.0` is not moved or reused.

## Residual work

- #245 — publication gated on one exact staged four-platform candidate set,
  four-platform first-use acceptance, negative controls, and a terminal
  readiness aggregate. The source-owned writer projection has not landed.
- Epic #246 Phase 5 — draft #389, the living release-preparation procedure,
  is not yet reviewed and landed.
- Remove the "not in the shipped `v0.11.0` binary" caveat for `shiplog start`
  from `README.md` and `docs/guides/guided-setup-doctor.md` as part of the
  release-preparation PR, once `start` actually ships.
- Audit `scripts/release-install-smoke.sh` and `.ps1` for the ambient
  credential isolation that #407 fixed in the Rust suites.
- Provider connection flows and share setup ergonomics remain deferred per the
  release decision.

## Claim boundary

In its present form this handoff proves nothing about a release. It records
intended execution and the receipts still owed.

When completed it will prove only what was observed. Checksums are not a
signing claim. A skipped or unavailable optional check is not executed proof. A
public GitHub Release does not by itself prove crates.io publication, Homebrew,
or Scoop. Preparation on `shiplog-swarm` confers no authority to tag, publish,
sign, or mutate a package channel; that authority is source-owned and requires
explicit owner approval.
