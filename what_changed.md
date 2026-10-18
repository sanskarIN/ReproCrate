# v0.9.0 hardening and release quality (published pre-release)

Date: 2026-10-18. Repo: https://github.com/sanskarIN/ReproCrate, development branch `feat/v0.9.0-release-quality` merged via [PR #13](https://github.com/sanskarIN/ReproCrate/pull/13) with 26 focused commits.
**Latest independently confirmed pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.9.0. Public `draft:false`, `prerelease:true`, published 2026-10-18T05:19:10Z, all 12 real nonempty uploaded CLI/desktop archives and checksums.

## Current source work

- Escaped untrusted inline Markdown metadata in `render_markdown` to prevent active Markdown image and remote link syntax in report title, executable, arguments, launch errors and warnings, while retaining HTML escaping.
- Added core regression tests for malicious inline Markdown and representative metadata channels. These reduce rendering hazards but do not prove arbitrary Markdown renderers secure.
- Hardened CLI `import_json` to reject nonregular selected report files (including symlinks, FIFOs, directories and device files), check opened handle type and 4 MiB cap, and on Unix check dev/inode identity before reading.
- Added import rejection tests for symlinks, directories and missing files, not claiming fully race-free filesystem access.
- Added `scripts/smoke_cli.py`, headless tests against actual built v0.9.0 CLI executable: `--version`, `doctor --json`, inert log analyzer, offline self-generated archive SHA-256 check, corrupted checksum rejection, existing-output rejection.
- Added `scripts/release_asset_gate.py` and unit tests. The published release gate now requires a non-published draft prerelease with exactly twelve correctly named, nonempty uploaded assets; six real executable archives and six checksum sidecars.
- The v0.9.0 release workflow also runs the just-built CLI to verify the checksum of each platform's actual release archive before upload.
- Updated all Rust core/CLI, native desktop/Tauri, npm frontend version metadata and locked manifests to v0.9.0.
- Added `docs/qa/release-candidate.md` distinguishing automated multi-platform checks from **not-yet-performed** manual clean-machine Windows/Linux/macOS GUI, accessibility, license and signing reviews.
- Source docs staged in `docs/releases/v0.9.0.md`, README, CHANGELOG, ROADMAP and SECURITY.

## Verified public release and remaining tasks

1. **Merged:** PR #13 retained 26 focused feature commits; source is on main. Release marker `.release/v0.9.0` committed with follow-up corrections for authenticated draft lookup.
2. **Final green workflow:** https://github.com/sanskarIN/ReproCrate/actions/runs/39099649232. Passed version validation, Rust formatting, Clippy, workspace tests and Python dependency inventory tests, 3 real platform CLI smoke tests and checksum self-verification, 3 desktop Tauri native builds/uploads, plus a strict **exactly 12 uploaded assets** publish gate.
3. **Verified GitHub release:** v0.9.0 is public and tagged as a **pre-release**, not stable. All six real CLI/desktop unsigned binary archives and six SHA-256 files for Windows x86-64, Linux x86-64 and macOS ARM64 are uploaded and nonempty.
4. **Safe failed attempt and fix:** Initial pipeline https://github.com/sanskarIN/ReproCrate/actions/runs/39098953794 compiled and uploaded all six binaries successfully, but refused to publish because the GitHub REST `releases/tags` lookup returned 404 for an unpublished draft. Fixed `release_asset_gate.py` to select a unique tag from the authenticated releases listing with regression tests, updated workflow, reran full 3-OS builds and published only after the strict checks passed.
5. **GitHub Pages site still needs owner setup** unless separately confirmed live: repo **Settings → Pages → Build and deployment → Source: GitHub Actions**, then rerun the docs workflow. Earlier deployment attempts failed from missing repo-level Pages enablement/permissions.
6. **Manual clean-machine GUI, accessibility, signing/notarization, comprehensive license/security review and cross-OS compatibility tests remain unverified**; see `docs/qa/release-candidate.md`. v0.9.0 is experimental and does not claim stable v1.0.0 or perfectly sanitized evidence.

