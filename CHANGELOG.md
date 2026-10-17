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

## Unreleased — toward 0.2.0

Future work: reliable cancellation, log parsers, deterministic fingerprints, privacy review and wider adversarial testing. These features are not yet shipped.
