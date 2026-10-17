# Log analysis (experimental v0.2.0 pre-release)

ReproCrate can analyze existing textual logs and capture output **without executing any embedded commands**. It never reads referenced files automatically.

## Commands

```bash
reprocrate analyze ./build.log --output ./build-analysis.json --markdown ./build-analysis.md
reprocrate analyze ./pytest.log --format pytest --output ./pytest-analysis.json
cat build.log | reprocrate analyze - --format generic --max-input-bytes 1048576 --output ./analysis.json
reprocrate capture --output ./failure.json --markdown ./failure.md -- cargo test
```

Formats: `auto`, `generic`, `cargo`, `pytest`, `jest`, `vitest`. `auto` picks the first recognized heuristic signal; real-world combined logs may be classified imperfectly. Explicit formats control labeling, not an unverified root-cause determination.

The default input cap is **1 MiB**, configurable from 1 byte to 8 MiB. There is also a maximum of **20,000 examined lines**, 200 unique findings and 600 Unicode characters per message. If the caller's input cap or findings cap is exceeded, corresponding `truncated_*` flags appear in the JSON. Existing output files are never replaced.

Analysis captures some generic error, warning, test-failure, dependency, permission and timeout indicators. Similar lines can be grouped after conservative normalization of line/column numbers and long numeric identifiers. Fingerprints are deterministic **FNV-1a 64-bit non-cryptographic grouping keys**; they are *not* integrity checks or guarantees of duplicate root cause.

**Privacy:** recognizers run on best-effort sanitized log text. They may not detect novel secrets, personal paths, or every confidential message. Manually review all exports. Analyze is an offline, inert operation. Log output is lossy UTF-8 and may omit malformed characters. Reports exported before v0.2.0 remain importable: their optional `analysis` field is absent.

Neither heuristics nor error fingerprints establish that a bug has been reproduced on another machine.
