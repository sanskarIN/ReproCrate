// SPDX-License-Identifier: EPL-2.0
//! Explicit privacy review and inert bundle import/export commands.

use crate::{create_new_report, import_json};
use reprocrate_core::bundle::{
    create_bundle, extract_bundle, verify_bundle, MAX_ARCHIVE_BYTES, MAX_ATTACHMENT_BYTES,
    MAX_ATTACHMENTS, MAX_REPORT_BYTES,
};
use reprocrate_core::{review_report, ReviewChoices};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

fn selected_regular_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.file_type().is_file() {
        return Err("only regular files (not symlinks/directories) are accepted".into());
    }
    Ok(())
}

fn read_limited(path: &Path, cap: usize) -> Result<Vec<u8>, Box<dyn Error>> {
    selected_regular_file(path)?;
    let file = File::open(path)?;
    if file.metadata()?.len() > cap as u64 {
        return Err(format!("input file exceeds {cap} bytes").into());
    }
    let mut bytes = Vec::new();
    file.take(cap as u64 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > cap {
        return Err("input grew past the configured limit while reading".into());
    }
    Ok(bytes)
}

fn masks_from_file(path: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let bytes = read_limited(path, 64 * 1024)?;
    let text = String::from_utf8(bytes)?;
    let mut masks = Vec::new();
    for line in text.lines() {
        let literal = line.strip_suffix('\r').unwrap_or(line);
        if literal.is_empty() || literal.len() > 1024 {
            return Err("mask list contains an empty or excessively long literal".into());
        }
        if masks.len() >= 100 {
            return Err("mask list exceeds 100 literal entries".into());
        }
        masks.push(literal.to_owned());
    }
    if masks.is_empty() {
        return Err("mask list must contain at least one non-empty literal".into());
    }
    Ok(masks)
}

#[allow(clippy::too_many_arguments)]
pub fn run_review(
    input: PathBuf,
    output: Option<PathBuf>,
    masks_file: Option<PathBuf>,
    preview: bool,
    exclude_stdout: bool,
    exclude_stderr: bool,
    exclude_arguments: bool,
    exclude_executable: bool,
    exclude_analysis: bool,
) -> Result<(), Box<dyn Error>> {
    if output.as_deref() == Some(input.as_path()) {
        return Err("review output must be a distinct path".into());
    }
    if output.as_ref().is_some_and(|p| p.exists()) {
        return Err("review output already exists (overwrite is disabled)".into());
    }
    selected_regular_file(&input)?;
    let original = import_json(&input)?;
    let opts = ReviewChoices {
        exclude_stdout,
        exclude_stderr,
        exclude_arguments,
        exclude_executable,
        exclude_analysis,
        additional_literals: match masks_file {
            Some(path) => masks_from_file(&path)?,
            None => Vec::new(),
        },
    };
    let (edited, summary) = review_report(&original, &opts);
    let rendered = serde_json::to_vec_pretty(&edited)?;
    if rendered.len() > MAX_REPORT_BYTES {
        return Err("reviewed report exceeds the 4 MiB export cap".into());
    }
    if let Some(path) = output {
        create_new_report(&path, &rendered)?;
        println!("Reviewed candidate: {}", path.display());
    }
    println!("Additional redaction matches: {}", summary.additional_mask_matches);
    println!("Excluded fields: {}", summary.excluded_fields.join(", "));
    if preview {
        // --preview is an explicit opt-in to print the exact JSON export.
        std::io::stdout().write_all(&rendered)?;
        println!();
    }
    eprintln!("Manual review is still required: a scanner cannot certify secrecy.");
    Ok(())
}

pub fn run_bundle_create(
    report_path: PathBuf,
    output: PathBuf,
    attachments: Vec<PathBuf>,
    acknowledge_reviewed: bool,
) -> Result<(), Box<dyn Error>> {
    if !acknowledge_reviewed {
        return Err("bundle creation requires --acknowledge-reviewed after examining every input".into());
    }
    if output.exists() {
        return Err("bundle output already exists (overwrite disabled)".into());
    }
    if attachments.len() > MAX_ATTACHMENTS {
        return Err("too many selected attachments".into());
    }
    selected_regular_file(&report_path)?;
    let report = import_json(&report_path)?;
    let mut files = Vec::new();
    for path in attachments {
        files.push(read_limited(&path, MAX_ATTACHMENT_BYTES)?);
    }
    let bytes = create_bundle(&report, &files)?;
    create_new_report(&output, &bytes)?;
    println!("Bundle: {}", output.display());
    println!("Approved attachments: {}", files.len());
    println!("Bytes: {}", bytes.len());
    eprintln!("No upload occurred. Report and attachment privacy is your responsibility.");
    Ok(())
}

pub fn run_bundle_inspect(path: PathBuf, verify: bool) -> Result<(), Box<dyn Error>> {
    let bytes = read_limited(&path, MAX_ARCHIVE_BYTES)?;
    // Inspection intentionally performs full validation, too.
    let manifest = verify_bundle(&bytes)?;
    println!("Bundle schema: {}", manifest.schema_version);
    println!("Report ID: {}", manifest.report_id);
    println!("Integrity: verified");
    for file in &manifest.artifacts {
        println!("{} ({} bytes, SHA-256 {})", file.path, file.size, file.sha256);
    }
    if verify {
        println!("Bundle validation passed; imported commands were not executed.");
    }
    Ok(())
}

pub fn run_bundle_extract(input: PathBuf, destination: PathBuf) -> Result<(), Box<dyn Error>> {
    let bytes = read_limited(&input, MAX_ARCHIVE_BYTES)?;
    let manifest = extract_bundle(&bytes, &destination)?;
    println!("Extracted verified bundle into {}", destination.display());
    println!("Report ID: {}", manifest.report_id);
    eprintln!("Extracted files remain inert. Do not execute untrusted contents.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_missing_review_acknowledgement() {
        let result = run_bundle_create(PathBuf::from("missing.json"), PathBuf::from("out.zip"), vec![], false);
        assert!(result.is_err());
    }
    #[test]
    fn refuses_missing_mask_list() {
        assert!(masks_from_file(Path::new("missing-mask-file-94321")).is_err());
    }
}
