# Archive threat model (v0.3.0)

**Assets:** local files, personal and proprietary report contents, the user's workspace, process integrity, and importer memory.

**Adversary:** an imported, potentially malicious ZIP with crafted path names, duplicate entries, symlinks, forged checksums, compressed bombs, missing reports, unknown schema fields, or malformed byte sequences.

**Defenses:**
1. Archive input byte size is strictly bounded before passing it to ZIP parsing.
2. Member count and individual + total **declared and actual** uncompressed sizes are bounded.
3. ZIP entries are restricted to `report.json`, `manifest.json`, and exact `attachments/NNN.bin` patterns; no input member's original name is used as an arbitrary filesystem path.
4. Directory/symlink/special-file entries, unknown files and duplicate case-folded names are rejected.
5. Manifest has a fixed schema; lists the exact set of payloads in sorted order, with size and SHA-256; all entries are verified.
6. The report JSON must parse and match the supported schema and ID before extraction.
7. Verification runs before creating an output directory. Extraction creates a new directory and uses create-new file semantics rather than overwriting existing paths.
8. Extracted byte contents remain inert; no automatic command execution or network communication.

**Residual risks:** attachment payloads may themselves be malicious or confidential; SHA-256 inside the archive is not a digital signature and does not establish publisher identity. A hostile report can contain strings with control characters even after best-effort masking. Local filesystem operations can fail partway through; cleanup is best-effort. ZIP parser code is a third-party dependency requiring ongoing auditing. Snapshot verification does not prevent a user from deliberately running a dangerous extracted file.

**Stable-release blockers:** further fuzzing, code review, process-tree handling, cross-platform path-hardening tests, dependency/license audit, robust atomic export semantics, and tested GUI privacy review. This milestone makes a narrow, verifiable security claim rather than promising universal archive safety.
