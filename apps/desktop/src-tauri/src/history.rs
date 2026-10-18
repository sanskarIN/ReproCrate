// SPDX-License-Identifier: EPL-2.0
//! Explicit, local-only report history. Never stores raw logs, commands, tokens,
//! imported report JSON or attachments. Indexing requires an affirmative action.

use reprocrate_core::redaction::redact_text;
use rusqlite::{params, Connection};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tauri::Manager;

const MAX_PATH_BYTES: usize = 4096;
const MAX_LABEL_BYTES: usize = 256;
const MAX_RECORDS: i64 = 1000;
const MAX_QUERY_BYTES: usize = 128;
const DEFAULT_PAGE_SIZE: u32 = 50;
const MAX_PAGE_SIZE: u32 = 100;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct HistoryEntry {
    pub id: i64,
    pub path: String,
    pub label: String,
    pub report_id: String,
    pub os: String,
    pub architecture: String,
    pub indexed_unix_seconds: i64,
}

fn prepare_dir(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| format!("Cannot create local app data: {e}"))?;
    let kind = fs::symlink_metadata(path).map_err(|e| e.to_string())?;
    if !kind.file_type().is_dir() {
        return Err("Desktop history directory is not a regular directory.".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        // The app data folder may contain locally sensitive filenames.
        fs::set_permissions(path, fs::Permissions::from_mode(0o700))
            .map_err(|e| format!("Cannot protect app data directory: {e}"))?;
    }
    Ok(())
}

fn open_database(path: &Path) -> Result<Connection, String> {
    if let Some(parent) = path.parent() {
        prepare_dir(parent)?;
    }
    // Refuse a symbolic link or other special file instead of blindly opening.
    if let Ok(stat) = fs::symlink_metadata(path) {
        if !stat.file_type().is_file() {
            return Err("History database must be a regular file, not a symlink.".into());
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if stat.permissions().mode() & 0o077 != 0 {
                return Err(
                    "History database permissions are too broad; restrict to owner (0600).".into(),
                );
            }
        }
    } else {
        use std::fs::OpenOptions;
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        // Do not overwrite pre-existing files. If another instance wins
        // creation, open it only after validating its file type next call.
        let _file = options
            .open(path)
            .map_err(|e| format!("Cannot safely create private history database: {e}"))?;
    }
    let conn = Connection::open(path).map_err(|e| format!("Cannot open local history: {e}"))?;
    conn.busy_timeout(Duration::from_secs(3))
        .map_err(|e| e.to_string())?;
    // Never silently downgrade or write to a database from a future release.
    let schema: i64 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .map_err(|e| format!("Cannot inspect local history schema: {e}"))?;
    if schema > 1 || schema < 0 {
        return Err(format!("Unsupported local history database schema {schema}; refusing to modify it."));
    }
    conn.execute_batch(
        "PRAGMA journal_mode = DELETE;
         PRAGMA secure_delete = ON;
         PRAGMA synchronous = FULL;
         CREATE TABLE IF NOT EXISTS report_history (
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            label TEXT NOT NULL,
            report_id TEXT NOT NULL,
            os TEXT NOT NULL,
            architecture TEXT NOT NULL,
            indexed_unix_seconds INTEGER NOT NULL
          );
         CREATE INDEX IF NOT EXISTS idx_history_recent
           ON report_history(indexed_unix_seconds DESC, id DESC);
         PRAGMA user_version = 1;",
    )
    .map_err(|e| format!("Cannot initialize local history: {e}"))?;
    Ok(conn)
}

fn bounded_text(value: &str, cap: usize) -> String {
    // Ensure char boundary and drop control characters so console/UI strings
    // can be presented safely. These are metadata, not raw evidence.
    let (sanitized, _) = redact_text(value);
    sanitized
        .chars()
        .filter(|ch| !ch.is_control())
        .scan(0usize, |size, ch| {
            let len = ch.len_utf8();
            if *size + len > cap {
                None
            } else {
                *size += len;
                Some(ch)
            }
        })
        .collect()
}

