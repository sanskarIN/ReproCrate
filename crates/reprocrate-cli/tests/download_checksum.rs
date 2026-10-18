// SPDX-License-Identifier: EPL-2.0
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COUNTER: AtomicU64 = AtomicU64::new(0);

fn fixture() -> (PathBuf, PathBuf, PathBuf) {
    let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::temp_dir().join(format!(
        "rc-download-check-{}-{time}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&root).unwrap();
    let archive = root.join("reprocrate-v0.8.0-demo.zip");
    let checksum = root.join("reprocrate-v0.8.0-demo.zip.sha256");
    fs::write(&archive, b"release archive bytes").unwrap();
    // SHA-256 for the fixture is computed by shared core to avoid fake digests.
    let digest = {
        use sha2::{Digest, Sha256};
        format!("{:x}", Sha256::digest(b"release archive bytes"))
    };
    fs::write(&checksum, format!("{digest}  reprocrate-v0.8.0-demo.zip\n")).unwrap();
    (root, archive, checksum)
}

fn verify(archive: &PathBuf, checksum: &PathBuf) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .arg("verify-download")
        .arg(archive)
        .arg("--checksum")
        .arg(checksum)
        .output()
        .unwrap()
}

#[test]
fn successful_verification_reports_integrity_without_extracting() {
    let (root, archive, checksum) = fixture();
    let response = verify(&archive, &checksum);
    assert!(
        response.status.success(),
        "{}",
        String::from_utf8_lossy(&response.stderr)
    );
    assert!(String::from_utf8_lossy(&response.stdout).contains("SHA-256 matches"));
    assert!(String::from_utf8_lossy(&response.stderr).contains("do not authenticate"));
    assert_eq!(fs::read(&archive).unwrap(), b"release archive bytes");
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn corrupt_archive_and_wrong_filename_fail_closed() {
    let (root, archive, checksum) = fixture();
    fs::write(&archive, b"tampered").unwrap();
    assert!(!verify(&archive, &checksum).status.success());
    fs::write(&checksum, format!("{}  other.zip\n", "a".repeat(64))).unwrap();
    assert!(!verify(&archive, &checksum).status.success());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn malformed_or_multiline_sidecar_cannot_pass() {
    let (root, archive, checksum) = fixture();
    fs::write(&checksum, format!("{}  reprocrate-v0.8.0-demo.zip\nextra\n", "a".repeat(64))).unwrap();
    assert!(!verify(&archive, &checksum).status.success());
    fs::remove_dir_all(root).unwrap();
}
