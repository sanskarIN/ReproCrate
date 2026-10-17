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

## Unreleased — v0.2.0 development

- Added bounded, offline `reprocrate analyze` for log files and standard input with optional Markdown export.
- Added best-effort Cargo/pytest/Jest/Vitest recognition and generic error/timeout/permission categorization.
- Added deterministic non-cryptographic FNV-1a message fingerprints and repeat-group counts.
- Added additive optional sanitized failure findings to existing JSON/Markdown capture reports.
- Added framework fixtures and CLI regression coverage for input limits, no-overwrite, and redaction.

**Not completed:** reliable process-tree cancellation, complete framework test-result parsers, desktop GUI, bundles, stable release. These are planned and not claimed.
