// SPDX-License-Identifier: EPL-2.0
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
static SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn executable() -> &'static str {
    env!("CARGO_BIN_EXE_reprocrate")
}

fn temp_dir() -> PathBuf {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rc-compare-{}-{now}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    path
}

fn fixture(dir: &Path) -> PathBuf {
    let file = dir.join("before.json");
    let result = Command::new(executable())
        .args(["capture", "--output"])
        .arg(&file)
        .args(["--", "rustc", "--version"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    file
}

#[test]
fn comparison_detects_changes_and_does_not_export_raw_values() {
    let dir = temp_dir();
    let before = fixture(&dir);
    let after = dir.join("after.json");
    let mut report: Value = serde_json::from_slice(&fs::read(&before).unwrap()).unwrap();
    report["capture"]["exit_code"] = Value::from(101);
    report["capture"]["stderr"] = Value::from("error: confidential-example-123");
    report["capture"]["arguments"] = serde_json::json!(["--private-example-arg"]);
    fs::write(&after, serde_json::to_vec(&report).unwrap()).unwrap();
    let json = dir.join("comparison.json");
    let md = dir.join("comparison.md");
    let result = Command::new(executable())
        .arg("compare")
        .arg(&before)
        .arg(&after)
        .arg("--output")
        .arg(&json)
        .arg("--markdown")
        .arg(&md)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let rendered = fs::read_to_string(&json).unwrap();
    assert!(!rendered.contains("confidential-example-123"));
    assert!(!rendered.contains("--private-example-arg"));
    let comparison: Value = serde_json::from_str(&rendered).unwrap();
    assert_eq!(comparison["schema_version"], "1.0.0");
    assert!(comparison["fields"]
        .as_array()
        .unwrap()
        .iter()
        .any(|field| {
            field["field"] == "capture.exit_code" && field["relationship"] == "changed"
        }));
    assert!(!fs::read_to_string(&md)
        .unwrap()
        .contains("confidential-example-123"));
    let again = Command::new(executable())
        .arg("compare")
        .arg(&before)
        .arg(&after)
        .arg("--output")
        .arg(&json)
        .output()
        .unwrap();
    assert!(!again.status.success(), "overwrite must be rejected");
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn recipe_is_inert_and_command_snapshot_opt_in() {
    let dir = temp_dir();
    let source = fixture(&dir);
    let recipe = dir.join("recipe.json");
    let md = dir.join("recipe.md");
    let output = Command::new(executable())
        .args(["recipe", "create", "--report"])
        .arg(&source)
        .arg("--output")
        .arg(&recipe)
        .arg("--markdown")
        .arg(&md)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut parsed: Value = serde_json::from_slice(&fs::read(&recipe).unwrap()).unwrap();
    assert!(parsed["command"].is_null());
    assert_eq!(parsed["automatic_execution"], false);
    assert_eq!(parsed["steps"].as_array().unwrap().len(), 5);
    let inspect = Command::new(executable())
        .args(["recipe", "inspect"])
        .arg(&recipe)
        .output()
        .unwrap();
    assert!(
        inspect.status.success(),
        "{}",
        String::from_utf8_lossy(&inspect.stderr)
    );
    assert!(String::from_utf8_lossy(&inspect.stdout).contains("Automatic execution: false"));
    let no_approval = Command::new(executable())
        .args(["recipe", "create", "--report"])
        .arg(&source)
        .arg("--output")
        .arg(dir.join("not-approved.json"))
        .arg("--include-command")
        .output()
        .unwrap();
    assert!(!no_approval.status.success());
    let approved = dir.join("approved.json");
    let result = Command::new(executable())
        .args(["recipe", "create", "--report"])
        .arg(&source)
        .arg("--output")
        .arg(&approved)
        .args(["--include-command", "--acknowledge-reviewed"])
        .output()
        .unwrap();
    assert!(result.status.success());
    let approved_recipe: Value = serde_json::from_slice(&fs::read(approved).unwrap()).unwrap();
    assert!(approved_recipe["command"]["executable"].is_string());
    parsed["automatic_execution"] = Value::Bool(true);
    let bad = dir.join("unsafe.json");
    fs::write(&bad, serde_json::to_vec(&parsed).unwrap()).unwrap();
    let response = Command::new(executable())
        .args(["recipe", "inspect"])
        .arg(&bad)
        .output()
        .unwrap();
    assert!(
        !response.status.success(),
        "auto execution must be rejected"
    );
    assert!(fs::read_to_string(&md)
        .unwrap()
        .contains("manual reproduction guide"));
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn compare_incomplete_logs_are_indeterminate() {
    let dir = temp_dir();
    let before = fixture(&dir);
    let after = dir.join("after.json");
    let mut report: Value = serde_json::from_slice(&fs::read(&before).unwrap()).unwrap();
    report["capture"]["stdout_truncated"] = Value::Bool(true);
    report["analysis"] = Value::Null;
    fs::write(&after, serde_json::to_vec(&report).unwrap()).unwrap();
    let output = dir.join("comparison.json");
    let result = Command::new(executable())
        .arg("compare")
        .arg(&before)
        .arg(&after)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(result.status.success());
    let parsed: Value = serde_json::from_slice(&fs::read(output).unwrap()).unwrap();
    assert!(parsed["fields"].as_array().unwrap().iter().any(|item| {
        item["field"] == "capture.stdout" && item["relationship"] == "indeterminate"
    }));
    assert_eq!(parsed["fingerprint_comparison_complete"], false);
    fs::remove_dir_all(&dir).unwrap();
}
