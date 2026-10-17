# Development roadmap

Versions reflect **planned** scope, not released artifacts. Releases must be supported by tested commits and authentic downloadable assets.

| Milestone | Planned objective |
| --- | --- |
| v0.1.0 | **Shipped as pre-release:** working Rust command capture, bounded output, basic sanitization, JSON/Markdown reports, CI |
| v0.2.0 | **Shipped as pre-release:** bounded offline log analyzer, initial format detection (Cargo/pytest/Jest/Vitest), deterministic fingerprints, test fixtures. Process-tree cancellation remains future work. |
| v0.3.0 | **Shipped as pre-release:** local report review, explicit attachment approval, bounded ZIP creation/inspection/verification/safe extraction and hostile-input regression checks. Further security hardening remains planned. |
| v0.4.0 | **Shipped as pre-release:** structural report comparison with missing/redacted/truncated field states, inert manual recipe guides, cross-platform CLI tests. |
| v0.5.0 | Shared-Rust-core Tauri desktop foundation |
| v0.6.0 | SQLite history, search, accessibility, desktop refinement |
| v0.7.0 | Issue export review and integration adapters |
| v0.8.0 | Cross-platform distribution, docs site, license/security audits |
| v0.9.0 | Release candidate and clean-machine testing |
| v1.0.0 | First stable release after quality, privacy and platform gates pass |

See [what_changed.md](what_changed.md) for actual progress. Do not confuse a GitHub pre-release with a production-ready build.
