// SPDX-License-Identifier: EPL-2.0
//! The desktop's narrow, explicit IPC boundary. No shell or filesystem plugin.
//! User-chosen paths are read as bounded regular files and never executed.

mod history;

use reprocrate_core::{
    analyze_text, compare_reports, recipe_from_report, review_report, LogAnalysis, LogFormat,
    Report, ReportComparison, ReproductionRecipe, ReviewChoices, SCHEMA_VERSION,
};
use serde::Serialize;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const MAX_REPORT_BYTES: usize = 4 * 1024 * 1024;
const MAX_LOG_PREVIEW_BYTES: usize = 1024 * 1024;
const MAX_MASK_ENTRIES: usize = 100;

#[derive(Debug, Serialize)]
struct DesktopStatus {
    name: &'static str,
    version: &'static str,
    operating_system: &'static str,
    architecture: &'static str,
    mode: &'static str,
}

#[derive(Debug, Serialize)]
struct ReportSummary {
    report_id: String,
    title: String,
    report_version: String,
    schema_version: String,
    os: String,
    architecture: String,
    exit_code: Option<i32>,
    duration_ms: u64,
    timed_out: bool,
    has_analysis: bool,
    warning_count: usize,
    stdout_truncated: bool,
    stderr_truncated: bool,
}

