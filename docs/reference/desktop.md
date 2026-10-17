# ReproCrate Desktop — v0.5.0 foundation

The first desktop frontend is a **local-first Tauri 2 + React/TypeScript application** backed by the existing open-source Rust \`reprocrate-core\`. It is an experimental developer preview, not a production-ready installer or stable release.

## Implemented interactions

- **Workspace** — local activity navigation and connection status. The browser preview has no native access.
- **Inspect report** — a file path selected manually by the user is read as a bounded, regular JSON file. The UI shows version, OS/architecture, exit information and warning counts; it does not run the saved command.
- **Compare reports** — reads two local JSON reports and uses the v0.4.0 Rust comparator. Changed field **values and raw logs are omitted** from comparison output; unknown or truncated observations remain inconclusive.
- **Analyze logs** — scans at most 1 MiB of a selected log with the existing bounded Rust analysis module; recognized failures are heuristic and best-effort redacted. Large logs show truncation.
- **Manual recipes** — displays the shared five-step troubleshooting checklist; command references are omitted by default, and the UI does not execute recipes.
- **Privacy review** — exclude stdout, stderr, arguments, executable and/or findings, apply manual literal masks, inspect a candidate and save a **new** JSON file with no overwrite. The preview itself may show confidential information. No one should share the file without independently reviewing its entirety.

## Structure

| Directory | Purpose |
| --- | --- |
| \`apps/desktop/src\` | React/TypeScript UI, IPC wrapper, tests, CSS |
| \`apps/desktop/src-tauri/src/lib.rs\` | Narrow Tauri IPC commands and bounded file handling |
| \`apps/desktop/src-tauri/capabilities/main.json\` | Core Tauri permissions only; no filesystem or shell plugins |
| \`crates/reprocrate-core\` | Shared Rust report, analysis, comparison, recipe and review functionality |

The desktop is a **separate Cargo workspace** to preserve the previous CLI's small dependency graph and independent \`Cargo.lock\`. Root \`cargo test --workspace\` continues to test the CLI and core without WebKitGTK or GUI dependencies.

## Development prerequisites

Install Node.js 22, npm and an up-to-date Rust toolchain. Follow [official Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/) for your operating system. Linux requires WebKitGTK 4.1 development files and other GTK/indicator libraries; macOS requires platform developer tools; Windows uses WebView2 and C++ build tools.

\`\`\`sh
cd apps/desktop
npm ci
npm test
npm run build
npm run desktop:dev
\`\`\`

To check the Rust IPC/backend without launching a desktop window:

\`\`\`sh
cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml --all-targets --locked
\`\`\`

The browser-only \`npm run dev\` preview is intentionally **read-only**; invoking a desktop command in a normal browser yields a descriptive error.

Native builds can be run from \`apps/desktop\` with \`npm run desktop:build\` (Tauri \`--no-bundle\`). The app has generated native icons. Native packaging and signing/distribution **must be separately verified** and should not be assumed from passing web build tests.

## Security boundaries

1. No shell execution, networking, external AI, auto-upload or telemetry is installed by the desktop IPC layer.
2. Local files require an explicit user-supplied path and must be regular files, not symlinks, directories or FIFO pipes.
3. Imported report JSON has a 4 MiB cap; log preview has a 1 MiB cap; reviews enforce up to 100 literal masks, 1024 bytes per item and 64 KiB total.
4. The existing Rust core performs best-effort redaction. Logs, warnings, titles and output may still contain unrecognized confidential information.
5. The UI deliberately does not expose full unsafe command execution from saved reports, automatic recipes, unrestricted file browsers or third-party filesystem/shell plugins.
6. Tauri CSP is restricted to local content and its IPC bridge; capabilities are scoped to the main window and \`core:default\` only.
7. File-size checks and new-file creation are protective but **not a substitute for a full filesystem race-condition audit**. Path entry by hand is a limitation of this first foundation.

## Remaining desktop work

Native OS smoke-testing in real graphical sessions; file-picker UX with precisely scoped grants; incremental large-log reading and cancellation; local history/search; workflow persistence; keyboard and screen-reader testing; signed installers, distribution and update policies; end-user documentation; network/permission review; and sandboxing for any future command execution. None of these features is claimed to be complete in v0.5.0.
