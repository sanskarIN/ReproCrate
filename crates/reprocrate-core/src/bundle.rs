// SPDX-License-Identifier: EPL-2.0
//! Versioned portable ZIP diagnostic bundles with constrained members.
//!
//! Import and verification are inert. No imported command is ever executed.
//! Attachments are deliberately renamed to opaque fixed archive paths.

use crate::report::{Report, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::{self, Cursor, Read, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

pub const BUNDLE_SCHEMA_VERSION: &str = "1.0.0";
pub const MAX_ARCHIVE_BYTES: usize = 12 * 1024 * 1024;
pub const MAX_UNCOMPRESSED_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_REPORT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_ATTACHMENT_BYTES: usize = 2 * 1024 * 1024;
pub const MAX_ATTACHMENTS: usize = 12;
const MAX_MANIFEST_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BundleArtifact {
    pub path: String,
    pub size: usize,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct BundleManifest {
    pub schema_version: String,
    pub report_id: String,
    pub artifacts: Vec<BundleArtifact>,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn zip_error(error: zip::result::ZipError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, error)
}
fn digest(data: &[u8]) -> String {
    format!("{:x}", Sha256::digest(data))
}
fn member_name(index: usize) -> String {
    format!("attachments/{index:03}.bin")
}
fn is_allowed_path(path: &str) -> bool {
    if path == "manifest.json" || path == "report.json" {
        return true;
    }
    let Some(number) = path
        .strip_prefix("attachments/")
        .and_then(|rest| rest.strip_suffix(".bin"))
    else {
        return false;
    };
    number.len() == 3 && number.bytes().all(|b| b.is_ascii_digit())
}

/// Makes a portable archive from an explicitly reviewed report and attachments.
///
/// The caller must obtain separate explicit consent before including attachments.
/// This library does not claim that the supplied content is secret-free.
pub fn create_bundle(report: &Report, attachments: &[Vec<u8>]) -> io::Result<Vec<u8>> {
    if attachments.len() > MAX_ATTACHMENTS {
        return Err(invalid("too many attachments"));
    }
    if report.schema_version != SCHEMA_VERSION {
        return Err(invalid("unsupported report schema"));
    }
    let report_bytes = serde_json::to_vec_pretty(report)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if report_bytes.len() > MAX_REPORT_BYTES {
        return Err(invalid("report is too large"));
    }

    let mut members = BTreeMap::new();
    members.insert("report.json".to_owned(), report_bytes);
    let mut budget = members["report.json"].len();
    for (index, bytes) in attachments.iter().enumerate() {
        if bytes.len() > MAX_ATTACHMENT_BYTES {
            return Err(invalid("attachment is too large"));
        }
        budget = budget
            .checked_add(bytes.len())
            .ok_or_else(|| invalid("bundle size overflow"))?;
        if budget > MAX_UNCOMPRESSED_BYTES {
            return Err(invalid("uncompressed bundle exceeds cap"));
        }
        members.insert(member_name(index), bytes.clone());
    }

    let artifacts = members
        .iter()
        .map(|(path, bytes)| BundleArtifact {
            path: path.clone(),
            size: bytes.len(),
            sha256: digest(bytes),
        })
        .collect();
    let manifest = BundleManifest {
        schema_version: BUNDLE_SCHEMA_VERSION.into(),
        report_id: report.report_id.clone(),
        artifacts,
    };
    let manifest_bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    if manifest_bytes.len() > MAX_MANIFEST_BYTES {
        return Err(invalid("manifest too large"));
    }
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o600);
    writer
        .start_file("manifest.json", options)
        .map_err(zip_error)?;
    writer.write_all(&manifest_bytes)?;
    for (path, contents) in &members {
        writer.start_file(path, options).map_err(zip_error)?;
        writer.write_all(contents)?;
    }
    let bytes = writer.finish().map_err(zip_error)?.into_inner();
    if bytes.len() > MAX_ARCHIVE_BYTES {
        return Err(invalid("compressed bundle exceeds size cap"));
    }
    Ok(bytes)
}

/// Verify ZIP structure, member names, individual byte caps, checksums,
/// duplicate paths, manifest consistency, and report schema before import.
///
/// Unknown paths, directory entries, symlinks, and trailing unpacked content
/// are never written to disk. Consumers must not execute any imported recipe.
pub fn verify_bundle(bytes: &[u8]) -> io::Result<BundleManifest> {
    inspect_members(bytes).map(|(manifest, _)| manifest)
}

fn inspect_members(bytes: &[u8]) -> io::Result<(BundleManifest, BTreeMap<String, Vec<u8>>)> {
    if bytes.len() > MAX_ARCHIVE_BYTES {
        return Err(invalid("archive exceeds compressed-size limit"));
    }
    let mut archive = ZipArchive::new(Cursor::new(bytes)).map_err(zip_error)?;
    if archive.len() < 2 || archive.len() > MAX_ATTACHMENTS + 2 {
        return Err(invalid("invalid ZIP member count"));
    }
    let mut members: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut normalized_names: BTreeSet<String> = BTreeSet::new();
    let mut uncompressed = 0usize;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(zip_error)?;
        let name = entry.name().map_err(zip_error)?.into_owned();
        if !is_allowed_path(&name)
            || name.contains('\\')
            || name.starts_with('/')
            || name.contains("..")
        {
            return Err(invalid("unsafe or unrecognized ZIP member path"));
        }
        if entry.is_dir() || entry.is_symlink() {
            return Err(invalid("archive directory or symlink is not allowed"));
        }
        if let Some(mode) = entry.unix_mode() {
            let kind = mode & 0o170000;
            if kind != 0 && kind != 0o100000 {
                return Err(invalid("non-regular archive entry is not allowed"));
            }
        }
        // Normalized duplicates are rejected even on case-insensitive filesystems.
        if !normalized_names.insert(name.to_ascii_lowercase()) {
            return Err(invalid("duplicate/conflicting member name"));
        }
        let cap = match name.as_str() {
            "manifest.json" => MAX_MANIFEST_BYTES,
            "report.json" => MAX_REPORT_BYTES,
            _ => MAX_ATTACHMENT_BYTES,
        };
        if entry.size() > cap as u64 {
            return Err(invalid("ZIP member exceeds declared size cap"));
        }
        uncompressed = uncompressed
            .checked_add(entry.size() as usize)
            .ok_or_else(|| invalid("uncompressed-size overflow"))?;
        if uncompressed > MAX_UNCOMPRESSED_BYTES + MAX_MANIFEST_BYTES {
            return Err(invalid("uncompressed archive exceeds aggregate cap"));
        }
        let mut data = Vec::new();
        entry.by_ref().take(cap as u64 + 1).read_to_end(&mut data)?;
        if data.len() > cap || data.len() as u64 != entry.size() {
            return Err(invalid("ZIP member's actual length is invalid"));
        }
        members.insert(name, data);
    }

    let manifest_bytes = members
        .remove("manifest.json")
        .ok_or_else(|| invalid("bundle manifest.json missing"))?;
    let manifest: BundleManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|_| invalid("malformed bundle manifest"))?;
    if manifest.schema_version != BUNDLE_SCHEMA_VERSION {
        return Err(invalid("unsupported bundle schema version"));
    }
    if manifest.artifacts.len() != members.len() {
        return Err(invalid("bundle manifest does not match entry count"));
    }

    let mut previous = String::new();
    for artifact in &manifest.artifacts {
        if artifact.path <= previous {
            return Err(invalid("manifest entries are not unique and sorted"));
        }
        previous = artifact.path.clone();
        if artifact.path == "manifest.json" {
            return Err(invalid("manifest cannot refer to itself"));
        }
        let Some(data) = members.get(&artifact.path) else {
            return Err(invalid("manifest references missing ZIP member"));
        };
        if data.len() != artifact.size || digest(data) != artifact.sha256 {
            return Err(invalid("archive integrity check failed"));
        }
    }
    let report_bytes = members
        .get("report.json")
        .ok_or_else(|| invalid("bundle report.json missing"))?;
    let report: Report = serde_json::from_slice(report_bytes)
        .map_err(|_| invalid("bundle contains malformed report"))?;
    if report.schema_version != SCHEMA_VERSION || report.report_id != manifest.report_id {
        return Err(invalid("bundle report schema or ID mismatch"));
    }

    Ok((manifest, members))
}

