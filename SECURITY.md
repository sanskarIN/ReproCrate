# Security policy

ReproCrate is a diagnostic tool that handles untrusted processes, output, and eventually imported files. **It does not sandbox commands.** Running a selected process is a user decision and may modify the local system.

## v0.1.0 threat boundary

- Only commands entered deliberately via CLI are executed. No shell is automatically inserted.
- Arguments, standard output and standard error are best-effort sanitized before report export; pattern detection is not proof of safety.
- Collection is bounded per output stream; command execution is time-limited by default.
- The initial CLI stores reports only in locations explicitly provided by the user.
- No telemetry, automatic GitHub issue posting, uploads or remote AI requests.
- The program is not an archive viewer or a security sandbox.

## What may still leak

Credentials in unfamiliar formats, identifiers, unrecognized file paths, application-specific secrets, and confidential data that does not match built-in rules. Console errors, system process lists, and local operating-system facilities are outside ReproCrate's control. Review the complete export before posting online.

## Reporting

Please **do not post secrets, exploit payloads, or confidential logs in public issues**. Use the repository's private security advisory mechanism *if it is enabled* or reach the maintainer via [the official contact page](https://sanskarin.github.io/contact). For non-sensitive bugs, file a normal GitHub issue.

Known boundaries and work remaining are tracked in [what_changed.md](what_changed.md).

## v0.2.0 log analysis boundary

The new `analyze` command reads only an explicitly selected log or stdin, with a strict 8 MiB maximum. It does not follow log paths, execute imported reproduction commands, or make network requests. Analysis fields are sanitized before persistence, but pattern redaction cannot guarantee removal of all secrets. Diffs, fingerprints and format detection remain untrusted heuristics, not conclusions about exploitability or root cause.

## v0.3.0 review and ZIP trust boundaries

`reprocrate review` creates a new report candidate with selected excluded fields and optional exact literal redactions loaded from a local file. No scanner can certify that confidential data was removed. Optional attachments are included verbatim only after explicit manual acknowledgment. The ZIP importer verifies fixed member names, sizes, SHA-256 values, report schema and identity, refuses symlinks/conflicting entries, and checks the whole archive before creating a fresh destination. Checksums **do not authenticate** an archive's author; never run imported scripts automatically. See [archive threat model](docs/security/archive-threat-model.md) and [review and bundles](docs/reference/privacy-bundles.md).

## v0.4.0 comparison and recipe trust boundary

Structural comparisons intentionally exclude raw stdout, stderr, command arguments and other changed values from exported diffs. They can reveal *whether* an observed field changed without sharing it. Masked/truncated fields are indeterminate. Report IDs are carried as labels and need review. Fingerprint comparisons are heuristic grouping signals, not proof of root cause.

Recipes are deliberately inert, omit command references by default, and cannot request automatic execution. Explicit command inclusion requires manual review acknowledgment. Never run a command solely because it appears in a recipe. No upload or telemetry is performed. Consult [comparison docs](docs/reference/report-comparison.md) and [manual recipes](docs/reference/reproduction-recipes.md).


## v0.5.0 desktop IPC and offline boundaries

The experimental Tauri 2 desktop app uses the shared Rust core behind a narrow IPC layer. Its frontend requests only core Tauri capabilities; it does not install shell, filesystem, HTTP, updater, or external upload plugins. The UI may open only paths the user explicitly types, and Rust import rejects symlinks/directories/pipes with strict report and log read caps. Native previews stay local.

The privacy-review screen explicitly presents potentially sensitive output for the user's inspection and creates a new output file without overwriting the source. Manual literal masks and scanner patterns are **not proof that the output is secret-free**. Local file path entry, TOCTOU races and native GUI privacy have not had independent external audits. Do not copy real secrets into reports or share reviewed exports blindly. See [desktop boundary documentation](docs/reference/desktop.md).

## v0.6.1 desktop maintenance boundaries

New reviewed JSON exports request owner-only (0600) permissions on Unix, alongside create-new-only behavior. On Windows, access control is still governed by system and directory settings. The importer checks the opened file handle is a regular file and, on Unix, verifies device/inode identity against the originally selected path. This reduces common symlink-swap risks but is **not** a complete defense against all races or concurrent content changes. History rejects unrecognized future database schema markers and stores a canonical path for explicitly indexed reports. None of these measures certifies imported content safe or redaction complete.

## v0.6.0 local history and metadata

The desktop history feature is opt-in: only explicitly selected report paths and limited report metadata are stored in SQLite. Raw stdout/stderr, command arguments, attachments and full reports are not indexed. The SQLite database resides in the application data directory, is owner-only on Unix, rejects an existing database symlink/special file, and uses parameterized queries and bounded results.

**File paths, titles, IDs, timestamps, and system details are potentially sensitive**, even after best-effort textual masking. History is not separately encrypted; trust your operating system's disk encryption and local account permissions. Forgetting an index entry removes a row without deleting the report and does not guarantee erasure from backups or device snapshots. History never scans folders, executes imported content or uploads data. See [history guide](docs/reference/history.md).
