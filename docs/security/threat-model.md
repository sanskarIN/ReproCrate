# Threat model (initial milestone)

## Trusted inputs

Only the local CLI user controls command execution and output destinations. The program assumes that the user knows the effects of the selected executable. The command is not run inside a container, virtual machine, or sandbox.

## Untrusted inputs

The child's stdout, stderr, exit status, operating-system error messages, and any imported JSON report are untrusted. Untrusted output can contain credentials, ANSI escapes, HTML and markdown metacharacters, malformed UTF-8, or huge streams.

## Protections already designed

- Process API invocation without a shell; null stdin.
- A strict per-stream output cap with truncation indicators.
- Direct child timeout, kill and reap; bounded post-exit drain.
- Best-effort credentials redaction before persisted JSON and Markdown.
- HTML tag escaping within Markdown exports.
- Explicit output destinations with no overwrite.
- Bounded JSON import and exact schema version rejection.
- No network requests, telemetry, or unapproved file collection.

## Known risks

- Secret detection cannot cover unknown formats.
- Transient raw output necessarily resides in memory; filesystem swap/crash dumps are outside scope.
- A timed-out child may have spawned descendants; v0.1.0 is not a process-tree supervisor.
- Output buffers can lose excess bytes or process output that arrives after the bounded post-exit drain.
- Imports are schema validated but not signature-authenticated or semantic-proof against forged field contents.
- Outputs are created safely against overwriting existing paths, but v0.1.0 writes are not fully atomic against power failure.
- Build artifacts and native binaries have not been independently security audited.

No automatic reproduction, archive extraction, or network upload exists at this milestone.
