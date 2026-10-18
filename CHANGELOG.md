# Changelog

## [0.1.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.1.0

- Initial EPL-2.0-licensed Rust workspace, reusable core and functional CLI.
- Explicit process capture without an implicit shell, bounded stdout/stderr, direct child timeout/exit metadata and error records.
- Minimal/standard environment profiles, best-effort token and credential masking, versioned local JSON and Markdown reports.
- `reprocrate doctor`, `reprocrate capture`, `reprocrate inspect`, `--help`, `--version`.
- Integration tests including process failure, non-UTF-8 output, timeout, sanitization and destination overwrite rejection.
- Three platform-verified portable CLI archives (Windows x86-64, Linux x86-64, macOS ARM64) and individual SHA-256 checksums.
- CI succeeded on Ubuntu 24.04, Windows 2022 and macOS 14 (build, tests, Clippy and rustfmt). Release packaging retried after fixing missing macOS `sha256sum` and an unscoped GitHub CLI publishing command. Finalization run succeeded.

**Security:** Commands are not sandboxed and best-effort redaction can miss secrets; inspect exports before sharing.

**Not included:** desktop GUI, portable diagnostic bundles, comparison engine, rich framework parsers, signed installers or stable quality claim.

## [0.2.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.2.0

**Validated:** native CLI build and tests on Ubuntu 24.04, Windows 2022, macOS 14; release workflow https://github.com/sanskarIN/ReproCrate/actions/runs/39037161008 (all jobs passed). Verified portable archives for Linux x86-64, Windows x86-64 and macOS ARM64, with individual SHA-256 files.

- Added bounded, offline `reprocrate analyze` for log files and standard input with optional Markdown export.
- Added best-effort Cargo/pytest/Jest/Vitest recognition and generic error/timeout/permission categorization.
- Added deterministic non-cryptographic FNV-1a message fingerprints and repeat-group counts.
- Added additive optional sanitized failure findings to existing JSON/Markdown capture reports.
- Added framework fixtures and CLI regression coverage for input limits, no-overwrite, and redaction.

**Not completed:** reliable process-tree cancellation, complete framework test-result parsers, desktop GUI, bundles or stable v1.0.0. These are planned and not claimed.

## [0.3.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.3.0

**Verification:** https://github.com/sanskarIN/ReproCrate/actions/runs/39038674027 (all jobs succeeded, including native Windows/Linux/macOS package builds). GitHub API confirms three uploaded CLI archives and accompanying SHA-256 files.

- Added `reprocrate review` with additional literal masking from a bounded private file, whole-report preview and explicit field exclusions.
- Added `reprocrate bundle create|inspect|verify|extract` for versioned, bounded, SHA-256-manifest ZIP archives.
- Implemented verification-before-extraction, fixed archive paths, entry-count and size limits, malicious entry rejection, and manual approval for attachments.
- Added security documentation and cross-platform tests, retaining EPL-2.0 and local-only operation.

**Remaining:** further fuzzing, fuller privacy coverage, end-to-end authentication guidance and stable-release hardening.

## [0.4.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.4.0

**Verified:** GitHub Actions https://github.com/sanskarIN/ReproCrate/actions/runs/39040339254 (validation, platform build/upload and publication all passed); GitHub Releases API confirmed public pre-release and six authentic CLI archive/checksum assets.

- Added `reprocrate compare` with deterministic structural field differences and explicit unknown/redacted/truncated states, without exporting raw log or argument values.
- Added heuristic fingerprint comparison only when *both* reports have complete log analysis.
- Added `reprocrate recipe create` and `recipe inspect`, versioned inert five-step manual guides and bounded validation.
- Command references are excluded by default; including one requires both `--include-command` and `--acknowledge-reviewed`.
- Added shared Rust-core APIs and offline CLI integration tests. Report schema 1.0.0 remains compatible.

**Not included:** automated reproduction, desktop GUI, or a stable v1.0.0 promise. Further privacy/ZIP hardening remains planned.

## [0.5.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.5.0

**Validation:** https://github.com/sanskarIN/ReproCrate/actions/runs/39043664762 (all release jobs passed). GitHub Releases API confirmed `draft: false`, `prerelease: true`, and six real unsigned CLI + desktop executable archives (Windows/Linux/macOS) with six accompanying SHA-256 checksum files.

- Added `apps/desktop`: React 19 + TypeScript + Vite 6 frontend and a separate Tauri 2 Rust workspace that reuses the existing ReproCrate Rust core. The existing CLI remains independently testable.
- Created a dark responsive workspace UI for report inspection, comparisons, log analysis, manual recipe previews and privacy-review candidate exports.
- Added explicit native IPC commands with 4 MiB report and 1 MiB log input limits, regular-file-only import, no shell/filesystem plugin grants and create-new-only reviewed JSON saving.
- Generated native icons from a project SVG and committed separate lockfiles for npm and the desktop's Rust dependencies.
- Added frontend unit/production build checks and GitHub Actions cross-platform native Rust verification.
- Verified actual native unsigned desktop executables via https://github.com/sanskarIN/ReproCrate/actions/runs/39043004232 and uploaded real v0.5.0 CLI + desktop packages in a guarded release workflow.

