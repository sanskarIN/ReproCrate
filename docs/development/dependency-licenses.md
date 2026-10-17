# Dependency review status

ReproCrate source is EPL-2.0, but dependencies preserve their own licenses. The current Rust workspace declares four core external dependency families: serde, serde_json, regex and clap; their upstream projects commonly offer MIT and/or Apache-2.0 terms. This does **not** amount to auditing every transitive package, native artifact, feature flag or redistributed license notice.

To inventory every resolved dependency and its declared license, run:

```sh
cargo metadata --locked --format-version 1 > cargo-metadata.json
```

Inspect the `packages[*].license`, `license_file`, and `source` fields, check any multi-license expression, and prepare notices for redistribution if required. Cargo.lock pins the resolution but does not assert compatibility.

**Status for the first milestone:** no complete independently reviewed dependency-license report. Before a stable release, perform a complete license and security audit and document findings.
