// SPDX-License-Identifier: EPL-2.0
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn executable() -> &'static str { env!("CARGO_BIN_EXE_reprocrate") }
fn temp_dir() -> PathBuf {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "rc-bundle-{}-{nanos}-{}", std::process::id(), SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&dir).unwrap();
    dir
}
fn capture(dir: &std::path::Path) -> PathBuf {
    let path = dir.join("capture.json");
    let output = Command::new(executable())
        .arg("capture").arg("--output").arg(&path)
        .args(["--", "rustc", "--version"]).output().unwrap();
    assert!(output.status.success(), "capture: {}", String::from_utf8_lossy(&output.stderr));
    path
}

#[test]
fn review_bundle_verify_extract_end_to_end() {
    let dir = temp_dir();
    let original = capture(&dir);
    let reviewed = dir.join("reviewed.json");
    let attachment = dir.join("approved.txt");
    fs::write(&attachment, "approved example artifact").unwrap();
    let review = Command::new(executable())
        .arg("review").arg(&original)
        .arg("--output").arg(&reviewed)
        .arg("--exclude-arguments")
        .output().unwrap();
    assert!(review.status.success(), "review: {}", String::from_utf8_lossy(&review.stderr));
    let parsed: Value = serde_json::from_slice(&fs::read(&reviewed).unwrap()).unwrap();
    assert!(parsed["capture"]["arguments"].as_array().unwrap().is_empty());
    let zip = dir.join("archive.zip");
    let without_approval = Command::new(executable())
        .args(["bundle", "create", "--report"]).arg(&reviewed)
        .arg("--output").arg(&zip).output().unwrap();
    assert!(!without_approval.status.success());
    assert!(!zip.exists());

    let created = Command::new(executable())
        .args(["bundle", "create", "--report"]).arg(&reviewed)
        .arg("--output").arg(&zip)
        .arg("--attachment").arg(&attachment)
        .arg("--acknowledge-reviewed")
        .output().unwrap();
    assert!(created.status.success(), "bundle: {}", String::from_utf8_lossy(&created.stderr));

    for command in ["inspect", "verify"] {
        let response = Command::new(executable())
            .args(["bundle", command]).arg(&zip).output().unwrap();
        assert!(response.status.success(), "{command}: {}", String::from_utf8_lossy(&response.stderr));
        assert!(String::from_utf8_lossy(&response.stdout).contains("Integrity: verified"));
    }
    let extraction = dir.join("new");
    let result = Command::new(executable()).args(["bundle", "extract"])
        .arg(&zip).arg(&extraction).output().unwrap();
    assert!(result.status.success(), "extract: {}", String::from_utf8_lossy(&result.stderr));
    assert_eq!(fs::read(extraction.join("attachments/000.bin")).unwrap(), b"approved example artifact");
    assert!(extraction.join("report.json").is_file());

    let again = Command::new(executable()).args(["bundle", "extract"])
        .arg(&zip).arg(&extraction).output().unwrap();
    assert!(!again.status.success(), "should refuse existing directory");
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn masks_file_text_and_preview_without_modifying_original() {
    let dir = temp_dir();
    let original = capture(&dir);
    let masks = dir.join("masks.txt");
    fs::write(&masks, "rustc\n").unwrap();
    let reviewed = dir.join("masked.json");
    let result = Command::new(executable()).arg("review").arg(&original)
        .arg("--masks-file").arg(&masks)
        .arg("--output").arg(&reviewed)
        .arg("--preview").output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let updated = fs::read_to_string(&reviewed).unwrap();
    assert!(!updated.contains("rustc"));
    assert!(updated.contains("[REDACTED BY USER]"));
    assert!(String::from_utf8_lossy(&result.stdout).contains("capture"));
    assert!(fs::read_to_string(&original).unwrap().contains("rustc"));
    fs::remove_dir_all(dir).unwrap();
}
