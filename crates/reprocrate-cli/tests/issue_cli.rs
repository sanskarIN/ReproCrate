// SPDX-License-Identifier: EPL-2.0
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn scratch() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "rc-issue-{}-{now}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&dir).unwrap();
    dir
}

#[test]
fn issue_export_requires_approval_and_never_includes_raw_logs() {
    let dir = scratch();
    let input = dir.join("source.json");
    let cmd = Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .args(["capture", "--output"])
        .arg(&input)
        .args(["--", "rustc", "--version"])
        .output()
        .unwrap();
    assert!(cmd.status.success());
    let mut report: Value = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
    report["title"] = Value::from("PRIVATE_TITLE");
    report["capture"]["stdout"] = Value::from("PRIVATE_STDOUT");
    report["capture"]["stderr"] = Value::from("PRIVATE_STDERR");
    report["capture"]["arguments"] = serde_json::json!(["PRIVATE_ARGS"]);
    fs::write(&input, serde_json::to_vec(&report).unwrap()).unwrap();
    let output = dir.join("issue.md");

    let no_approval = Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .args(["issue", "export", "--report"])
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .args(["--include-title", "--target", "gitlab"])
        .output()
        .unwrap();
    assert!(!no_approval.status.success());
    assert!(!output.exists());

    let success = Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .args(["issue", "export", "--report"])
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .args(["--target", "gitea"])
        .output()
        .unwrap();
    assert!(
        success.status.success(),
        "{}",
        String::from_utf8_lossy(&success.stderr)
    );
    let md = fs::read_to_string(&output).unwrap();
    assert!(md.contains("Gitea"));
    for private in ["PRIVATE_TITLE", "PRIVATE_STDOUT", "PRIVATE_STDERR", "PRIVATE_ARGS"] {
        assert!(!md.contains(private), "{private} leaked into issue draft");
    }
    assert!(md.contains("No issue was created or uploaded"));
    let again = Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .args(["issue", "export", "--report"])
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(!again.status.success(), "no overwrite allowed");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn unknown_target_is_rejected_by_cli() {
    let result = Command::new(env!("CARGO_BIN_EXE_reprocrate"))
        .args(["issue", "export", "--report", "unused.json", "--target", "unknown"])
        .output()
        .unwrap();
    assert!(!result.status.success());
}
