# Latest handoff — v0.4.0 pre-release published

Date: October 17, 2026. Repository: https://github.com/sanskarIN/ReproCrate.
Source development branch: `feat/v0.4.0-compare-recipes`. Merged pull request: https://github.com/sanskarIN/ReproCrate/pull/4.
**Current published pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.4.0. GitHub API confirmed `draft:false`, `prerelease:true`, `published_at:2026-10-17T14:23:19Z`, tag target `18456cafb3a758771a52784350bcd1fd069fad17`.

## Implemented v0.4.0 source

- `crates/reprocrate-core/src/compare.rs`: privacy-minimizing structural comparison, explicit field relationship and observation-state enums, bounded heuristic fingerprint differences only where both analyses are complete. No raw log or argument values in exported diffs.
- `crates/reprocrate-core/src/recipe.rs`: schema-versioned, five-step inert human review/checklist recipes; command references omitted by default; validation explicitly rejects automatic-execution flag. Recipe Markdown uses HTML escaping and does not generate executable shell scripts.
- `crates/reprocrate-cli/src/compare_recipe_commands.rs`: offline `compare`, `recipe create`, `recipe inspect`; independent 4 MiB report import limits and 1 MiB recipe limit, no-overwrite output; command references require explicit review acknowledgement.
- `crates/reprocrate-cli/src/main.rs`, `crates/reprocrate-core/src/lib.rs`: register commands and core APIs.
- `crates/reprocrate-cli/tests/compare_recipes.rs`: integration checks for privacy-preserving diffs, incomplete data, opt-in command disclosure, recipe import non-execution and overwrite prevention.
- `Cargo.toml`, `Cargo.lock`: workspace v0.4.0. No new third-party dependencies introduced for this milestone.
- `docs/reference/report-comparison.md`, `docs/reference/reproduction-recipes.md`, `docs/releases/v0.4.0.md`: documents supported CLI behavior and conservative claims.

## Validation / release gate

- **Passed CI:** https://github.com/sanskarIN/ReproCrate/actions/runs/39040215862. Ubuntu 24.04, macOS 14 and Windows 2022 all passed workspace compilation, tests, Clippy with `-D warnings`, and rustfmt.
- Fixed an early Clippy unreachable-pattern error and an intermittent macOS parallel scratch-directory test collision; normalized Rust source via a temporary formatter workflow then removed it.
- **PR #4 merged:** `2d592a116851a05c241edc89ce2a7e194d05c309` with individual feature commits retained.
- **Release workflow successful:** https://github.com/sanskarIN/ReproCrate/actions/runs/39040339254 (validate, draft, Linux/macOS/Windows build-upload, and publish).
- GitHub Releases API independently confirmed six uploaded real assets:
  - `reprocrate-v0.4.0-linux-x86_64.tar.gz` (1,616,775 bytes) + `.sha256` (104 bytes)
  - `reprocrate-v0.4.0-macos-arm64.tar.gz` (1,432,261 bytes) + `.sha256` (103 bytes)
  - `reprocrate-v0.4.0-windows-x86_64.zip` (1,401,405 bytes) + `.sha256` (104 bytes)
- The v0.1.0, v0.2.0, v0.3.0 pre-releases remain available; none is stable v1.0.0.

## Known limitations and next steps

1. **v0.5.0:** develop shared-Rust-core Tauri 2 + React desktop app foundation, add cross-platform GUI build and smoke tests.
2. Continue privacy/redaction improvements, hostile bundle fuzzing, file IO race review, better process-tree cancellation and dependency-license audit.
3. Strengthen report/recipe schema validation against attacker-supplied values and clarify manual review controls in UI.
4. Keep all builds reproducible and track CI and release evidence. Only advance to stable v1.0.0 after its own privacy, reliability and platform gates are verified.
5. Limitations: structural similarity is **not** independently verified reproduction; fingerprints can collide; automatic masking can miss secrets; command capture remains unsandboxed and recipes cannot auto-execute.

---

