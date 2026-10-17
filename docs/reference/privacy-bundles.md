# Privacy review and portable bundles (v0.3.0 pre-release candidate)

These tools work fully offline. Commands and scripts contained in an imported report or attachment are **not executed**. Neither importing nor extracting automatically uploads data or contacts GitHub.

## Step 1 — Review your report

Create a capture report in an explicit path, then write a new reviewed candidate:

```sh
reprocrate capture --output failed.json -- cargo test
reprocrate review failed.json --output reviewed.json --exclude-arguments
```

The original report stays unchanged. Review rescans the report's text fields with best-effort secret rules. Use `--exclude-stdout`, `--exclude-stderr`, `--exclude-arguments`, `--exclude-executable`, and/or `--exclude-analysis` to omit fields. `--preview` explicitly prints the **entire** reviewed JSON payload to the terminal; be aware that terminal history or screen recordings might retain it.

To mask project-specific strings, create a local UTF-8 file with one literal per line, then run:

```sh
reprocrate review failed.json --masks-file private-phrases.txt --output masked.json --preview
```

The mask file itself is never copied into exports. It may contain at most 100 non-empty strings, each at most 1,024 bytes, totaling at most 64 KiB. Do not put private strings into command-line arguments.

**Privacy warning:** neither automated masking nor a reviewed candidate proves that a report is free of credentials, names, proprietary details, paths or other personal data. Inspect the full output and any attachments manually before distributing. No accounts or online checks are required.

## Step 2 — Create a reviewed bundle

```sh
reprocrate bundle create --report reviewed.json --attachment approved.log --output report.zip --acknowledge-reviewed
reprocrate bundle inspect report.zip
reprocrate bundle verify report.zip
reprocrate bundle extract report.zip extracted-report
```

The mandatory `--acknowledge-reviewed` switch is an explicit assertion that you personally inspected and approved the report and **every attachment**, not a security guarantee. `bundle create` does not modify or automatically sanitize attachments. Attachments are stored as inert opaque names such as `attachments/000.bin`, deliberately dropping local filesystem paths and original names. Create and extract refuse existing output paths to avoid accidental overwrites.

Bundles contain only `manifest.json`, `report.json`, and (when selected) `attachments/NNN.bin`. Manifest records declared sizes and SHA-256 checksums for each payload file. Verification rejects missing, duplicate, inconsistent, unknown or malformed entries, unsupported schema versions, symlinks and unsafe paths. Checksums detect accidental modifications but do not authenticate a bundle's author; an attacker could replace both file and checksum.

Limits: 12 MiB compressed archive; 8 MiB total uncompressed files, plus a small manifest budget; 4 MiB report; 2 MiB per attachment; 12 attachments; 64 KiB manifest. The importer validates entries *before* writing files, extracts into a newly created directory, and writes only fixed safe paths. No archive instructions are run automatically.

## Security and limitation notes

- ZIP file bytes are held in bounded memory, rather than streamed to arbitrary disk paths. This is deliberate for the initial security milestone.
- Imported material remains untrusted; do not execute an extracted attachment without separately investigating its provenance and safety.
- Report privacy review is a user-driven operation and may produce false positives or false negatives.
- Command capture itself remains **unsandboxed**; its direct-child timeout cannot guarantee killing all descendants.
- Platform packaging remains a CLI pre-release, not a signed desktop installer or stable release.
- There is no automatic cloud upload, telemetry, issue posting, GitHub authentication, or AI processing.
