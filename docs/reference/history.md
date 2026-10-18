# Report history and search (v0.6.0 experimental desktop)

The ReproCrate desktop app provides an **opt-in local SQLite index** for reports. The index is inactive until you explicitly enter a saved ReproCrate JSON report path and select **Index this report locally**. The app reads the same bounded, schema-validated report format as the existing inspector before writing index metadata.

## How it works

1. Open the **Report history** desktop section.
2. Enter the path of an existing regular JSON report. Directories, symlinks, pipes and oversized reports are rejected by the shared native input boundary.
3. Select **Index this report locally** to save its path and minimal metadata to the desktop's private SQLite database.
4. Search by title, operating system or architecture. Search is local and parameterized; the UI shows the first 50 matches. The backend caps each search at 100 records and query length at 128 bytes.
5. Select **Inspect** to open the original report in the existing inspector. Select **Forget index** to remove its local index row without deleting or modifying the report file.

From v0.6.1, paths are canonicalized after the selected report has been validated, so relative spellings and equivalent paths resolve to one stored absolute path (assuming they point to the same file). Paths and filenames are still potentially sensitive. Indexing the same path a second time updates its metadata instead of duplicating the row. A maximum of 1,000 paths are indexed; adding another requires forgetting an old row.

## What is stored

The SQLite table contains **only** the explicitly selected report path, a best-effort-sanitized report title, report ID, operating system, architecture, and the indexing timestamp. It **does not store** stdout, stderr, raw command arguments, log contents, attachment bytes or the original report JSON. Paths, titles and report IDs themselves can reveal personal or proprietary information. Never copy the SQLite database into public issue reports or backups you do not trust.

On Windows/Linux/macOS the app stores `history.sqlite3` in Tauri's application-data directory. The native layer creates a private app-data directory and owner-only database permissions on Unix, rejects an existing symlink/special-file database, and requests SQLite secure deletion. SQL requests use bound parameters. This is best-effort protection; filesystem backups, copies, snapshots, OS logs and storage remnants can persist. **Forget index** is not proof of secure erasure.

## Privacy and scope limits

There is no automatic scanning, background folder watching, sync, account requirement, telemetry or cloud upload. Search never executes a saved report or a recipe. The history database is not encrypted separately from your operating system's normal disk protection. Use device disk encryption and appropriate account permissions for sensitive metadata.

Current scope is an experimental developer preview; full database migration strategy, encrypted history, file picker integration, paginated browsing, multi-user profiles, security audit and graphical smoke tests are not complete. The existing CLI remains independent and does not access desktop history.
