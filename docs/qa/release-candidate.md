# v0.9.0 release-candidate QA: automated evidence and manual gates

ReproCrate v0.9.0 is an **experimental pre-release aimed at release-candidate quality**, not a stable v1.0.0, signed installer or certified cross-platform product.

## Automated checks

| Gate | Platform | Meaning |
| --- | --- | --- |
| Rust CLI/core build, format, Clippy, tests | Windows 2022, Ubuntu 24.04, macOS 14 | Library and CLI compile, static lint checks and automated tests |
| Native desktop IPC tests + actual executable build | Same three hosted runner OSs | UI source builds and native command boundary tests pass |
| Headless CLI smoke against final executable | Same three runners in v0.9.0 release workflow | Real CLI reports version/doctor data, analyzes inert logs, checks SHA-256 fixture, rejects corrupted archive and refused overwrite |
| Platform archive creation and checksum self-check | Same three OS runners | Real native CLI archive agrees with generated SHA-256 sidecar |
| Pre-publication manifest inspection | GitHub Releases API | Exactly 12 assets (6 archives and 6 checksum sidecars), expected names, successful upload states, plausible nonzero sizes, draft/prerelease status |
| Dependency inventory tests | Ubuntu | Lockfile manifest consistency, not a complete vulnerability or license clearance |

Tests are useful release gates but are **not** a substitute for Windows/macOS/Linux manual GUI and system integration validation. Release metadata checks do not authenticate authors or prove that downloaded binaries are safe.

## Manual clean-machine checks — not marked complete

A maintainer must perform these on systems **separate from the build runners** before a stable v1.0.0 declaration:

- [ ] Windows x86-64 machine: clean installation/user profile, WebView2 prerequisite, launch native desktop ZIP binary, tab through navigation, inspect an actual local report, check exports and privacy permissions.
- [ ] Linux x86-64 machine: document GTK/WebKitGTK dependencies, verify native desktop launch and cleanup, verify no network access is required for ordinary local operations.
- [ ] macOS ARM64 machine: check unsigned-binary OS warning, safe launch policy, app window behavior, file read/write permissions and normal shutdown.
- [ ] Windows/Linux/macOS CLI: execute the published binary and verify `--version`, `doctor --json`, and checksum verification with independently obtained .sha256 sidecar.
- [ ] Manual secret review: check redaction on benign representative fixtures and intentionally malformed inputs; ensure no export unintentionally posts data.
- [ ] Manual rendering review: use crafted report metadata containing Markdown/HTML markup and verify no active remote images/links or script markup appear.
- [ ] Accessibility review: keyboard-only navigation, screen reader labels, high-contrast settings, scaling, focus order and error announcements on supported OSs.
- [ ] Security/supply-chain review: verify dependency licenses and notice obligations, review RustSec/npm audit findings and report import/symlink limitations, document residual risk.
- [ ] Distribution review: record supported OS versions, code signing/notarization/installer plan, archive provenance, SHA-256 trust boundary and rollback procedure.

Do not check boxes without actually completing the test and recording date, OS version, device/runtime information, observed result and issue references. Use a new issue for each unresolved blocker. This checklist is *a plan*, not proof that tests have been performed.

## Files and links

- [Release downloads and limitations](../reference/distribution.md)
- [Checksum integrity](../reference/download-integrity.md)
- [Dependency audits](../security/dependency-audits.md)
- [Security policy](../../SECURITY.md)
- [Current handoff](../../what_changed.md)
