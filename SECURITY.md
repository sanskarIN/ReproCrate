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