# Historical handoff — v0.3.0 pre-release published

Date: 2026-10-17. Repository: https://github.com/sanskarIN/ReproCrate. Source development branch: `feat/v0.3.0-privacy-bundles`. Merged PR: https://github.com/sanskarIN/ReproCrate/pull/3.

## Implemented source and design

- Core `crates/reprocrate-core/src/privacy.rs`: additional literal masks and exclusions for a copied report, with scanner-limit warnings.
- Core `crates/reprocrate-core/src/bundle.rs`: ZIP manifest, per-file SHA-256, compressed/decompressed bounds, fixed archive member names, duplicate/symlink/traversal defenses, verification-before-extraction and new-directory writes.
- CLI `crates/reprocrate-cli/src/privacy_bundle_commands.rs`: review, bundle create, inspect, verify and extract with explicit manual approval of selected report and attachments.
- `crates/reprocrate-cli/src/main.rs`: registers CLI subcommands.
- `Cargo.toml` and `Cargo.lock`: v0.3.0 Rust workspace, ZIP (deflate-only) and sha2 dependencies.
- `crates/reprocrate-cli/tests/bundle_cli.rs` and core unit tests: end-to-end report review, ZIP verification/extraction and hostile-input rejection.
- `docs/reference/privacy-bundles.md`, `docs/security/archive-threat-model.md`, `docs/releases/v0.3.0.md`, README/roadmap/changelog/security/CLI reference.
- **v0.3.0 is published and verified:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.3.0. GitHub Releases API confirms `draft: false`, `prerelease: true`, published `2026-10-17T13:59:45Z`, source tag commit `ba63bb18481ab92eabd91e80c0fbf632781d253a`.
- Six real uploaded release assets: `reprocrate-v0.3.0-linux-x86_64.tar.gz` (1,565,671 bytes) + its `.sha256`; `reprocrate-v0.3.0-macos-arm64.tar.gz` (1,382,422 bytes) + its `.sha256`; `reprocrate-v0.3.0-windows-x86_64.zip` (1,347,797 bytes) + its `.sha256`.
- v0.2.0 and v0.1.0 pre-releases remain available; stable v1.0.0 is not complete.

## Validation and known gaps

- Generated Cargo.lock and formatted Rust through a one-shot GitHub Actions workflow; temporary elevated-permission workflow was then removed.
- Initial ZIP unit tests exposed test assumptions incompatible with zip 2.x (duplicate names refused on writing, end-of-directory signature width). Corrected the fixture tests in commit `8288b2c`.
- Rust CI on Ubuntu 24.04, Windows 2022 and macOS 14 passed build, tests, Clippy `-D warnings` and formatting at https://github.com/sanskarIN/ReproCrate/actions/runs/39038561203 (also https://github.com/sanskarIN/ReproCrate/actions/runs/39038595395). Initial failing ZIP test assumptions were corrected before merging.
- PR #3 merged into `main` at commit `7c1c9085d283e0b3d655e7284215bc4975733535`.
- v0.3.0 publication workflow fully succeeded: https://github.com/sanskarIN/ReproCrate/actions/runs/39038674027 (validation, draft, Linux/Windows/macOS archive uploads, public release).
- GitHub Releases API separately confirmed the tag, publication status and uploaded asset sizes. No placeholder downloads.
- ZIP content integrity != author authentication. Manual review is not a guarantee of secret-free exports. Archive importer is bounded but needs further third-party review/fuzzing.
- No cloud upload, automatic imported-script execution, GUI, SQLite, cross-report comparison or stable release.

## Next steps

1. Start **v0.4.0**: compare sanitized reports/environments with explicit missing/redacted/unknown states and inert reproduction recipes.
2. Deepen security tests: ZIP fuzzing, symlink/hardlink edge cases, compression bombs, atomic writes and Windows path peculiarities.
3. Harden privacy review: richer manual workflow, data provenance and separate attachment preview.
4. Audit dependency licenses and vulnerable transitive dependencies before expanding distribution targets.
5. Continue Rust-core-sharing Tauri 2/React desktop development after API stability is proven; stable v1.0.0 is not authorized or ready.

