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

## Unreleased — towards v0.4.0

Planned: report/environment comparisons and versioned inert reproduction recipes, with continued archive/privacy hardening.
