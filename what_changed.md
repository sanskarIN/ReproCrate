# ReproCrate development continuity record

Last verified: October 17, 2026 (Asia/Kolkata).

## Repository identity

- **Actual repository:** https://github.com/sanskarIN/ReproCrate
- `https://github.com/sanskarIN/RepoCrate` was not found. The uploaded master specification and the writable repository use **ReproCrate**.
- Default branch: `main`
- Project source license: **EPL-2.0**; the repository's existing official `LICENSE` was preserved.
- Confirmed Git author/committer on newly created commits: **Sanskar <sanskarin@outlook.in>**.

## Current shipped milestone: v0.1.0 pre-release

**Verified published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.1.0

- GitHub API confirmed `tag_name: v0.1.0`, `prerelease: true`, `draft: false`, published October 17, 2026, 12:56:58 UTC.
- Six uploaded assets (three real archives and three SHA-256 files):
  - `reprocrate-v0.1.0-linux-x86_64.tar.gz`
  - `reprocrate-v0.1.0-linux-x86_64.tar.gz.sha256`
  - `reprocrate-v0.1.0-macos-arm64.tar.gz`
  - `reprocrate-v0.1.0-macos-arm64.tar.gz.sha256`
  - `reprocrate-v0.1.0-windows-x86_64.zip`
  - `reprocrate-v0.1.0-windows-x86_64.zip.sha256`
- Release tag target: `d187c85f1c3ee47a9fef4535663c2c850fb1f54d`. Subsequent commits on main only repaired release automation and updated documentation; avoid implying tag tracks latest main.
- This is an **experimental CLI pre-release**, not stable v1.0.0. No Tauri desktop executable or installer is included.

## Implemented source

- `Cargo.toml`, `Cargo.lock`: versioned Rust 2021 Cargo workspace with reproducible dependency resolution.
- `crates/reprocrate-core/src/lib.rs`: shared library entry points.
- `environment.rs`: minimal and standard non-identifying OS/architecture/parallelism capture profiles.
- `capture.rs`: explicit program execution without shell; bounded separate stdout/stderr, configurable timeout, direct-child reaping, truncation, launch errors, exit state and lossy UTF-8 conversion.
- `redaction.rs`: conservative best-effort credential, bearer-token, private-key and sensitive argument masking.
- `report.rs`: schema 1.0.0, per-report IDs, sanitization at the common report boundary, JSON serialization and HTML-escaped Markdown export.
- `crates/reprocrate-cli/src/main.rs`: functional help/version/doctor/capture/inspect with output overwrite prevention and bounded inert JSON imports.
- `crates/reprocrate-cli/tests/cli.rs`: independent command-launch, timeout, non-UTF-8, secret redaction and no-overwrite regression tests.
- `.github/workflows/rust-ci.yml`: Rust build/test/Clippy/rustfmt on Ubuntu 24.04, Windows 2022 and macOS 14.
- `.github/workflows/prerelease-v0.1.0.yml`: version-specific guarded release workflow, platform packaging, checksums and GitHub Release publication.
- `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `docs/architecture/overview.md`, `docs/reference/cli.md`, `docs/security/threat-model.md`, `docs/development/dependency-licenses.md`, `docs/releases/v0.1.0.md`.

## Actual verification

- Initial compiler was absent from the local execution container, and direct Rust installation was unavailable there. All Rust compiler/test results below originate from actual **GitHub-hosted runners**, not inferred local runs.
- Rust CI builds, tests, Clippy with `-D warnings`, and `cargo fmt --all --check` passed on all 3 target runner platforms. Verified workflow: https://github.com/sanskarIN/ReproCrate/actions/runs/39034021500 (head `90ba9a07`), and again https://github.com/sanskarIN/ReproCrate/actions/runs/39033911011 (timeout/redaction test commit `60c20126`).
- The prerelease validation job and all three package build/upload jobs succeeded in https://github.com/sanskarIN/ReproCrate/actions/runs/39034318864.
- First packaging run failed on macOS because `sha256sum` is not installed. Added `shasum -a 256` fallback. Next all 3 platform uploads passed, but the final publication job initially failed because the GitHub CLI ran outside a repository checkout. Fixed GH_REPO handling; a one-time recovery workflow verified the six assets and published the release: https://github.com/sanskarIN/ReproCrate/actions/runs/39034551126 (success).
- The GitHub Releases API independently confirmed the published release and its real asset sizes. No unbuilt placeholders were attached.
- Found and fixed a duplicate `env:` key in the release workflow after an unsuccessful workflow-file validation attempt. Temporary release-recovery workflow and its one-time trigger marker were removed after successful publication.
- The initial release development PR was merged with individual commits preserved: https://github.com/sanskarIN/ReproCrate/pull/1.

## Security boundaries / remaining gaps

- Running a selected command is **not sandboxed**; child processes may alter the host and their descendants may survive a timeout.
- Secret detection is best-effort and may miss or over-mask content; user must review full exports.
- Text conversion is lossy for invalid UTF-8; very large outputs are truncated.
- Report import is bounded and inert, but archives/bundles and advanced report verification are not implemented.
- Fully atomic export writes and complete dependency/license/security audits remain incomplete.
- Desktop GUI, SQLite history, comparisons, reproduction recipes, comprehensive log parsers, GitHub Pages docs, cross-platform desktop installers, code signing/notarization and stable release are **not implemented**.
- No v1.0.0 release is approved, built or represented as available.

## Version roadmap and next tasks

1. **v0.2.0:** Implement a bounded generic log analyzer and initial Cargo/pytest/Jest/Vitest adapters with tested deterministic fingerprints.
2. Add explicit export-review/redaction workflow, more timeout and large-output adversarial tests, and reliable cancellation.
3. Introduce environment/report comparison and safe bundle design only after threat-model review.
4. Begin a Tauri 2 + React desktop app reusing `reprocrate-core`, followed by optional SQLite history.
5. Complete dependency and license audit, secure packaging, cross-platform smoke testing, docs deployment and all stable gates before considering **v1.0.0**.

**Handoff principle:** All actual work and release evidence is in repository history, workflow runs, PR #1 and the verified release. Future phases must not claim completion of planned functionality.
