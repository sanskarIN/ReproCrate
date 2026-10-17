# Dependency review status

ReproCrate source is EPL-2.0, but dependencies preserve their own licenses. The original CLI/core Rust workspace relies on serde, serde_json, regex and clap. The **separate Tauri desktop workspace** also declares Tauri 2, Tauri build tools and, starting in v0.6.0, rusqlite with bundled SQLite. The React frontend depends on React, Vite, TypeScript and their resolved npm packages. Upstream license expressions and binary redistribution obligations must be checked for each resolved version; one project's EPL-2.0 license does not relicense its dependencies. This does **not** amount to auditing every transitive package, native artifact, feature flag or redistributed license notice.

To inventory every resolved dependency and its declared license, run:

```sh
cargo metadata --locked --format-version 1 > cargo-metadata.json
```

Inspect the `packages[*].license`, `license_file`, and `source` fields, check any multi-license expression, and prepare notices for redistribution if required. Cargo.lock pins the resolution but does not assert compatibility.

**Status for the first milestone:** no complete independently reviewed dependency-license report. Before a stable release, perform a complete license and security audit and document findings.

## v0.6.0 desktop-native and JavaScript inventory

Desktop Rust and npm have **separate lockfiles**. To generate complete, auditable inputs:

```sh
cargo metadata --manifest-path apps/desktop/src-tauri/Cargo.toml --locked --format-version 1 > desktop-cargo-metadata.json
cd apps/desktop
npm ls --all --json > dependency-tree.json
```

A complete audit must inspect Tauri's WebKitGTK/WebView2/system runtime requirements, bundled SQLite native sources, rusqlite, all transitive Rust crates, and every frontend npm package. License expressions, notices, provenance, vulnerability reports and code-signing implications must be reviewed **before stable distribution**. The GitHub pre-release currently contains unsigned CLI and desktop developer previews; passing build tests does not constitute a security or dependency-license certification.