/// Safely extracts a fully verified archive into a *new* directory.
///
/// Extracted artifacts are inert bytes: ReproCrate never executes them.
/// No attacker-controlled member names are used as filesystem paths.
pub fn extract_bundle(bytes: &[u8], destination: &Path) -> io::Result<BundleManifest> {
    let (manifest, members) = inspect_members(bytes)?;
    // Refuse to reuse directories or follow existing paths for extraction.
    fs::create_dir(destination)?;
    let result = (|| -> io::Result<()> {
        for (name, data) in members {
            let path = destination.join(&name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut opts = OpenOptions::new();
            opts.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                opts.mode(0o600);
            }
            let mut file = opts.open(path)?;
            file.write_all(&data)?;
            file.sync_all()?;
        }
        Ok(())
    })();
    if let Err(error) = result {
        let _ = fs::remove_dir_all(destination);
        return Err(error);
    }
    Ok(manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        environment::{inspect_environment, CaptureProfile},
        report::Capture,
    };
    fn sample_report() -> Report {
        Report::new(
            "safe sample".into(),
            inspect_environment(CaptureProfile::Minimal),
            Capture {
                executable: "rustc".into(),
                arguments: vec!["--version".into()],
                started_unix_ms: 0,
                duration_ms: 1,
                exit_code: Some(0),
                termination_signal: None,
                timed_out: false,
                launch_error: None,
                stdout: "rustc test".into(),
                stderr: String::new(),
                stdout_truncated: false,
                stderr_truncated: false,
                max_output_bytes_per_stream: 32,
            },
        )
    }

    #[test]
    fn roundtrip_report_and_attachments() {
        let archive = create_bundle(&sample_report(), &[b"approved fixture".to_vec()]).unwrap();
        let manifest = verify_bundle(&archive).unwrap();
        assert_eq!(manifest.artifacts.len(), 2);
        assert!(manifest
            .artifacts
            .iter()
            .any(|a| a.path == "attachments/000.bin"));
    }

    #[test]
    fn rejects_modified_or_missing_checksums() {
        let mut bytes = create_bundle(&sample_report(), &[]).unwrap();
        // An incomplete ZIP directory is always rejected without unpacking.
        bytes.truncate(10);
        assert!(verify_bundle(&bytes).is_err());
    }

    #[test]
    fn refuses_too_many_attachments() {
        let many = vec![Vec::new(); MAX_ATTACHMENTS + 1];
        assert!(create_bundle(&sample_report(), &many).is_err());
    }

    #[test]
    fn refuses_unsafe_member_names() {
        for bad in [
            "../report.json",
            "/report.json",
            "attachments/../../foo",
            "c:\\evil",
            "attachments/000.bin/extra",
        ] {
            assert!(!is_allowed_path(bad));
        }
    }

    #[test]
    fn rejects_duplicate_entry_even_when_archive_is_valid_zip() {
        let writer = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(writer);
        let opts = SimpleFileOptions::default();
        zip.start_file("report.json", opts).unwrap();
        zip.write_all(b"first").unwrap();
        zip.start_file("REPORT.JSON", opts).unwrap();
        zip.write_all(b"second").unwrap();
        let bytes = zip.finish().unwrap().into_inner();
        assert!(verify_bundle(&bytes).is_err());
    }

    #[test]
    fn tampered_manifest_is_rejected() {
        let writer = Cursor::new(Vec::new());
        let mut zip = ZipWriter::new(writer);
        let opts = SimpleFileOptions::default();
        zip.start_file("manifest.json", opts).unwrap();
        zip.write_all(br#"{"schema_version":"1.0.0","report_id":"x","artifacts":[]}"#)
            .unwrap();
        zip.start_file("report.json", opts).unwrap();
        zip.write_all(b"{}").unwrap();
        let bytes = zip.finish().unwrap().into_inner();
        assert!(verify_bundle(&bytes).is_err());
    }

    #[test]
    fn oversized_attachment_is_refused() {
        let too_large = vec![0u8; MAX_ATTACHMENT_BYTES + 1];
        assert!(create_bundle(&sample_report(), &[too_large]).is_err());
    }
}
