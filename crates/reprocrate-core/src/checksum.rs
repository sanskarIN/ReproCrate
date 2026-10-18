// SPDX-License-Identifier: EPL-2.0
//! Check the integrity of explicitly selected downloaded archives against a
//! nearby GNU-style SHA-256 sidecar. This does NOT authenticate the publisher.

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

const MAX_CHECKSUM_BYTES: u64 = 512;
const MAX_ARCHIVE_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChecksumVerification {
    pub filename: String,
    pub sha256: String,
    pub bytes_checked: u64,
}

fn regular_open(path: &Path, limit: u64) -> Result<(File, u64), String> {
    let before =
        fs::symlink_metadata(path).map_err(|e| format!("Cannot inspect selected file: {e}"))?;
    if !before.file_type().is_file() {
        return Err("Selected file must be regular (symlinks and devices are rejected).".into());
    }
    if before.len() > limit {
        return Err(format!("Selected file exceeds {limit} bytes."));
    }
    let file = File::open(path).map_err(|e| format!("Cannot open selected file: {e}"))?;
    let opened = file
        .metadata()
        .map_err(|e| format!("Cannot inspect opened file: {e}"))?;
    if !opened.is_file() || opened.len() > limit {
        return Err("Selected file changed type or exceeded its size limit.".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if before.dev() != opened.dev() || before.ino() != opened.ino() {
            return Err("Selected file changed while opening.".into());
        }
    }
    Ok((file, opened.len()))
}

fn expected_hash(sidecar: &str, filename: &str) -> Result<String, String> {
    // One SHA-256 record in the format emitted by sha256sum, shasum and
    // the PowerShell release packager. No loose substring matching.
    let record = sidecar.strip_suffix('\n').unwrap_or(sidecar);
    let record = record.strip_suffix('\r').unwrap_or(record);
    if record.contains(['\n', '\r']) {
        return Err("Checksum file must contain exactly one record.".into());
    }
    let (hash, name) = record
        .split_once("  ")
        .ok_or("Invalid checksum record; expected HASH followed by two spaces and filename.")?;
    if hash.len() != 64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("Invalid SHA-256 digest: expected exactly 64 hexadecimal digits.".into());
    }
    if name != filename {
        return Err("Checksum filename does not match the chosen archive.".into());
    }
    Ok(hash.to_ascii_lowercase())
}

/// Verify archive bytes without extracting, opening scripts, or using network.
/// A matching checksum alone does not establish origin or publisher authenticity.
pub fn verify_release_checksum(
    archive_path: &Path,
    checksum_path: &Path,
) -> Result<ChecksumVerification, String> {
    if archive_path == checksum_path {
        return Err("Archive and checksum paths must be distinct.".into());
    }
    let filename = archive_path
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .ok_or("Archive requires a UTF-8 file name.")?;
    let (sidecar, _) = regular_open(checksum_path, MAX_CHECKSUM_BYTES)?;
    let mut sidecar_bytes = Vec::new();
    sidecar
        .take(MAX_CHECKSUM_BYTES + 1)
        .read_to_end(&mut sidecar_bytes)
        .map_err(|e| format!("Cannot read checksum: {e}"))?;
    if sidecar_bytes.len() as u64 > MAX_CHECKSUM_BYTES {
        return Err("Checksum content exceeded 512 bytes.".into());
    }
    let checksum_text =
        std::str::from_utf8(&sidecar_bytes).map_err(|_| "Checksum must be valid UTF-8.")?;
    let expected = expected_hash(checksum_text, filename)?;

    let (mut file, declared_size) = regular_open(archive_path, MAX_ARCHIVE_BYTES)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    let mut read_total: u64 = 0;
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("Cannot hash archive: {e}"))?;
        if n == 0 {
            break;
        }
        read_total = read_total.saturating_add(n as u64);
        if read_total > MAX_ARCHIVE_BYTES {
            return Err("Archive exceeded the 1 GiB verification limit while reading.".into());
        }
        hasher.update(&buffer[..n]);
    }
    if declared_size != read_total {
        return Err("Archive size changed while hashing; retry with a stable file.".into());
    }
    let actual = format!("{:x}", hasher.finalize());
    if actual != expected {
        return Err(
            "SHA-256 mismatch: the chosen archive differs from the checksum sidecar.".into(),
        );
    }
    Ok(ChecksumVerification {
        filename: filename.to_owned(),
        sha256: actual,
        bytes_checked: read_total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn strict_sidecar_parser_requires_exact_filename_and_one_record() {
        let hash = "a".repeat(64);
        assert_eq!(
            expected_hash(&format!("{hash}  package.tar.gz\n"), "package.tar.gz").unwrap(),
            hash
        );
        assert!(expected_hash(&format!("{hash}  other.tar.gz"), "package.tar.gz").is_err());
        assert!(expected_hash(
            &format!("{hash}  package.tar.gz\n{hash}  other"),
            "package.tar.gz"
        )
        .is_err());
        assert!(expected_hash("abc  package.tar.gz", "package.tar.gz").is_err());
        assert!(expected_hash(&format!("{hash} *package.tar.gz"), "package.tar.gz").is_err());
        assert!(expected_hash(&format!("{hash}  ../package.tar.gz"), "package.tar.gz").is_err());
    }

    #[test]
    fn roundtrip_detects_mutation_and_does_not_extract() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("rc-checksum-{}-{stamp}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let archive = root.join("archive.zip");
        let sidecar = root.join("archive.zip.sha256");
        let payload = b"not-a-zip; no scripts will run";
        fs::write(&archive, payload).unwrap();
        let digest = format!("{:x}", Sha256::digest(payload));
        fs::write(&sidecar, format!("{digest}  archive.zip\n")).unwrap();
        let verified = verify_release_checksum(&archive, &sidecar).unwrap();
        assert_eq!(verified.bytes_checked, payload.len() as u64);
        assert_eq!(verified.sha256, digest);
        fs::write(&archive, b"changed bytes").unwrap();
        assert!(verify_release_checksum(&archive, &sidecar).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_missing_files_and_directory_inputs() {
        let dir = std::env::temp_dir();
        assert!(verify_release_checksum(&dir, &dir).is_err());
        assert!(regular_open(&dir, 512).is_err());
        assert!(regular_open(&dir.join("this-file-is-missing-reprocrate"), 512).is_err());
    }
}