fn read_regular(path: &Path, limit: usize) -> Result<(Vec<u8>, bool), String> {
    let stat = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if !stat.file_type().is_file() {
        return Err("Only explicitly selected regular files are supported (no symlinks, pipes or directories).".into());
    }
    let file = File::open(path).map_err(|error| error.to_string())?;
    let opened = file.metadata().map_err(|error| error.to_string())?;
    if !opened.is_file() {
        return Err("Selected input changed to a non-regular file while opening.".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if stat.dev() != opened.dev() || stat.ino() != opened.ino() {
            return Err("Selected input changed while opening; retry after reviewing the path.".into());
        }
    }
    if opened.len() > limit as u64 {
        // Log preview can truncate, but report input must use strict bounds.
        if limit == MAX_REPORT_BYTES {
            return Err("Report exceeds the 4 MiB import cap.".into());
        }
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    let truncated = bytes.len() > limit;
    bytes.truncate(limit);
    Ok((bytes, truncated))
}

fn load_report(path: &str) -> Result<Report, String> {
    let (bytes, truncated) = read_regular(Path::new(path), MAX_REPORT_BYTES)?;
    if truncated {
        return Err("Report exceeded the 4 MiB import cap while reading.".into());
    }
    let report: Report = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    if report.schema_version != SCHEMA_VERSION {
        return Err("Unsupported report schema; expected 1.0.0.".into());
    }
    Ok(report)
}

fn manual_masks(text: &str) -> Result<Vec<String>, String> {
    if text.len() > 64 * 1024 {
        return Err("Manual mask list exceeds 64 KiB.".into());
    }
    let mut masks = Vec::new();
    for literal in text.lines() {
        let literal = literal.trim_end_matches('\r');
        if literal.is_empty() || literal.len() > 1024 {
            return Err("Mask entries must be 1–1024 bytes, one nonempty literal per line.".into());
        }
        masks.push(literal.to_owned());
        if masks.len() > MAX_MASK_ENTRIES {
            return Err("A maximum of 100 manual mask entries is supported.".into());
        }
    }
    Ok(masks)
}

#[allow(clippy::too_many_arguments)]
fn reviewed_candidate(
    path: String,
    exclude_stdout: bool,
    exclude_stderr: bool,
    exclude_arguments: bool,
    exclude_executable: bool,
    exclude_analysis: bool,
    literals: String,
) -> Result<Report, String> {
    let original = load_report(&path)?;
    let choices = ReviewChoices {
        exclude_stdout,
        exclude_stderr,
        exclude_arguments,
        exclude_executable,
        exclude_analysis,
        additional_literals: manual_masks(&literals)?,
    };
    Ok(review_report(&original, &choices).0)
}

#[tauri::command]
fn desktop_status() -> DesktopStatus {
    DesktopStatus {
        name: "ReproCrate",
        version: env!("CARGO_PKG_VERSION"),
        operating_system: std::env::consts::OS,
        architecture: std::env::consts::ARCH,
        mode: "local-first, no automatic upload",
    }
}

#[tauri::command]
fn inspect_report(path: String) -> Result<ReportSummary, String> {
    let report = load_report(&path)?;
    Ok(ReportSummary {
        report_id: report.report_id,
        title: report.title,
        report_version: report.reprocrate_version,
        schema_version: report.schema_version,
        os: report.environment.os,
        architecture: report.environment.architecture,
        exit_code: report.capture.exit_code,
        duration_ms: report.capture.duration_ms,
        timed_out: report.capture.timed_out,
        has_analysis: report.analysis.is_some(),
        warning_count: report.warnings.len(),
        stdout_truncated: report.capture.stdout_truncated,
        stderr_truncated: report.capture.stderr_truncated,
    })
}

#[tauri::command]
fn compare_report_files(
    baseline_path: String,
    candidate_path: String,
) -> Result<ReportComparison, String> {
    let baseline = load_report(&baseline_path)?;
    let candidate = load_report(&candidate_path)?;
    compare_reports(&baseline, &candidate)
}

#[tauri::command]
fn analyze_log_file(path: String) -> Result<LogAnalysis, String> {
    let (bytes, truncated) = read_regular(Path::new(&path), MAX_LOG_PREVIEW_BYTES)?;
    let text = String::from_utf8_lossy(&bytes);
    Ok(analyze_text(&text, LogFormat::Auto, truncated))
}

#[tauri::command]
fn preview_recipe(path: String) -> Result<ReproductionRecipe, String> {
    recipe_from_report(&load_report(&path)?, false)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn preview_review(
    path: String,
    exclude_stdout: bool,
    exclude_stderr: bool,
    exclude_arguments: bool,
    exclude_executable: bool,
    exclude_analysis: bool,
    literals: String,
) -> Result<Report, String> {
    reviewed_candidate(
        path,
        exclude_stdout,
        exclude_stderr,
        exclude_arguments,
        exclude_executable,
        exclude_analysis,
        literals,
    )
}

// Reviewed exports can still contain private evidence. Do not inherit a
// permissive umask on Unix; refuse overwriting any existing output.
fn write_private_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|error| format!("Cannot create a new report (no overwrite): {error}"))?;
    if let Err(error) = file.write_all(bytes).and_then(|_| file.sync_all()) {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(format!("Cannot write reviewed report: {error}"));
    }
    Ok(())
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn save_review(
    path: String,
    output_path: String,
    exclude_stdout: bool,
    exclude_stderr: bool,
    exclude_arguments: bool,
    exclude_executable: bool,
    exclude_analysis: bool,
    literals: String,
) -> Result<String, String> {
    if path == output_path || output_path.trim().is_empty() {
        return Err("Choose a new, distinct output file path.".into());
    }
    let report = reviewed_candidate(
        path,
        exclude_stdout,
        exclude_stderr,
        exclude_arguments,
        exclude_executable,
        exclude_analysis,
        literals,
    )?;
    let bytes = serde_json::to_vec_pretty(&report).map_err(|error| error.to_string())?;
    if bytes.len() > MAX_REPORT_BYTES {
        return Err("Reviewed report exceeds 4 MiB.".into());
    }
    let output = Path::new(&output_path);
    write_private_new(output, &bytes)?;
    Ok("New reviewed report saved locally. Examine it before sharing.".into())
}

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            desktop_status,
            inspect_report,
            compare_report_files,
            analyze_log_file,
            preview_recipe,
            preview_review,
            save_review,
            history::remember_report,
            history::search_history,
            history::forget_history_entry
        ])
        .run(tauri::generate_context!())
        .expect("Unable to start the ReproCrate desktop application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static TEMP: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn rejects_empty_and_overlong_secret_mask_entries() {
        assert!(manual_masks("secret\n\nnext").is_err());
        assert!(manual_masks(&"x".repeat(1025)).is_err());
        assert_eq!(manual_masks("private\nphrase").unwrap().len(), 2);
    }

    #[test]
    fn desktop_reports_offline_intent() {
        assert!(desktop_status().mode.contains("no automatic upload"));
    }

    #[test]
    fn reviewed_exports_never_overwrite_existing_files() {
        let path = std::env::temp_dir().join(format!(
            "reprocrate-reviewed-{}-{}",
            std::process::id(),
            TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        write_private_new(&path, b"reviewed secret").unwrap();
        assert!(write_private_new(&path, b"overwritten").is_err());
        assert_eq!(fs::read(&path).unwrap(), b"reviewed secret");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o077, 0, "reviewed exports must be owner-only");
        }
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn bounded_reader_rejects_missing_or_non_regular_files() {
        let path = std::env::temp_dir().join(format!(
            "reprocrate-missing-{}-{}",
            std::process::id(),
            TEMP.fetch_add(1, Ordering::Relaxed)
        ));
        assert!(read_regular(&path, MAX_REPORT_BYTES).is_err());
        assert!(read_regular(&std::env::temp_dir(), MAX_REPORT_BYTES).is_err());
    }
}
