// SPDX-License-Identifier: EPL-2.0
//! The schema is additive: v0.1 JSON reports must remain importable in v0.2.
use reprocrate_core::analysis::{analyze_text, LogFormat};
use reprocrate_core::Report;
use serde_json::{json, Value};

#[test]
fn legacy_v0_1_json_deserializes_without_analysis_field() {
    let legacy = json!({
        "schema_version": "1.0.0",
        "reprocrate_version": "0.1.0",
        "report_id": "rc-legacy",
        "created_unix_ms": 0,
        "title": "legacy example",
        "environment": {
            "capture_profile": "minimal",
            "os": "linux",
            "architecture": "x86_64",
            "available_parallelism": null
        },
        "capture": {
            "executable": "cargo",
            "arguments": ["test"],
            "started_unix_ms": 0,
            "duration_ms": 100,
            "exit_code": 101,
            "termination_signal": null,
            "timed_out": false,
            "launch_error": null,
            "stdout": "",
            "stderr": "error[E0308]: mismatched types",
            "stdout_truncated": false,
            "stderr_truncated": false,
            "max_output_bytes_per_stream": 65536
        },
        "redaction_matches": 0,
        "warnings": ["Test only"]
    });
    let parsed: Report = serde_json::from_value(legacy).expect("v0.1 report should load");
    assert!(parsed.analysis.is_none());
    let round_trip: Value = serde_json::to_value(parsed).expect("serialize");
    assert!(round_trip.get("analysis").is_none());
}

#[test]
fn new_reports_use_additive_optional_analysis_field() {
    use reprocrate_core::{inspect_environment, Capture, CaptureProfile};

    let capture = Capture {
        executable: "cargo".into(),
        arguments: vec!["test".into()],
        started_unix_ms: 1,
        duration_ms: 3,
        exit_code: Some(101),
        termination_signal: None,
        timed_out: false,
        launch_error: None,
        stdout: String::new(),
        stderr: "error[E0308]: mismatched types\npassword=known-demo-key".into(),
        stdout_truncated: false,
        stderr_truncated: false,
        max_output_bytes_per_stream: 100,
    };
    let report = Report::new(
        "compiler failure".into(),
        inspect_environment(CaptureProfile::Minimal),
        capture,
    );
    let findings = report.analysis.as_ref().expect("analysis for error");
    assert_eq!(findings.detected_format, "cargo");
    assert!(!serde_json::to_string(&report).expect("json").contains("known-demo-key"));
}

#[test]
fn error_fingerprints_are_stable_between_calls() {
    let text = "error[E0308]: mismatched types at src/lib.rs:12:5";
    let a = analyze_text(text, LogFormat::Cargo, false);
    let b = analyze_text(text, LogFormat::Cargo, false);
    assert_eq!(a.findings[0].fingerprint, b.findings[0].fingerprint);
}
