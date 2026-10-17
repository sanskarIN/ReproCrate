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

## Unreleased — v0.6.0 development

- Added opt-in SQLite history in the desktop's private app-data folder; indexes explicitly selected JSON reports only.
- Stored limited metadata: report path, best-effort-sanitized title and ID, OS/architecture, indexing time. Raw output, arguments, attachments and report JSON are not stored.
- Added bounded case-insensitive search, refresh-on-reindex, "Inspect" navigation and "Forget index" deletion without touching original files.
- Added parameterized SQL and tests for indexing, duplicate paths, query limits, secret-like metadata, injection and deletion.
- Kept React/browser preview read-only and extended native Windows/Linux/macOS desktop checks.

**Unfinished:** native file-picker UX, secure erasure guarantees, encrypted database, full UI accessibility evaluation and stable v1.0.0. Not published until CI and release assets are verified.
