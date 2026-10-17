# Manual reproduction recipes (v0.4.0 experimental CLI)

A ReproCrate recipe is a **versioned, declarative checklist**, never a script, shell command, installer or automatic reproduction engine.

## Export without a command (default)

```bash
reprocrate recipe create --report reviewed.json --output recipe.json --markdown recipe.md
reprocrate recipe inspect recipe.json
```

By default the recipe omits executable and arguments entirely, as well as raw stdout and stderr. It contains report schema/version, reference ID, high-level environment, expected exit code, timeout observation, heuristic failure fingerprint IDs when available, and five manual checklist steps: review, prepare environment, manually reproduce only if trusted, capture, and compare.

To include a command reference **after inspecting the report and arguments**, specify both flags:

```bash
reprocrate recipe create --report reviewed.json --output with-command.json --include-command --acknowledge-reviewed
```

Command references are informational fields, never shell scripts. Best-effort secret masking runs again, but cannot ensure safety or remove every sensitive argument. **Never blindly run commands from someone else's report or recipe.** Use an isolated environment if and only if you have independently assessed the command.

`recipe inspect` imports up to 1 MiB of JSON, refuses unknown recipe and report schemas, unknown properties, requests for automatic execution, oversized commands and malformed step ordering. No imported instruction is executed. CLI writes fresh output paths only and refuses overwrites.

This is an experimental cross-platform CLI feature, not automatic reproduction. It does not infer dependency installation, construct executable scripts, validate real environment compatibility or prove the original bug happened again.
