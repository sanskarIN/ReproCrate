# ReproCrate

**Turn bugs into reproducible reports.**

ReproCrate is a local-first, EPL-2.0-licensed developer diagnostics toolkit. The Rust CLI can capture **explicitly selected** commands without a shell, bound captured output, record outcomes, sanitize common secret patterns, and export JSON and Markdown reports. Version v0.2.0 added bounded offline log analysis and heuristic failure fingerprints. Version v0.3.0 added explicit privacy review and integrity-checked portable ZIP bundles. Version v0.4.0 added privacy-minimizing report comparisons and inert reproduction checklists. Version v0.5.0 introduces a Tauri 2/React desktop interface backed by the same Rust core.

> **Latest pre-release:** [v0.5.0 CLI + Desktop developer preview](https://github.com/sanskarIN/ReproCrate/releases/tag/v0.5.0), published October 17, 2026. The release contains real unsigned Windows x86-64, Linux x86-64 and macOS ARM64 binaries for **both** CLI and desktop, each with a SHA-256 file. **Experimental: not a stable v1.0.0, signed installer or clean-machine graphical QA certification.**

## Download

See the [v0.5.0 pre-release](https://github.com/sanskarIN/ReproCrate/releases/tag/v0.5.0) for six unsigned executable archives (three **CLI** and three **Tauri desktop developer-preview** builds) and their six SHA-256 files. Desktop binaries are raw executables, not installers or signed `.app`/`.msi`/`.deb` packages; system WebView2/WebKitGTK prerequisites may apply. Inspect and verify checksums before running downloaded software.

## Analyze logs (available in v0.2.0)

```bash
cargo run -p reprocrate-cli -- analyze fixtures/logs/cargo.txt --output findings.json --markdown findings.md
```

Generic log analysis and early Cargo/pytest/Jest/Vitest heuristics are offline and do not execute imported logs. Grouping fingerprints are **not cryptographic** and do not establish the root cause. Read [log analysis reference](docs/reference/log-analysis.md).

## Privacy review and diagnostic bundles (available in v0.3.0)

```bash
cargo run -p reprocrate-cli -- review failed.json --output reviewed.json --exclude-arguments --preview
cargo run -p reprocrate-cli -- bundle create --report reviewed.json --output diagnostic.zip --acknowledge-reviewed
cargo run -p reprocrate-cli -- bundle verify diagnostic.zip
cargo run -p reprocrate-cli -- bundle extract diagnostic.zip extracted
```

Review the exact JSON and every optional attachment yourself before sharing. The `--acknowledge-reviewed` flag explicitly confirms that review. The tool does not sanitize attachments automatically, upload reports, or execute imported commands. Read the [privacy and bundle guide](docs/reference/privacy-bundles.md).

## Compare reports and generate manual recipes (available in v0.4.0)

```bash
cargo run -p reprocrate-cli -- compare old.json new.json --output diff.json --markdown diff.md
cargo run -p reprocrate-cli -- recipe create --report reviewed.json --output recipe.json --markdown recipe.md
cargo run -p reprocrate-cli -- recipe inspect recipe.json
```

Comparisons report structural changes without exporting raw log lines or arguments. The recipe's command reference is omitted by default and requires explicit approval when included. Imported recipes **never execute automatically**. See [comparison reference](docs/reference/report-comparison.md) and [recipe reference](docs/reference/reproduction-recipes.md).

## Desktop foundation (v0.5.0 pre-release)

A local-first, Tauri 2 + React workspace now lives in `apps/desktop`. It currently supports report inspection, report comparisons, bounded log analysis, inert recipe previews and manually controlled privacy-review exports.

```bash
cd apps/desktop
npm ci
npm test
npm run build
npm run desktop:dev
```

Desktop development requires Node.js, Rust, and the platform requirements for Tauri 2 (WebKitGTK on Linux, WebView2/C++ tools on Windows, Xcode CLI tools on macOS). Its frontend and native Rust workspace have separate reproducible lockfiles; building the CLI does **not** require the desktop prerequisites.

The browser preview is read-only. Commands cannot be executed from imported recipes or reports. Native developer-preview executables have been compiled and published across three CI operating systems. **Signed installers, notarization, broad OS compatibility and real-window manual QA are not claimed.** Read the [desktop developer guide](docs/reference/desktop.md).

## Build from source

Install a stable Rust toolchain (Rust 1.82 or newer), then:

```bash
cargo build --workspace
cargo test --workspace
cargo run -p reprocrate-cli -- --help
cargo run -p reprocrate-cli -- doctor
```

To inspect failures, choose a command you trust and a local destination:

```bash
cargo run -p reprocrate-cli -- capture --output ./failure.json --markdown ./failure.md -- cargo test
cargo run -p reprocrate-cli -- inspect ./failure.json
```

Once installed, replace `cargo run -p reprocrate-cli --` with `reprocrate`.

**Security:** Capture executes the exact program and arguments you request, not an isolated sandbox. Commands might modify files or execute code. Captured data remains local. Pattern-based redaction can miss secrets; **inspect exported JSON and Markdown before sharing**. Never include credentials in command arguments. The tool does not send telemetry or upload reports.

## Implemented scope

- Command capture with no implicit shell
- Independent stdout/stderr byte limits and truncation indicators
- Configurable timeout with child termination and recorded exit state
- Minimal/standard OS and architecture metadata
- Offline generic log import and early Cargo/pytest/Jest/Vitest heuristics (v0.2.0 pre-release)
- Basic secret masking for common credential patterns
- Explicit field exclusion and manually selected literal masks (v0.3.0 pre-release)
- Bounded ZIP bundle create, inspect, verify and extract with SHA-256 manifest checks (v0.3.0 development)
- Versioned JSON and Markdown, generated without a hosted service
- Report inspection and diagnostic environment summary

Not yet provided: desktop installers/GUI, exhaustive archive security audits, comprehensive test-framework parsers, **automated** reproduction, or cloud integrations. Comparing reports and generating manual recipes do not constitute automated reproduction. The pre-release provides verified portable CLI archives only.

See [ROADMAP.md](ROADMAP.md), [SECURITY.md](SECURITY.md) and [what_changed.md](what_changed.md) for scope and validation status.

## Privacy and license

The project is published under the [Eclipse Public License 2.0](LICENSE). Source distributions and third-party components may have additional notice obligations. This project does not require sign-in, subscriptions, or donations for functionality.

## Official links

- [Source repository](https://github.com/sanskarIN/ReproCrate)
- [Developer GitHub](https://github.com/sanskarIN)
- [Website](https://sanskarin.github.io)
- [Contact](https://sanskarin.github.io/contact)
- [Documentation (planned, not yet deployed)](https://sanskarin.github.io/ReproCrate)

Optional support: [Buy Me a Coffee](https://buymeacoffee.com/SanskarCodes) · [Razorpay](https://razorpay.me/@SanskarCodes) · [PayPal](https://paypal.me/SanskarCodes).
