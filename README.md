# ReproCrate

**Turn bugs into reproducible reports.**

ReproCrate is a local-first, EPL-2.0-licensed developer diagnostics toolkit. Its first milestone is a standalone Rust CLI that captures **explicitly selected** commands without a shell, bounds captured output, records outcomes, sanitizes common secret patterns, and exports JSON and Markdown reports.

> **Development status:** v0.1.0 CLI foundation in progress. No stable or downloadable release is claimed. Desktop UI, archival bundles, cross-report comparison and hosted docs are future milestones.

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
- Basic secret masking for common credential patterns
- Versioned JSON and Markdown, generated without a hosted service
- Report inspection and diagnostic environment summary

Not yet provided: verified platform binaries, GUI, secure diagnostic ZIP bundles, rich log adapters, report comparisons, automated reproduction, or cloud integrations.

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
