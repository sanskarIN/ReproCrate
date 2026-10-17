# Architecture: initial CLI milestone

A Cargo workspace has two components: `reprocrate-core` defines versioned report models, redaction, environment inspection and process capture; `reprocrate-cli` handles argument parsing and local report export. No library code depends on Clap or Tauri.

Trust boundary: untrusted program and its output -> bounded byte channels -> lossy UTF-8 conversion -> automatic best-effort sanitization -> serialized report -> user review before manual sharing.

Export is not an automatic sharing action. Import does not execute commands. The future Tauri desktop app must call the same Rust core and must not expose arbitrary command execution to untrusted frontend content.

No confidential data is intentionally written to application logs. v0.1.0 does not persist raw capture archives. More detailed security testing is required prior to distributing binaries.