---

# Historical handoff — v0.2.0 pre-release published

Date: 2026-10-17 (Asia/Kolkata). Repository: https://github.com/sanskarIN/ReproCrate.
Branch: `feat/v0.2.0-log-analysis`. Pull request: https://github.com/sanskarIN/ReproCrate/pull/2.

**Current state:** **v0.2.0 experimental CLI pre-release is published and verified**, https://github.com/sanskarIN/ReproCrate/releases/tag/v0.2.0. GitHub API confirms `draft: false`, `prerelease: true`, published `2026-10-17T13:37:22Z`, tag target `65325fd3a39edcc87c64a15fc208754462fd28e8`. The previous v0.1.0 pre-release remains available. Stable v1.0.0 is not complete.

## Implemented changes on development branch

- `crates/reprocrate-core/src/analysis.rs`: 8 MiB hard input cap, 20,000-line scan cap, 200 distinct finding cap, 600-character message cap; automatic generic/Cargo/pytest/Jest/Vitest recognition; basic heuristic categories; occurrence grouping; deterministic FNV-1a 64-bit non-cryptographic grouping fingerprint; best-effort sanitized analysis; Markdown escaping.
- `crates/reprocrate-cli/src/log_command.rs`: offline `reprocrate analyze` supporting regular files and standard input (`-`), customizable input bounds, explicit output files, no overwriting of existing output, inert imports and Markdown/JSON output.
- `crates/reprocrate-cli/src/main.rs`: registers new CLI subcommand.
- `crates/reprocrate-core/src/report.rs`: optional additive `analysis` field to existing v1.0.0 *report schema*, legacy report deserialization compatible. Analysis runs after capture output is sanitized, not on unsanitized data. No claimed independent root cause verification.
- `crates/reprocrate-core/src/lib.rs`: shared exports for future Tauri UI.
- `crates/reprocrate-cli/tests/log_analysis.rs`, `crates/reprocrate-core/tests/schema_compatibility.rs` and `fixtures/logs/`: mock logs, old schema import, stdin safety cap, format detection, no-overwrite, privacy and fingerprint regression tests.
- `Cargo.toml` and `Cargo.lock`: workspace bumped to v0.2.0; no additional external runtime dependencies.
- `docs/reference/log-analysis.md`, `docs/releases/v0.2.0.md` and README, changelog, roadmap, CLI reference and SECURITY updates.
- `.github/workflows/prerelease-v0.2.0.yml`: release workflow stages tested portable CLI archives and SHA-256 checksums from Linux x86-64, macOS ARM64 and Windows x86-64, then verifies expected assets before public pre-release publication.

## Verified quality (branch)

- GitHub Actions CI https://github.com/sanskarIN/ReproCrate/actions/runs/39036931608: Linux Ubuntu 24.04, macOS 14 and Windows 2022 build, test, Clippy (`-D warnings`), and rustfmt all **passed** for commit `9e82f646`. A subsequent workflow-only commit requires its own CI confirmation.
- Prior CI found a temporary-directory collision on macOS under concurrent tests. Fixed with a monotonic per-process atomic sequence; follow-up CI passed.
- Earlier formatting-only failures were repaired using `cargo fmt --all` in a temporary trusted GitHub Actions workflow. That write-enabled normalization workflow and trigger were deleted after use.
- **Release pipeline verified successful:** https://github.com/sanskarIN/ReproCrate/actions/runs/39037161008 (validation, draft, Ubuntu/macOS/Windows build-upload, publish all passed). GitHub's Releases API independently verified the published status and these six real assets:
  - `reprocrate-v0.2.0-linux-x86_64.tar.gz` (1,289,296 bytes)
  - `reprocrate-v0.2.0-linux-x86_64.tar.gz.sha256` (104 bytes)
  - `reprocrate-v0.2.0-macos-arm64.tar.gz` (1,125,997 bytes)
  - `reprocrate-v0.2.0-macos-arm64.tar.gz.sha256` (103 bytes)
  - `reprocrate-v0.2.0-windows-x86_64.zip` (1,094,265 bytes)
  - `reprocrate-v0.2.0-windows-x86_64.zip.sha256` (104 bytes)