**Remaining:** signed installers/notarization, clean-machine graphical smoke tests, file picker UX, history/search, automatic update and stronger filesystem sandboxing. The published desktop artifacts are real compiled developer-preview executables but not installers or stable v1.0.0.

## [0.6.0] — 2026-10-17 (pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.6.0

**Verified:** https://github.com/sanskarIN/ReproCrate/actions/runs/39046224080 (all validation, six CLI/desktop build/upload and publish jobs passed). GitHub Releases API independently confirmed the public pre-release and 12 real binary/checksum files.

- Added opt-in SQLite history in the desktop's private app-data folder; indexes explicitly selected JSON reports only.
- Stored limited metadata: report path, best-effort-sanitized title and ID, OS/architecture, indexing time. Raw output, arguments, attachments and report JSON are not stored.
- Added bounded case-insensitive search, refresh-on-reindex, "Inspect" navigation and "Forget index" deletion without touching original files.
- Added parameterized SQL and tests for indexing, duplicate paths, query limits, secret-like metadata, injection and deletion.
- Kept React/browser preview read-only and extended native Windows/Linux/macOS desktop checks.

**Unfinished:** native file-picker UX, secure erasure guarantees, encrypted database, full UI accessibility evaluation and stable v1.0.0. Native desktop files are unsigned developer previews, not installers.

## [0.6.1] — 2026-10-18 (published pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.6.1. **Verified:** https://github.com/sanskarIN/ReproCrate/actions/runs/39085609817 — all validation, draft, six build-upload jobs and publish passed. GitHub Releases API confirmed 12 authentic executable/checksum assets.

- Fixed reviewed JSON export file permissions on Unix by explicitly creating new files with owner-only mode (0600), retaining no-overwrite behavior.
- Verified file handle type and, on Unix, original-vs-opened device/inode identity to detect common path-swap races during bounded import.
- Refused to downgrade unrecognized future SQLite history schema versions; added preservation regression coverage.
- Canonicalized manually selected report paths before indexing, so history persists stable absolute paths.
- Added targeted native regression tests, version-consistent CLI/desktop metadata and gated three-platform pre-release workflow.

**Remaining:** native GUI smoke tests, Windows ACL review, comprehensive race audit, database migrations/encryption, code signing, and installer distribution. No automatic upload or recipe execution.

## [0.8.0] — 2026-10-18 (published pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.8.0. Full gate succeeded: https://github.com/sanskarIN/ReproCrate/actions/runs/39095891087. Verified 12 authentic nonempty CLI/desktop archive and checksum assets through the GitHub Releases API. The static site is committed, but first-time GitHub Pages enablement was blocked by missing workflow-token administration rights.

- Added strict offline SHA-256 release archive verifier with streaming reads, size caps, filename matching and negative-path tests, exposed through `reprocrate verify-download`.
- Added a self-contained accessible static documentation site and a separate GitHub Pages deployment workflow; deployment availability requires confirmation.
- Added deterministic combined dependency inventory for both Rust lockfiles and the desktop npm lockfile, with CI artifact and regression tests.
- Added separate scheduled/manual RustSec and npm advisory scans; these do not guarantee full vulnerability or licensing compliance.
- Added distribution matrix and release verification documentation, synchronized source versions and lockfiles, plus guarded cross-platform prerelease workflow.
- No new automatic network access, third-party runtime scripts, unsigned installer claims or background telemetry.

## [0.7.0] — 2026-10-18 (published pre-release)

**Published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.7.0. Verified successful gate: https://github.com/sanskarIN/ReproCrate/actions/runs/39091493244. GitHub Releases API confirms `draft:false`, `prerelease:true` and 12 genuine CLI/desktop archive/checksum assets.

- Added privacy-first, no-network issue draft rendering for GitHub, GitLab, Gitea and generic Markdown to the shared Rust core.
- Added `reprocrate issue export` with explicit report input, adapter choice, new-only Markdown output, and consent-gated optional metadata.
- Added a native desktop issue draft page with complete preview and local private export; browser demo remains read-only.
- Kept report title, logs, command lines, arguments, paths and raw finding messages out of issue drafts by default; categories are capped and opt-in.
- Added Rust core security regressions and end-to-end CLI tests; all work is kept local and uses existing import size/schema limits.
- Aligned CLI, desktop and reproducible lockfile versions; prepared gated three-OS prerelease pipeline.

**Remaining limitations:** no remote API integration, automatic posting, complete secret scanning, signed installers or clean-machine GUI validation. Manual review remains mandatory before sharing anything.

## Unreleased — toward v0.8.0

Next: packaging/distribution, docs site, dependency licenses/security audits, native file picker and broader GUI QA.

Plan: secure, explicit issue export previews and integration adapters, improved native file selection/accessibility, comprehensive privacy and filesystem tests.