fn add_report(
    conn: &Connection,
    path: &str,
    title: &str,
    report_id: &str,
    os: &str,
    architecture: &str,
) -> Result<HistoryEntry, String> {
    if path.is_empty() || path.len() > MAX_PATH_BYTES || path.contains('\0') {
        return Err("A valid report path of at most 4096 bytes is required.".into());
    }
    let label = bounded_text(title, MAX_LABEL_BYTES);
    let label = if label.is_empty() {
        "Untitled report".to_owned()
    } else {
        label
    };
    let report_id = bounded_text(report_id, 160);
    let os = bounded_text(os, 80);
    let architecture = bounded_text(architecture, 80);
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM report_history", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM report_history WHERE path = ?1)",
            params![path],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if !exists && count >= MAX_RECORDS {
        return Err(
            "History contains 1000 reports. Forget an unused entry before adding another.".into(),
        );
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;
    conn.execute(
        "INSERT INTO report_history(path,label,report_id,os,architecture,indexed_unix_seconds)
         VALUES (?1,?2,?3,?4,?5,?6)
         ON CONFLICT(path) DO UPDATE SET
           label=excluded.label, report_id=excluded.report_id, os=excluded.os,
           architecture=excluded.architecture, indexed_unix_seconds=excluded.indexed_unix_seconds",
        params![path, label, report_id, os, architecture, now],
    )
    .map_err(|e| format!("Could not index local report: {e}"))?;
    conn.query_row(
        "SELECT id,path,label,report_id,os,architecture,indexed_unix_seconds
         FROM report_history WHERE path = ?1",
        params![path],
        decode_entry,
    )
    .map_err(|e| e.to_string())
}

fn decode_entry(row: &rusqlite::Row<'_>) -> rusqlite::Result<HistoryEntry> {
    Ok(HistoryEntry {
        id: row.get(0)?,
        path: row.get(1)?,
        label: row.get(2)?,
        report_id: row.get(3)?,
        os: row.get(4)?,
        architecture: row.get(5)?,
        indexed_unix_seconds: row.get(6)?,
    })
}

fn search(conn: &Connection, query: &str, page_size: u32) -> Result<Vec<HistoryEntry>, String> {
    let query = query.trim();
    if query.len() > MAX_QUERY_BYTES {
        return Err("Search query must be at most 128 bytes.".into());
    }
    let size = i64::from(page_size.clamp(1, MAX_PAGE_SIZE));
    let mut statement = conn
        .prepare(
            "SELECT id,path,label,report_id,os,architecture,indexed_unix_seconds
         FROM report_history WHERE
           ?1 = '' OR instr(lower(label), lower(?1)) > 0
           OR instr(lower(os), lower(?1)) > 0
           OR instr(lower(architecture), lower(?1)) > 0
         ORDER BY indexed_unix_seconds DESC, id DESC LIMIT ?2",
        )
        .map_err(|e| e.to_string())?;
    let entries = statement
        .query_map(params![query, size], decode_entry)
        .map_err(|e| e.to_string())?;
    entries
        .collect::<rusqlite::Result<Vec<_>>>()
        .map_err(|e| e.to_string())
}

fn forget(conn: &Connection, id: i64) -> Result<bool, String> {
    if id < 1 {
        return Err("Invalid history ID.".into());
    }
    conn.execute("DELETE FROM report_history WHERE id = ?1", params![id])
        .map(|rows| rows > 0)
        .map_err(|e| e.to_string())
}

fn database_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let folder = app.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(folder.join("history.sqlite3"))
}

/// Explicit indexing. Report is loaded and schema validated before persistence.
#[tauri::command]
pub fn remember_report(app: tauri::AppHandle, path: String) -> Result<HistoryEntry, String> {
    let report = crate::load_report(&path)?;
    // Persist a stable absolute path rather than one relative to the current
    // process directory; this also deduplicates equivalent path spellings.
    let canonical = fs::canonicalize(&path)
        .map_err(|e| format!("Cannot resolve selected report path: {e}"))?;
    let canonical = canonical
        .to_str()
        .ok_or("Report path cannot be represented as UTF-8.")?;
    let conn = open_database(&database_path(&app)?)?;
    add_report(
        &conn,
        canonical,
        &report.title,
        &report.report_id,
        &report.environment.os,
        &report.environment.architecture,
    )
}