---

# v0.8.0 distribution and integrity handoff (published pre-release)

Date: October 18, 2026. Repository: https://github.com/sanskarIN/ReproCrate. Development branch `feat/v0.8.0-distribution` merged by [PR #12](https://github.com/sanskarIN/ReproCrate/pull/12), preserving 33 focused commits.

**Latest verified published pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.8.0. The [release workflow](https://github.com/sanskarIN/ReproCrate/actions/runs/39095891087) completed successfully. GitHub Releases API verified `draft:false`, `prerelease:true`, published 2026-10-18T04:16:28Z and all 12 genuine nonempty CLI/desktop archives/checksum files.

## Delivered source changes

1. Streaming `verify_release_checksum` in Rust core; standalone CLI command `reprocrate verify-download ARCHIVE --checksum SIDE-CAR` with strict single-record SHA-256 parsing, basename checking, import caps, symlink rejection, non-execution, and misleading-authenticity warning.
2. Core and CLI tests cover successful checks, mutation, mismatched or malformed sidecars and refusal to open directories.
3. Accessible static website in `site/` with local CSS, restrictive CSP, keyboard navigation, no remote assets/scripts; independent GitHub Pages deployment workflow.
4. Dependency inventory script and tests parse Cargo and npm lockfiles, preserving unknown license values rather than guessing. Workflow uploads a machine-readable inventory and schedules separate Cargo RustSec and npm advisory scans. The inventory is **not** a license clearance/security certification.
5. Developer distribution table, checksum instructions, dependency review guidance and full notes at `docs/releases/v0.8.0.md`.
6. Version and lockfiles for CLI, core, native Tauri, desktop JS and bundle metadata aligned to 0.8.0; guarded release workflow only publishes after all platforms and checksum sidecars exist.
7. Security, roadmap, changelog and README describe limitations and current publication state.

## Verified release and remaining work

- PR #12 merged into `main`; genuine release marker `.release/v0.8.0` committed at `bf0691c8d7e03776cc6f0f2321c51e1ec9e92c2a`.
- The gated release run https://github.com/sanskarIN/ReproCrate/actions/runs/39095891087 passed version checks, Rust formatting, Clippy, all workspace tests and dependency inventory validation. Three CLI and three native desktop package jobs all succeeded, followed by successful publication.
- GitHub Releases API confirmed exactly **12 uploaded nonempty assets**: CLI and desktop unsigned binaries for Linux x86-64, Windows x86-64, macOS ARM64, plus six SHA-256 files.
- **GitHub Pages not live yet**: first deployment https://github.com/sanskarIN/ReproCrate/actions/runs/39095879663 failed because Pages was not enabled; retry https://github.com/sanskarIN/ReproCrate/actions/runs/39095938968 failed with `Resource not accessible by integration` on creating a Pages site. Repository administrator must open **Settings → Pages → Build and deployment → Source = GitHub Actions**, then rerun the Docs workflow. Do not advertise the site before it succeeds.
- Weekly/manual advisory scans were configured but are **not** proof of completed advisories or license compliance. A separate security/third-party license review, signed installers and clean-machine GUI QA remain necessary before a stable v1.0.0.

---

# Current handoff — v0.7.0 offline issue draft adapters published

Date: October 18, 2026. Repository: https://github.com/sanskarIN/ReproCrate
Development branch: `feat/v0.7.0-issue-drafts`, merged through [PR #11](https://github.com/sanskarIN/ReproCrate/pull/11) at `abed25994bb209982ae7cd40d77dd0aa1b5747f1` (28 focused feature commits preserved).
**Latest verified published pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.7.0. Published October 18, 2026 at `2026-10-18T02:58:12Z`, with `draft:false` and `prerelease:true`, verified via GitHub Releases API.

## Implemented

- Rust core `issue.rs` introduces `IssueTarget`, `IssueOptions`, `IssueDraft`, and inert `draft_issue` with adapters for GitHub, GitLab, Gitea and generic Markdown.
- Default drafts disclose only structural execution status/truncation. Report titles, arguments, commands, raw stdout/stderr, paths, report IDs and finding messages are not exported.
- Optional title, environment and bounded structural finding categories require affirmative review acknowledgment; re-redaction and escaping are applied to included untrusted strings.
- CLI `issue export` writes a new local Markdown file, refuses overwrite, performs existing bounded 4 MiB report import and schema validation; Unix files use owner-only permissions.
- Tauri IPC `preview_issue` and `save_issue` use the same Rust core, validate adapter names and refuse overwrites, preserving the no-network/no-shell boundary.
- React `IssuePanel` provides tracker selection, opt-in fields, explicit acknowledgment, complete Markdown preview and user-initiated local save. Added page navigation and typed IPC.
- Added Rust privacy regressions and cross-platform CLI integration tests for no secret disclosure by default, approval requirements, unsafe markup and overwrite refusal.
- Versioned CLI/core, Tauri, frontend, config and locked dependency manifests to v0.7.0; added guarded release workflow.
- Documentation: `docs/reference/issue-export.md`, `docs/releases/v0.7.0.md`, README, ROADMAP, CHANGELOG and SECURITY.

## Verified release evidence

1. **Successful release pipeline:** https://github.com/sanskarIN/ReproCrate/actions/runs/39091493244. Validation, draft, three CLI build-upload jobs, three desktop build-upload jobs and publication all passed.
2. **Genuine uploaded assets:** Six unsigned executable archives (CLI and desktop, Windows x86-64 / Linux x86-64 / macOS ARM64), each paired with SHA-256 file, total 12. GitHub Releases API independently confirmed all nonempty uploads.
3. **Review:** PR #11 merged into main retaining all 28 focused commits. All GitHub-authored source commits use `sanskarin@outlook.in`; merge commits may be attributed to GitHub web-flow.
4. **CI fixes:** Early gated attempts failed safely before publication. Corrected rustfmt in the new CLI integration test and removed a stale `0.6.1` assertion from the newly cloned version validation stage, then retriggered from `.release/v0.7.0` at `ca68483daf2ba08044b524c4c8c01e700e3d25aa`.
5. **No stable v1.0.0 claim** or signed installer/clean-machine GUI certification. New issue drafts remain local only and manual review is required before any publication.

**Limits:** No remote issue posting, credentials, automatic commands, verified reproduction, signed installers, full GUI smoke-test, or guaranteed removal of unrecognized secrets. Issue drafts require review before manual sharing.

---

# Current handoff — v0.6.1 desktop privacy/history maintenance published

Date: 2026-10-18. Repository: https://github.com/sanskarIN/ReproCrate. Development branch: `feat/v0.6.1-hardening`, merged as [PR #10](https://github.com/sanskarIN/ReproCrate/pull/10) at `f8ff9549900c591c9cc786e00bab2db33ec5a251` with 21 feature/maintenance commits preserved.

**Latest verified public experimental pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.6.1. GitHub Releases API confirms `draft:false`, `prerelease:true`, published `2026-10-18T01:20:13Z`; tag target `995b8f8261f26ba0508fe2e8752a0cc8fbda724c`.

## Implemented and published

1. Native Rust desktop history refuses SQLite databases with future `PRAGMA user_version` markers, protecting forward-compatible user data.
2. Explicit indexing records canonical report paths after JSON schema validation to reduce duplicate aliases and preserve history across different working directories.
3. Native import checks opened-handle regular-file status and Unix device/inode identity to reduce common file-swap races; no complete filesystem-race proof is claimed.
4. Reviewed JSON exports use exclusive creation and owner-only (0600) Unix file permissions with partial-write cleanup. Windows file ACLs are not independently assured.
5. Added regression coverage for future SQLite schemas remaining unchanged, reviewed-export permissions and no-overwrite behavior.
6. Aligned root Rust, separate native Tauri Rust, npm/TypeScript, Tauri bundle and lockfile package versions to 0.6.1.
7. Added `.github/workflows/prerelease-v0.6.1.yml`, guarded on a future `.release/v0.6.1` marker pushed to main after verification, building three CLI and three unsigned desktop archives plus SHA-256 checksums.
8. Updated changelog, roadmap, security and technical docs; release notes at `docs/releases/v0.6.1.md`.

## Verified publication and known limitations

- **Source:** PR #10 merged and release marker `.release/v0.6.1` added to `main` at `995b8f8261f26ba0508fe2e8752a0cc8fbda724c`.
- **Successful gated GitHub Actions run:** https://github.com/sanskarIN/ReproCrate/actions/runs/39085609817 — validation, draft, 3 CLI package jobs, 3 native desktop package jobs and publish all succeeded.
- **Verified 12 nonempty assets via GitHub Releases API:** Linux x86-64, Windows x86-64 and macOS ARM64 each have CLI and desktop unsigned executable archives, with separate SHA-256 checksum files.
- Existing unsigned developer-preview artifacts are not signed installers; no real-window clean-machine QA, comprehensive filesystem race proof, automatic reproduction, upload or enterprise certificate is claimed.
- Next feature milestone remains **v0.7.0** (explicitly reviewed issue-export and integration adapters), not bundled into this maintenance update.

---

# Latest handoff — v0.6.0 local desktop history pre-release published

Date: 2026-10-17. Repo: https://github.com/sanskarIN/ReproCrate
Source branch: `feat/v0.6.0-desktop-history`, merged via [PR #9](https://github.com/sanskarIN/ReproCrate/pull/9) at `511642112e450727c1269aa6918072f9cc7bad91`.
**Current published pre-release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.6.0. GitHub Releases API confirms `draft:false`, `prerelease:true`, publication `2026-10-17T15:50:54Z`, tag target `18dcaabdeafb2d84f8a935aef54799df5498dc1e`.

## Implemented this milestone

- `apps/desktop/src-tauri/src/history.rs`: explicit SQLite indexing under local app-data directory; stores user-selected path, best-effort-masked report title/ID, OS/architecture and index timestamp, but no raw logs, command arguments, attachment bytes or report JSON.
- Schema v1, unique report paths (re-index updates), 1,000-row limit, 128-byte search query limit, result cap, SQL parameters, basic DB permissions and SQLite secure-delete request. Data storage is opt-in.
- `apps/desktop/src-tauri/src/lib.rs`: Tauri command registration for `remember_report`, `search_history` and `forget_history_entry`; keeps inert report parsing and size validation.
- `apps/desktop/src/HistoryPanel.tsx` and App navigation: index, search, reopen in report inspector, forget only the index row. No auto scan/upload.
- `apps/desktop/src/bridge.ts` / `types.ts` and CSS: type-safe desktop index API and accessible form controls, disabled in browser preview.
- Added `rusqlite` bundled SQLite in the desktop-only Cargo workspace; CLI/root Cargo dependencies remain independent.
- Versioned root CLI/core, desktop Rust, app config and npm metadata to v0.6.0, plus regenerated reproducible dependency lockfiles.
- Updated desktop and native CI branch filters for future development; temporary elevated-permission lock-generation workflow removed after success.
- Security/history guide: `docs/reference/history.md`. Release candidate notes: `docs/releases/v0.6.0.md`.

## Verified CI, native packages, and publication

- **Dependency/formatting:** https://github.com/sanskarIN/ReproCrate/actions/runs/39045206451. One-shot trusted lockfile generator succeeded and its elevated-permission workflow was removed before merge.
- **Source tests:** https://github.com/sanskarIN/ReproCrate/actions/runs/39045623956. React tests/production build and native Rust Tauri checks passed on Ubuntu 24.04, Windows 2022 and macOS 14. Root CLI CI was green, including checked source commits.
- **Real unsigned desktop executable compilation:** https://github.com/sanskarIN/ReproCrate/actions/runs/39045623960 (Windows/Linux/macOS all succeeded).
- **PR #9 merged** into main at `511642112e450727c1269aa6918072f9cc7bad91`.
- **Versioned v0.6.0 release workflow:** https://github.com/sanskarIN/ReproCrate/actions/runs/39046224080 — validation, draft, all three CLI package jobs, all three desktop package jobs and final publish passed.
- GitHub Releases API verified the publication timestamp, pre-release status and real, nonempty assets:
  - `reprocrate-desktop-v0.6.0-linux-x86_64.tar.gz` (4,693,020 bytes) + `.sha256` (112 bytes)
  - `reprocrate-desktop-v0.6.0-macos-arm64.tar.gz` (4,348,505 bytes) + `.sha256` (111 bytes)
  - `reprocrate-desktop-v0.6.0-windows-x86_64.zip` (4,366,750 bytes) + `.sha256` (112 bytes)
  - `reprocrate-v0.6.0-linux-x86_64.tar.gz` (1,616,673 bytes) + `.sha256` (104 bytes)
  - `reprocrate-v0.6.0-macos-arm64.tar.gz` (1,432,310 bytes) + `.sha256` (103 bytes)
  - `reprocrate-v0.6.0-windows-x86_64.zip` (1,401,427 bytes) + `.sha256` (104 bytes)
- These binaries are **unsigned developer previews**; native GUI launch on clean machines and signed installer flows have not been validated. Filesystem paths, report titles and index metadata remain sensitive even after best-effort sanitization. SQLite deletion cannot promise secure erasure.
- **No stable v1.0.0** has been published or represented as production ready.

## Next milestone — v0.7.0 and later

1. Implement *explicit, reviewed* GitHub issue Markdown export and conservative integration adapters with no automatic upload.
2. Add native file picker UX with least-privilege grants, keyboard/screen-reader verification and persisted settings controls.
3. Deepen security with archive fuzzing, SQLite migration/retention review, atomic output, process-tree cleanup and real GUI smoke tests.
4. Audit all resolved dependency licenses and security reports across Rust, npm, SQLite, Tauri and system libraries.
5. Continue distribution/docs site and signed/notarized packaging in later milestones; stable v1.0.0 remains blocked on its own quality/privacy gates.

---

# Historical handoff — v0.5.0 desktop + CLI pre-release published

Date: 2026-10-17, repository https://github.com/sanskarIN/ReproCrate.
Source development branch: `feat/v0.5.0-desktop-foundation` (merged as [PR #5](https://github.com/sanskarIN/ReproCrate/pull/5) at `a806aeb7a02c4936d53b9e03dd7e5a2f0e91db59`).
**Published public experimental release:** https://github.com/sanskarIN/ReproCrate/releases/tag/v0.5.0. Verified via GitHub Releases API: `draft:false`, `prerelease:true`, publication `2026-10-17T15:15:28Z`, immutable release target `f05920e198cf8abd0df7fc01d096715e604fd1bf`.

## Implemented code

- `apps/desktop/src-tauri`: independently versioned Tauri 2 Rust workspace, committed Cargo.lock, generated native icons, restrictive Tauri capability file and CSP; explicitly invokes the same `reprocrate-core` workspace crate via a path dependency.
- `apps/desktop/src-tauri/src/lib.rs`: native IPC commands: `desktop_status`, `inspect_report`, `compare_report_files`, `analyze_log_file`, `preview_recipe`, `preview_review`, `save_review`.
- Core IPC safety: only user-selected regular files, symlink/directory/pipe rejection, 4 MiB JSON report cap and 1 MiB log cap, limited manual literal list and new-file-only reviewed export. No shell, network request, authentication or automatic recipe execution command exists.
- `apps/desktop/src`: React + TypeScript Vite native UI for workspace, inspect, structural comparisons, heuristic log signals, inert manual recipe and privacy preview with selected field omissions/manual masking. Browser view is read-only and explicitly warns that native file access is unavailable.
- `apps/desktop/src/format.test.ts`: frontend formatting/privacy-presentation unit tests. Npm packages pinned by package-lock.json. Source icon `apps/desktop/public/reprocrate.svg` produces platform assets committed under the native icons folder.
- Root `Cargo.toml` and lockfile versioned 0.5.0; desktop package and native Rust app also at 0.5.0.
- Separate `.github/workflows/desktop-ci.yml`: npm tests and production build, then Tauri native Rust check against independent lockfile on Ubuntu 24.04/Windows 2022/macOS 14.
- `.github/workflows/desktop-build.yml`: additional gated developer-preview native executable build, OS matrix Rust boundary tests and retained build artifacts. This is a build verification workflow, *not* a GitHub Release.
- Documentation: `docs/reference/desktop.md`, `docs/releases/v0.5.0.md`, security and architecture notes.
- All new commits use Git identity Sanskar <sanskarin@outlook.in>. Temporary write-enabled icon/lock generator workflow and its trigger were removed after completion.

## Verified evidence and blockers

- Frontend npm tests/production build **passed** and Windows/Linux/macOS Tauri cargo check **passed**: https://github.com/sanskarIN/ReproCrate/actions/runs/39042465054.
- Existing root CLI Rust CI remained successful after adding the desktop scaffold.
- Locked npm deps and separate Tauri Cargo.lock and generated icons via successful one-shot run https://github.com/sanskarIN/ReproCrate/actions/runs/39042289779.
- All three unsigned Tauri native executables (Windows 2022, Ubuntu 24.04, macOS 14) built successfully with real binaries and passed Rust boundary tests: https://github.com/sanskarIN/ReproCrate/actions/runs/39043004232.
- **Entire gated v0.5.0 release run succeeded:** https://github.com/sanskarIN/ReproCrate/actions/runs/39043664762 (workspace validation, draft, all three CLI build-upload jobs, all three desktop build-upload jobs, final public publish). GitHub Releases API separately confirms twelve assets:
  - Desktop Linux x86-64 tarball 3,645,225 bytes + `.sha256` 112 bytes
  - Desktop macOS ARM64 tarball 3,454,645 bytes + `.sha256` 111 bytes
  - Desktop Windows x86-64 ZIP 3,435,567 bytes + `.sha256` 112 bytes
  - CLI Linux x86-64 tarball 1,616,753 bytes + `.sha256` 104 bytes
  - CLI macOS ARM64 tarball 1,432,212 bytes + `.sha256` 103 bytes
  - CLI Windows x86-64 ZIP 1,401,423 bytes + `.sha256` 104 bytes
- The desktop uses system Tauri/WebView prerequisites; actual graphical smoke tests, binary signing/notarization and native installer distributions are still **not** verified. Archives are portable unsigned developer-preview executables, not installers.

## Next

1. Next milestone **v0.6.0**: local SQLite history/search, durable safe report index, improved desktop accessibility and controlled native file-picker UX.
2. Improve desktop privacy review: finer-grained optional fields, no incidental screen logging, test secure export paths and race conditions.
3. Run clean-machine GUI sessions on Windows/Linux/macOS, build signed installers and validate WebKitGTK/WebView2 availability; do not claim this has already been done.
4. Audit Tauri/React transitive dependency licenses, archive fuzzing and process-tree cancellation; preserve EPL-2.0 obligations.
5. Keep v1.0.0 gated on platform, privacy, security, reproducibility and release-quality evidence.

---

# Historical handoff — v0.4.0 pre-release published

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
