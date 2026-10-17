# ReproCrate development handoff

Date: 2026-10-17 (UTC+05:30). Repository: https://github.com/sanskarIN/ReproCrate.

**Important naming:** the user's prose mentioned RepoCrate, but the attached project specification and the existing writable repository both name **ReproCrate**. https://github.com/sanskarIN/RepoCrate returned 404; ReproCrate is the target. Do not rename or create another repository without explicit direction.

## Current milestone

Phase 0 completed as repository discovery and architecture design. **v0.1.0 is in development; no GitHub pre-release or verified binary is published.** v1.0.0 is not claimed.

## Source implemented

- `Cargo.toml` defines a Rust 2021 workspace with `reprocrate-core` and `reprocrate-cli` (license metadata EPL-2.0).
- `crates/reprocrate-core/src/environment.rs`: minimal and standard high-level profiles without usernames/hostnames.
- `crates/reprocrate-core/src/capture.rs`: explicit non-shell child spawning, separate bounded pipes, timeouts, child reap, failure capture and truncation flags.
- `crates/reprocrate-core/src/redaction.rs`: best-effort token/password/bearer/key masking and command argument masking.
- `crates/reprocrate-core/src/report.rs`: schema 1.0.0, sanitized JSON model and HTML-escaped Markdown renderer.
- `crates/reprocrate-cli/src/main.rs`: help/version, capture, doctor and inert report inspection, destination overwrite protection.
- `crates/reprocrate-cli/tests/cli.rs`: CLI smoke tests.
- `.github/workflows/rust-ci.yml`: matrix build, tests, clippy and fmt.
- Root README, roadmap, changelog, security, contributions, conduct, issue/PR templates, architecture, CLI reference and threat model.
- Pre-existing root `LICENSE` preserved: Eclipse Public License 2.0.

## Verification

- GitHub repository, branch, initial LICENSE and permissions: **verified** using the connected GitHub API.
- Remote commits and branch updates: **verified** through the connected GitHub API.
- Local compiler, cargo fmt, cargo clippy, tests and binary E2E: **not run**. The execution container does not have rustc/cargo and cannot reach the Rust distribution network.
- GitHub Actions: **not yet verified** at the time this entry was prepared; check the Actions tab and update status after runs.
- `Cargo.lock`: **not generated**. The initial CI workflow uses `--locked` and will need a committed lockfile before it can pass. Do not claim passing CI until generated.
- Cross-platform release binaries, SHA-256 sums, GitHub releases/tags, GitHub Pages, Tauri/React desktop, bundles, comparisons: **not implemented/not published**.

## Known limitations / security

- Best-effort redaction is not a guarantee; review files before sharing.
- No shell or sandbox. Child may create descendants; timeout terminates only direct child.
- Output decode is lossy UTF-8, reports are output-limited, capture may miss late grandchild output.
- Export writes use create-new and remove partial failed files; no fully atomic power-failure-safe file replacement yet.
- Security tests, privacy review UI, ZIP extraction checks and license dependency audit are incomplete.
- Platform behavior is unverified until CI passes on each system.
- HTML/Markdown output is escaped for captured log text, but still requires adversarial review.

## Git handoff

Branch: `feat/v0.1.0-cli`. Initial main commit `fefef91f9da27101d0636e30b94164849b0f6d84`. Work commits include:
- `b645e0ed` chore: establish ReproCrate Rust workspace and EPL-2.0 project documentation
- `92bd9e5d` feat(core): define versioned reports, environment profiles and privacy masking
- `810d7e81` feat(capture): execute approved processes with bounded pipes and timeouts
- `edd193f9` feat(cli): add capture, doctor and inert report inspection commands
- `759f56dd` fix(redaction): sanitize captured arguments at the shared report boundary

GitHub's created commits have been checked to use author and committer **Sanskar <sanskarin@outlook.in>**.

## Next work

1. Generate and commit Cargo.lock using a trusted Rust environment; repair compile errors.
2. Run CI matrix; fix formatting, clippy and tests and record evidence.
3. Add real output/cancellation/timeout security regression tests and a final local export review facility.
4. Prepare v0.1.0 pre-release only after passable builds and real artifacts.
5. Then progress to v0.2.0 streaming robustness and log parsers; do not skip release-quality gates.

No maintainer approval for a stable v1.0.0 release should be inferred from this milestone.
