# Architecture: shared Rust core, CLI and native desktop

A Cargo workspace has two components: `reprocrate-core` defines versioned report models, redaction, environment inspection and process capture; `reprocrate-cli` handles argument parsing and local report export. No library code depends on Clap or Tauri.

Trust boundary: untrusted program and its output -> bounded byte channels -> lossy UTF-8 conversion -> automatic best-effort sanitization -> serialized report -> user review before manual sharing.

Export is not an automatic sharing action. Import does not execute commands. The v0.5.0 Tauri 2 desktop calls the same Rust core via explicit, bounded IPC commands. There is no imported-command execution API, no third-party shell/filesystem plugin permission and no automatic network upload. The frontend is React/Vite and only accesses locally entered report/log file paths through the native bridge.

No confidential data is intentionally written to application logs. v0.1.0 does not persist raw capture archives. More detailed security testing is required prior to distributing binaries.


## v0.5.0 native desktop isolation

The desktop has its own Cargo workspace in `apps/desktop/src-tauri` and its own lockfile; root CLI builds remain light and do not acquire WebKitGTK dependencies. Commands are defined in `apps/desktop/src-tauri/src/lib.rs` and dispatched via Tauri's allowlisted IPC handler. The Rust backend checks file types and limits before parsing, uses the shared versioned report models, never evaluates loaded recipe steps, and writes reviewed candidates only at newly created filenames.

A Tauri window and frontend code are not a sandbox for user-launched commands. The app deliberately does not expose command execution from the desktop UI in this milestone. Host-process and attachment risks described elsewhere in the security policy still apply. Native code signing and graphical automation require later milestones.

## v0.6.0 opt-in history service

A separate `rusqlite`-backed module in the Tauri-only crate maintains selected report metadata. The shared core and CLI **do not depend on SQLite**. A narrow desktop IPC boundary provides remember/search/forget methods, and the UI uses the existing read-only report inspector for opening saved entries. SQL queries are parameterized and both row count and query length are bounded. Paths and identifiers remain potentially sensitive; no automatic indexing, execution or cloud syncing occurs.