- PR #2 merged into main at `71bd1e3dccbee6b39c611501a575f093ca3d587a`. Subsequent release marker and documentation commits do not change the immutable v0.2.0 tag source.
- Author and committer on new commits: **Sanskar <sanskarin@outlook.in>** (GitHub connector verified for feature commits; check subsequent automations as needed).

## Known limitations

- Framework adapters are heuristic text recognizers, not complete parsers of all test-result formats, and may misclassify mixed logs.
- Log fingerprints are **not** cryptographic verification, do not encode proof of causality and are not guaranteed unique.
- Sensitive-information redaction is incomplete by design; unknown secrets, private paths and host identifiers may remain in exports. Users must review before sharing.
- CLI capture remains a direct host command execution, not a sandbox or full process-tree supervisor.
- No Tauri desktop UI, safe portable bundle, report comparison, SQLite history, cross-platform desktop installer, hosted documentation, or stable v1.0.0 are claimed.

## Next actions

1. Begin **v0.3.0** privacy review: explicit payload preview and manual redaction before sharing.
2. Threat-model portable ZIP bundles, enforce extraction limits, verify hashes and reject malicious paths/symlinks.
3. Extend log adapters conservatively, add parser fixtures and avoid claims of proven root cause.
4. Review input handling, cancellation, multi-process cleanup, and atomic output writes with adversarial tests.
5. Continue desktop Tauri/React and long-term stability roadmap only after tested backend foundations; no v1.0.0 publishing without verified gates.

---

# Historical handoff — v0.1.0

Last verified: October 17, 2026 (Asia/Kolkata).

## Repository identity

- **Actual repository:** https://github.com/sanskarIN/ReproCrate
- `https://github.com/sanskarIN/RepoCrate` was not found. The uploaded master specification and the writable repository use **ReproCrate**.
- Default branch: `main`
- Project source license: **EPL-2.0**; the repository's existing official `LICENSE` was preserved.
- Confirmed Git author/committer on newly created commits: **Sanskar <sanskarin@outlook.in>**.

## Current shipped milestone: v0.1.0 pre-release

**Verified published:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.1.0

- GitHub API confirmed `tag_name: v0.1.0`, `prerelease: true`, `draft: false`, published October 17, 2026, 12:56:58 UTC.
- Six uploaded assets (three real archives and three SHA-256 files):
  - `reprocrate-v0.1.0-linux-x86_64.tar.gz`
  - `reprocrate-v0.1.0-linux-x86_64.tar.gz.sha256`
  - `reprocrate-v0.1.0-macos-arm64.tar.gz`
  - `reprocrate-v0.1.0-macos-arm64.tar.gz.sha256`
  - `reprocrate-v0.1.0-windows-x86_64.zip`
  - `reprocrate-v0.1.0-windows-x86_64.zip.sha256`
- Release tag target: `d187c85f1c3ee47a9fef4535663c2c850fb1f54d`. Subsequent commits on main only repaired release automation and updated documentation; avoid implying tag tracks latest main.
- This is an **experimental CLI pre-release**, not stable v1.0.0. No Tauri desktop executable or installer is included.

## Implemented source

