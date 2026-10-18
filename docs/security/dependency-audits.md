# Dependency inventory and advisory monitoring (v0.8.0)

This project contains two Rust dependency lockfiles and one frontend npm lockfile. The CLI/core workspace is separate from the Tauri desktop build. The reproducible lockfiles are:

- `Cargo.lock` — Rust CLI and core.
- `apps/desktop/src-tauri/Cargo.lock` — Tauri/native desktop Rust dependencies.
- `apps/desktop/package-lock.json` — browser/desktop JavaScript dependencies.

Run:

```sh
python3 -m unittest discover -s scripts/tests -p 'test_*.py'
python3 scripts/dependency_inventory.py --output dependency-inventory.json
```

This produces a deterministic JSON list with package names, versions, source locators and any lockfile-provided checksums. It includes npm license **declarations when present**, but Cargo.lock has no license fields; the inventory uses `null`, not a guessed license. Repeated crate versions between the two Rust lockfiles are distinct entries. This is **not a CycloneDX/SPDX attestation, vulnerability-free guarantee, software bill of materials certification or license clearance**.

GitHub Actions `Dependency inventory and advisory checks` tests and uploads the inventory on relevant changes. It also runs `cargo-audit` for each Rust lockfile and `npm audit` for desktop dependencies on a weekly schedule or manual trigger. Audit services require network access **within CI**; this does not add networking to the ReproCrate desktop or CLI. Tool installation, advisory database outages, dependency metadata gaps and false negatives are possible. Check failed actions and resolve flagged issues before any stable release.

Review licensing with a qualified maintainer: each dependency may have more than one permissible license, and `license: null` means no declaration in the lockfile, **not license-free**. Check original package manifests and required notice conditions. `LICENSE` in this repository covers ReproCrate's own code; third-party code has independent obligations.

Source code dependencies are controlled by committed lockfiles. Existing pinned GitHub Actions are maintained separately from packages. The v0.8.0 audit process provides evidence to review—not a claim of complete supply-chain safety.
