# ReproCrate report comparisons (v0.4.0 experimental CLI)

Compare two previously saved, local ReproCrate JSON reports without executing commands, following references or uploading any data.

```bash
reprocrate compare baseline.json candidate.json --output comparison.json --markdown comparison.md
```

The comparator uses report schema `1.0.0`, including reports written by earlier pre-releases. Each import is bounded by the existing **4 MiB** report limit. Every exported comparison lists a fixed set of observed fields (environment profile, OS, architecture, parallelism; command name/argument equality; exit/signal/timeout/duration; stream and launch-error equality and truncation). **Raw output, arguments and their changed values are not included in the comparison.** Report identifiers are included as reference labels, so review them before sharing.

Relationships are `same`, `changed`, `added`, `removed`, `indeterminate`. Observation states are `present`, `missing`, `redacted`, `incomplete`. Missing is not automatically the same as redacted. If a text field includes known masking markers or a stream was truncated, the result is indeterminate rather than a definitive equality/change.

If *both* reports include complete log analysis, the comparator lists heuristic fingerprint grouping differences and repeat-count deltas. It does not include raw error messages. If either report has absent or truncated analysis, it does **not** claim resolved or newly introduced findings. Fingerprints are non-cryptographic and may collide, so this is not proof that a defect was fixed, newly introduced or reproducible.

Comparison files are no-overwrite and are static output. A source report may still contain confidential data; the CLI never uploads reports and does not provide a security guarantee.

**Limitations:** v0.4.0 provides *structural* comparison, not a full source-code diff, cross-machine environment simulation, binary comparison, proof of causality or runtime reproduction.
