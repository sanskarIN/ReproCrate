# Development roadmap

Versions reflect **planned** scope, not released artifacts. Releases must be supported by tested commits and authentic downloadable assets.

| Milestone | Planned objective |
| --- | --- |
| v0.1.0 | **Shipped as pre-release:** working Rust command capture, bounded output, basic sanitization, JSON/Markdown reports, CI |
| v0.2.0 | **Shipped as pre-release:** bounded offline log analyzer, initial format detection (Cargo/pytest/Jest/Vitest), deterministic fingerprints, test fixtures. Process-tree cancellation remains future work. |
| v0.3.0 | **Shipped as pre-release:** local report review, explicit attachment approval, bounded ZIP creation/inspection/verification/safe extraction and hostile-input regression checks. Further security hardening remains planned. |
| v0.4.0 | **Shipped as pre-release:** structural report comparison with missing/redacted/truncated field states, inert manual recipe guides, cross-platform CLI tests. |
| v0.5.0 | **Shipped as pre-release:** Tauri 2/React desktop foundation, shared Rust-core IPC, report inspection, comparisons, bounded log analysis, inert recipes, privacy review, cross-platform CI and unsigned desktop developer-preview executables. No installers or stable claim. |
| v0.6.0 | **Shipped as pre-release:** opt-in private SQLite report history, bounded local search, inspection links, forgetting indexed entries, three-OS desktop tests and real unsigned executables. Accessibility/file-picker/security refinement remains planned. |
| v0.6.1 | **Shipped as pre-release:** Unix reviewed-export privacy, selected-file read checks, canonical history paths, SQLite future-schema refusal, regression tests and verified three-platform packages. |
| v0.7.0 | **Shipped as pre-release:** offline GitHub/GitLab/Gitea/generic issue Markdown adapters, opt-in reviewed metadata, local CLI/desktop exports and verified unsigned three-platform binaries. |
| v0.8.0 | **Shipped as pre-release:** offline download checksum verification, static website source/Pages workflow, locked dependency inventory, scheduled advisory scans, cross-platform distribution docs and verified CLI/desktop archives. **Pages first-time enablement requires repository Settings action.** |
| v0.9.0 | **Shipped as pre-release:** literal Markdown metadata, safer CLI JSON input checks, real CLI binary smoke tests, authenticated draft-aware 12-asset gate and tested cross-platform CLI/desktop binaries. **Manual clean-machine GUI/accessibility and signing remain pending.** |
| v0.9.1 | **Shipped as pre-release (October 24, 2026):** patched ZIP 9.0.5 in independently locked CLI/core and native desktop crates; version synchronization; clean dual RustSec scans and npm audit; corrected manifest-aware real executable smoke check; real CLI/desktop packages across three operating systems with all 12 SHA-256-paired uploaded assets. Clean-machine GUI/accessibility testing and signing remain pending. |

See [what_changed.md](what_changed.md) for actual progress. Do not confuse a GitHub pre-release with a production-ready build.
