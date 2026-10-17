# CLI reference (v0.4.0 pre-release)

Build first with a stable Rust toolchain:

```sh
cargo build --workspace
cargo run -p reprocrate-cli -- --help
```

`reprocrate doctor [--json]` prints operating system, architecture and available processor parallelism. It does not inspect environment variables, hardware serials or the network.

`reprocrate capture [OPTIONS] -- COMMAND [ARGS...]` executes the specified program **without a shell**, stores sanitized bounded stdout/stderr, timing and exit status. Default timeout is 30 seconds; default budget is 65,536 bytes **per stream**. `--timeout-seconds N`, `--max-output-bytes N`, `--profile minimal|standard`, `--title TEXT`, `--output PATH`, and `--markdown PATH` control capture and output. `minimal` includes OS and architecture; `standard` adds available processor parallelism. The child inherits the invoking process environment and working directory. ReproCrate does not sandbox the selected executable.

```sh
cargo run -p reprocrate-cli -- capture --output failure.json --markdown failure.md -- cargo test
cargo run -p reprocrate-cli -- capture --timeout-seconds 5 --profile minimal --output python-failure.json -- python -m pytest
```

`reprocrate inspect PATH` prints metadata of a previously exported JSON report (max 4 MiB) with the current schema version. Import is **inert**: it never runs the recorded command.

Capture reports are created only at the chosen paths. Existing files are not overwritten. Command failure does **not** mean ReproCrate failed to capture; check `capture.exit_code` in the JSON. CLI runtime errors return process exit code 2. Capture output is lossy UTF-8; binary output is not faithfully preserved.

**Review before sharing.** Redaction uses documented patterns and may miss secrets. `capture` is not a secure isolation boundary.

## Analyze selected logs (v0.2.0 pre-release)

`reprocrate analyze INPUT [--format auto|generic|cargo|pytest|jest|vitest] [--output PATH] [--markdown PATH] [--max-input-bytes N]` analyzes bounded text only. Use `-` as INPUT for standard input. It never executes commands found in the file. Default byte cap 1 MiB; hard cap 8 MiB. Automatic pattern redaction is fallible and exported text must be reviewed before sharing. See [log analysis](log-analysis.md) for details.

## Privacy review and portable archives (v0.3.0 pre-release)

`reprocrate review INPUT --output NEW.json [--masks-file FILE] [--preview] [--exclude-stdout] [--exclude-stderr] [--exclude-arguments] [--exclude-executable] [--exclude-analysis]` creates an independently inspectable edited report. `--preview` explicitly prints the full JSON payload.

`reprocrate bundle create --report reviewed.json --output data.zip [--attachment FILE]... --acknowledge-reviewed` requires approval of each included file. `reprocrate bundle inspect data.zip`, `bundle verify data.zip`, and `bundle extract data.zip NEW_DIRECTORY` validate portable bundles without executing them. See [privacy bundle guide](privacy-bundles.md) for exact limits and limitations.

## Report comparison and manual recipes (v0.4.0 pre-release)

`reprocrate compare BASELINE.json CANDIDATE.json [--output diff.json] [--markdown diff.md]` performs a bounded offline **structural** comparison. The resulting files omit raw log and argument values, and represent redacted/truncated fields as indeterminate. Fingerprints are only compared when both analyses are complete.

`reprocrate recipe create --report reviewed.json [--output recipe.json] [--markdown recipe.md]` creates a non-executable manual checklist. Optional `--include-command` must be accompanied by `--acknowledge-reviewed`. `reprocrate recipe inspect PATH` validates a JSON recipe, bounded to 1 MiB, without running commands. See [comparisons](report-comparison.md) and [recipes](reproduction-recipes.md).