- `Cargo.toml`, `Cargo.lock`: versioned Rust 2021 Cargo workspace with reproducible dependency resolution.
- `crates/reprocrate-core/src/lib.rs`: shared library entry points.
- `environment.rs`: minimal and standard non-identifying OS/architecture/parallelism capture profiles.
- `capture.rs`: explicit program execution without shell; bounded separate stdout/stderr, configurable timeout, direct-child reaping, truncation, launch errors, exit state and lossy UTF-8 conversion.
- `redaction.rs`: conservative best-effort credential, bearer-token, private-key and sensitive argument masking.
- `report.rs`: schema 1.0.0, per-report IDs, sanitization at the common report boundary, JSON serialization and HTML-escaped Markdown export.
- `crates/reprocrate-cli/src/main.rs`: functional help/version/doctor/capture/inspect with output overwrite prevention and bounded inert JSON imports.
- `crates/reprocrate-cli/tests/cli.rs`: independent command-launch, timeout, non-UTF-8, secret redaction and no-overwrite regression tests.
- `.github/workflows/rust-ci.yml`: Rust build/test/Clippy/rustfmt on Ubuntu 24.04, Windows 2022 and macOS 14.
- `.github/workflows/prerelease-v0.1.0.yml`: version-specific guarded release workflow, platform packaging, checksums and GitHub Release publication.
- `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `SECURITY.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `docs/architecture/overview.md`, `docs/reference/cli.md`, `docs/security/threat-model.md`, `docs/development/dependency-licenses.md`, `docs/releases/v0.1.0.md`.

## Actual verification

- Initial compiler was absent from the local execution container, and direct Rust installation was unavailable there. All Rust compiler/test results below originate from actual **GitHub-hosted runners**, not inferred local runs.
- Rust CI builds, tests, Clippy with `-D warnings`, and `cargo fmt --all --check` passed on all 3 target runner platforms. Verified workflow: https://github.com/sanskarIN/ReproCrate/actions/runs/39034021500 (head `90ba9a07`), and again https://github.com/sanskarIN/ReproCrate/actions/runs/39033911011 (timeout/redaction test commit `60c20126`).
- The prerelease validation job and all three package build/upload jobs succeeded in https://github.com/sanskarIN/ReproCrate/actions/runs/39034318864.
- First packaging run failed on macOS because `sha256sum` is not installed. Added `shasum -a 256` fallback. Next all 3 platform uploads passed, but the final publication job initially failed because the GitHub CLI ran outside a repository checkout. Fixed GH_REPO handling; a one-time recovery workflow verified the six assets and published the release: https://github.com/sanskarIN/ReproCrate/actions/runs/39034551126 (success).
- The GitHub Releases API independently confirmed the published release and its real asset sizes. No unbuilt placeholders were attached.
- Found and fixed a duplicate `env:` key in the release workflow after an unsuccessful workflow-file validation attempt. Temporary release-recovery workflow and its one-time trigger marker were removed after successful publication.
- The initial release development PR was merged with individual commits preserved: https://github.com/sanskarIN/ReproCrate/pull/1.

## Security boundaries / remaining gaps

- Running a selected command is **not sandboxed**; child processes may alter the host and their descendants may survive a timeout.
- Secret detection is best-effort and may miss or over-mask content; user must review full exports.
- Text conversion is lossy for invalid UTF-8; very large outputs are truncated.
- Report import is bounded and inert, but archives/bundles and advanced report verification are not implemented.
- Fully atomic export writes and complete dependency/license/security audits remain incomplete.
- Desktop GUI, SQLite history, comparisons, reproduction recipes, comprehensive log parsers, GitHub Pages docs, cross-platform desktop installers, code signing/notarization and stable release are **not implemented**.
- No v1.0.0 release is approved, built or represented as available.

## Version roadmap and next tasks

1. **v0.2.0:** Implement a bounded generic log analyzer and initial Cargo/pytest/Jest/Vitest adapters with tested deterministic fingerprints.
2. Add explicit export-review/redaction workflow, more timeout and large-output adversarial tests, and reliable cancellation.
3. Introduce environment/report comparison and safe bundle design only after threat-model review.
4. Begin a Tauri 2 + React desktop app reusing `reprocrate-core`, followed by optional SQLite history.
5. Complete dependency and license audit, secure packaging, cross-platform smoke testing, docs deployment and all stable gates before considering **v1.0.0**.

**Handoff principle:** All actual work and release evidence is in repository history, workflow runs, PR #1 and the verified release. Future phases must not claim completion of planned functionality.
