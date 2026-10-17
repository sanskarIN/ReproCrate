// SPDX-License-Identifier: EPL-2.0
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_reprocrate")
}

fn scratch() -> PathBuf {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).expect("clock").as_nanos();
    let path = std::env::temp_dir().join(format!("reprocrate-test-{}-{unique}", std::process::id()));
    fs::create_dir(&path).expect("create isolated scratch directory");
    path
}

#[test]
fn doctor_reports_only_selected_fields() {
    let output = Command::new(binary()).args(["doctor", "--json"]).output().expect("CLI starts");
    assert!(output.status.success());
    let json: Value = serde_json::from_slice(&output.stdout).expect("valid JSON");
    assert!(json["os"].is_string());
    assert!(json["architecture"].is_string());
    assert!(json.get("hostname").is_none());
}

#[test]
fn capture_round_trip_and_prevent_overwrite() {
    let dir = scratch();
    let path = dir.join("capture.json");
    let md = dir.join("capture.md");
    let output = Command::new(binary())
        .args(["capture", "--output"])
        .arg(&path)
        .args(["--markdown"])
        .arg(&md)
        .args(["--", "rustc", "--version"])
        .output()
        .expect("CLI starts");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let report: Value = serde_json::from_slice(&fs::read(&path).expect("report exists")).expect("valid JSON");
    assert_eq!(report["schema_version"], "1.0.0");
    assert_eq!(report["capture"]["exit_code"], 0);
    assert!(report["capture"]["stdout"].as_str().unwrap_or_default().contains("rustc"));
    assert!(fs::read_to_string(&md).expect("Markdown exists").contains("Standard output"));

    let again = Command::new(binary())
        .args(["capture", "--output"]).arg(&path)
        .args(["--", "rustc", "--version"]).output().expect("repeat starts");
    assert!(!again.status.success(), "output overwrite unexpectedly permitted");

    let inspect = Command::new(binary()).arg("inspect").arg(&path).output().expect("inspection starts");
    assert!(inspect.status.success());
    assert!(String::from_utf8_lossy(&inspect.stdout).contains("Schema: 1.0.0"));
    fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn failed_program_still_produces_a_useful_report() {
    let dir = scratch();
    let path = dir.join("missing.json");
    let output = Command::new(binary()).arg("capture").arg("--output").arg(&path)
        .args(["--", "reprocrate_missing_program_104729"])
        .output().expect("CLI starts");
    assert!(output.status.success());
    let report: Value = serde_json::from_slice(&fs::read(&path).expect("capture saved")).expect("valid JSON");
    assert!(report["capture"]["launch_error"].is_string());
    assert!(report["capture"]["exit_code"].is_null());
    fs::remove_dir_all(dir).expect("cleanup");
}
