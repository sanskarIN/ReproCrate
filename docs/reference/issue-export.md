# Offline issue draft adapters (v0.7.0)

ReproCrate generates **local Markdown drafts** for GitHub, GitLab, Gitea and generic issue trackers. These are offline formatting adapters, **not** API integrations. No credentials, authentication, outbound HTTP, issue creation or upload occurs.

## CLI

```sh
reprocrate issue export --report reviewed.json --target github --output new-issue.md
reprocrate issue export --report reviewed.json --target gitlab --output gitlab-issue.md
reprocrate issue export --report reviewed.json --target gitea --output gitea-issue.md
reprocrate issue export --report reviewed.json --target generic --output general-issue.md
```

The default draft contains only structural observations: an intentionally generic title, whether execution succeeded/failed/timed out and whether captured output was truncated. A manual checklist asks you to write reproducible steps, expected behavior and actual behavior. **It never copies stdout, stderr, command arguments, commands, file paths, raw finding messages or credentials.**

Optional metadata must be *explicitly enabled* and the source report manually reviewed:

```sh
reprocrate issue export --report reviewed.json --target github --output reviewed-issue.md \
  --include-title --include-environment --include-categories --acknowledge-reviewed
```

- `--include-title`: shortened, pattern-redacted report title with Markdown and HTML escaping. May still reveal context that scanners cannot identify.
- `--include-environment`: OS and architecture, re-scrubbed before export.
- `--include-categories`: up to 10 structural failure categories, severities and counts. **Never includes raw finding messages or log lines.**
- `--acknowledge-reviewed`: mandatory for **any** optional data. It is an affirmative statement by the exporter, not a security certification.

CLI imports an existing ReproCrate schema 1.0.0 report up to 4 MiB, and writes a fresh Markdown file without overwriting. On Unix, new exports request 0600 permissions. The desktop issue-draft page uses the same Rust core and native file boundaries, shows the entire draft for review, and writes a local file only when explicitly requested. Browser preview remains read-only.

## Safely posting

Review the complete new Markdown file. Replace all placeholders with independently verified observations. Evaluate sensitivity and local issue policy before *manually copying* the text into an issue tracker. Never share a diagnostic bundle automatically. The adapters do not link to remote projects or carry authentication tokens, cannot create remote issues, and do not validate that the reproduced outcome is accurate.

The field filters and pattern masking reduce accidental exposure but **cannot guarantee absence of secrets**. Never treat a rendered issue draft as proof of root cause or as an executable recipe.