#[tauri::command]
pub fn search_history(
    app: tauri::AppHandle,
    query: Option<String>,
    limit: Option<u32>,
) -> Result<Vec<HistoryEntry>, String> {
    let conn = open_database(&database_path(&app)?)?;
    search(
        &conn,
        query.as_deref().unwrap_or(""),
        limit.unwrap_or(DEFAULT_PAGE_SIZE),
    )
}

#[tauri::command]
pub fn forget_history_entry(app: tauri::AppHandle, id: i64) -> Result<bool, String> {
    let conn = open_database(&database_path(&app)?)?;
    forget(&conn, id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory() -> Connection {
        let c = Connection::open_in_memory().unwrap();
        c.execute_batch(
            "CREATE TABLE report_history(
              id INTEGER PRIMARY KEY, path TEXT NOT NULL UNIQUE, label TEXT NOT NULL,
              report_id TEXT NOT NULL, os TEXT NOT NULL,
              architecture TEXT NOT NULL, indexed_unix_seconds INTEGER NOT NULL);
             CREATE INDEX idx_history_recent ON report_history(indexed_unix_seconds DESC, id DESC);"
        ).unwrap();
        c
    }

    #[test]
    fn opt_in_index_search_and_delete_without_touching_reports() {
        let conn = in_memory();
        let first = add_report(
            &conn,
            "/private/report-one.json",
            "First Rust failure",
            "r1",
            "linux",
            "x86_64",
        )
        .unwrap();
        let second = add_report(
            &conn,
            "/private/report-two.json",
            "Python failure",
            "r2",
            "windows",
            "x86_64",
        )
        .unwrap();
        assert_ne!(first.id, second.id);
        assert_eq!(search(&conn, "rust", 50).unwrap().len(), 1);
        assert_eq!(search(&conn, "windows", 50).unwrap().len(), 1);
        assert_eq!(search(&conn, "' OR 1=1 --", 50).unwrap().len(), 0);
        assert!(forget(&conn, first.id).unwrap());
        assert!(!forget(&conn, first.id).unwrap());
        assert_eq!(search(&conn, "", 50).unwrap().len(), 1);
    }

    #[test]
    fn duplicate_path_refreshes_instead_of_replicating() {
        let conn = in_memory();
        let one = add_report(&conn, "/tmp/one.json", "One", "rid", "linux", "aarch64").unwrap();
        let two = add_report(&conn, "/tmp/one.json", "Updated", "rid", "linux", "aarch64").unwrap();
        assert_eq!(one.id, two.id);
        assert_eq!(two.label, "Updated");
        assert_eq!(search(&conn, "", 100).unwrap().len(), 1);
    }

    #[test]
    fn persists_selected_metadata_across_reopens() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rc-history-{}-{stamp}", std::process::id()
        ));
        let file = root.join("history.sqlite3");
        let first = open_database(&file).unwrap();
        let saved = add_report(&first, "/tmp/manual-report.json", "Recorded issue",
            "stored-id", "linux", "x86_64").unwrap();
        drop(first);
        let second = open_database(&file).unwrap();
        let items = search(&second, "recorded", 20).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, saved.id);
        assert!(forget(&second, saved.id).unwrap());
        drop(second);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_future_schema_without_downgrading_it() {
        let stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rc-history-future-{}-{stamp}",
            std::process::id()
        ));
        let file = root.join("history.sqlite3");
        let conn = open_database(&file).unwrap();
        conn.pragma_update(None, "user_version", 2_i64).unwrap();
        drop(conn);
        let error = open_database(&file).unwrap_err();
        assert!(error.contains("Unsupported local history database schema 2"));
        let untouched = Connection::open(&file).unwrap();
        let version: i64 = untouched
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .unwrap();
        assert_eq!(version, 2);
        drop(untouched);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn strict_query_limits_and_no_raw_credentials() {
        let conn = in_memory();
        let item = add_report(
            &conn,
            "/tmp/test.json",
            "Bearer abcdefgh1234567890 should be scrubbed",
            "abc",
            "linux",
            "x86_64",
        )
        .unwrap();
        assert!(!item.label.contains("abcdefgh1234567890"));
        assert!(search(&conn, &"q".repeat(129), 10).is_err());
        assert_eq!(search(&conn, "none", 500).unwrap().len(), 0);
        assert!(forget(&conn, 0).is_err());
    }
}
