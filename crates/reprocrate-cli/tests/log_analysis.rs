// SPDX-License-Identifier: EPL-2.0
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_reprocrate")
}

fn scratch() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "reprocrate-analyze-{}-{nanos}-{}",
        std::process::id(),
        TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).expect("temp directory");
    path
}

#[test]
fn analyze_four_framework_fixtures_without_running_them() {
    let fixtures = [
        ("cargo", include_str!("../../../fixtures/logs/cargo.txt")),
        ("pytest", include_str!("../../../fixtures/logs/pytest.txt")),
        ("jest", include_str!("../../../fixtures/logs/jest.txt")),
        ("vitest", include_str!("../../../fixtures/logs/vitest.txt")),
    ];
    let dir = scratch();
    for (name, text) in fixtures {
        let log = dir.join(format!("{name}.log"));
        let json = dir.join(format!("{name}.json"));
        let markdown = dir.join(format!("{name}.md"));
        fs::write(&log, text).expect("write fixture");
        let result = Command::new(binary())
            .arg("analyze")
            .arg(&log)
            .arg("--output")
            .arg(&json)
            .arg("--markdown")
            .arg(&markdown)
            .output()
            .expect("spawn CLI");
        assert!(
            result.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let parsed: Value =
            serde_json::from_slice(&fs::read(&json).expect("read JSON")).expect("valid JSON");
        assert_eq!(parsed["detected_format"], name);
        assert!(parsed["findings"].as_array().is_some_and(|x| !x.is_empty()));
        assert!(fs::read_to_string(&markdown)
            .expect("Markdown")
            .contains("ReproCrate log analysis"));
        let repeat = Command::new(binary())
            .arg("analyze")
            .arg(&log)
            .arg("--output")
            .arg(&json)
            .output()
            .expect("repeat CLI");
        assert!(
            !repeat.status.success(),
            "overwriting an existing file was permitted"
        );
    }
    fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn analyze_stdin_is_bounded_and_sanitizes_password() {
    let dir = scratch();
    let json = dir.join("stdin-analysis.json");
    let mut child = Command::new(binary())
        .arg("analyze")
        .arg("-")
        .arg("--max-input-bytes")
        .arg("75")
        .arg("--output")
        .arg(&json)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn analyzer");
    child.stdin.take().expect("stdin").write_all(
        b"error: password=never-expose-me\nerror: second failure\nerror: third failure\nerror: fourth"
    ).expect("write log");
    let result = child.wait_with_output().expect("analyzer response");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let json_text = fs::read_to_string(&json).expect("read output");
    assert!(!json_text.contains("never-expose-me"));
    let parsed: Value = serde_json::from_str(&json_text).expect("parse output");
    assert_eq!(parsed["truncated_input"], true);
    assert!(parsed["findings"].as_array().is_some_and(|x| !x.is_empty()));
    fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn capture_embeds_optional_findings_and_keeps_report_compatible() {
    let dir = scratch();
    let json = dir.join("captured.json");
    // A fake program name cannot execute. Importing that record must be inert.
    let result = Command::new(binary())
        .arg("capture")
        .arg("--output")
        .arg(&json)
        .args(["--", "reprocrate_missing_program_again"])
        .output()
        .expect("run CLI");
    assert!(result.status.success());
    let saved: Value = serde_json::from_slice(&fs::read(&json).expect("read")).expect("parse");
    assert_eq!(saved["schema_version"], "1.0.0");
    assert!(saved["capture"]["launch_error"].is_string());
    let imported = Command::new(binary())
        .arg("inspect")
        .arg(&json)
        .output()
        .expect("import");
    assert!(imported.status.success());
    fs::remove_dir_all(dir).expect("cleanup");
}

#[test]
fn log_analyzer_rejects_illegal_bounds() {
    let result = Command::new(binary())
        .args(["analyze", "fake.log", "--max-input-bytes", "9999999999"])
        .output()
        .expect("run CLI");
    assert!(!result.status.success());
}
