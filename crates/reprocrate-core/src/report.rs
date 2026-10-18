// SPDX-License-Identifier: EPL-2.0
use crate::{
    analysis::{analyze_text, LogAnalysis, LogFormat},
    environment::Environment,
    redaction::{redact_args, redact_text},
    VERSION,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub const SCHEMA_VERSION: &str = "1.0.0";
static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capture {
    pub executable: String,
    pub arguments: Vec<String>,
    pub started_unix_ms: u64,
    pub duration_ms: u64,
    pub exit_code: Option<i32>,
    pub termination_signal: Option<i32>,
    pub timed_out: bool,
    pub launch_error: Option<String>,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
    pub max_output_bytes_per_stream: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: String,
    pub reprocrate_version: String,
    pub report_id: String,
    pub created_unix_ms: u64,
    pub title: String,
    pub environment: Environment,
    pub capture: Capture,
    pub redaction_matches: usize,
    /// Optional, heuristic findings introduced additively in v0.2.0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<LogAnalysis>,
    pub warnings: Vec<String>,
}

pub fn timestamp_millis() -> u64 {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
}

impl Report {
    pub fn new(title: String, environment: Environment, capture: Capture) -> Self {
        let (stdout, out_count) = redact_text(&capture.stdout);
        let (stderr, err_count) = redact_text(&capture.stderr);
        let (launch_error, launch_count) = match &capture.launch_error {
            Some(message) => {
                let (message, count) = redact_text(message);
                (Some(message), count)
            }
            None => (None, 0),
        };
        let (executable, exe_count) = redact_text(&capture.executable);
        let mut sanitized_capture = capture;
        sanitized_capture.stdout = stdout;
        sanitized_capture.stderr = stderr;
        sanitized_capture.launch_error = launch_error;
        sanitized_capture.executable = executable;
        let (masked_arguments, arg_count) = redact_args(&sanitized_capture.arguments);
        sanitized_capture.arguments = masked_arguments;
        let (_, title_count) = redact_text(&title);
        let (title, _) = redact_text(&title);
        let count = out_count + err_count + launch_count + exe_count + arg_count + title_count;
        // Only analyze already-sanitized output. Never add raw secrets to reports.
        let observed_output = format!("{}\n{}", sanitized_capture.stderr, sanitized_capture.stdout);
        let findings = analyze_text(&observed_output, LogFormat::Auto, false);
        let analysis = if findings.findings.is_empty() {
            None
        } else {
            Some(findings)
        };
        let mut warnings = vec![
            "Automatic secret masking is best-effort. Review the entire report before sharing."
                .to_owned(),
            "The selected command was executed on the host; it was not sandboxed.".to_owned(),
        ];
        if sanitized_capture.stdout_truncated || sanitized_capture.stderr_truncated {
            warnings.push("Output exceeded the per-stream capture limit and was truncated.".into());
        }
        if sanitized_capture.timed_out {
            warnings.push(
                "The command exceeded its timeout and its direct child process was terminated."
                    .into(),
            );
        }
        Self {
            schema_version: SCHEMA_VERSION.into(),
            reprocrate_version: VERSION.into(),
            report_id: format!(
                "rc-{}-{}-{}",
                timestamp_millis(),
                std::process::id(),
                ID_COUNTER.fetch_add(1, Ordering::Relaxed)
            ),
            created_unix_ms: timestamp_millis(),
            title,
            environment,
            capture: sanitized_capture,
            redaction_matches: count,
            analysis,
            warnings,
        }
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
/// Literal Markdown text, never active links, images, emphasis or headings.
/// HTML entities are emitted for markup delimiters before Markdown escaping.
fn inline(text: &str) -> String {
    let escaped = html_escape(text);
    let mut result = String::with_capacity(escaped.len());
    for ch in escaped.chars() {
        if matches!(ch, '\n' | '\r') {
            result.push(' ');
        } else {
            if matches!(
                ch,
                '\\' | '`' | '*' | '_' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '!' | '|' | '~'
            ) {
                result.push('\\');
            }
            result.push(ch);
        }
    }
    result
}
fn fenced(text: &str) -> String {
    let escaped = html_escape(text);
    let tick = char::from(96);
    let longest = escaped
        .split(|c| c != tick)
        .map(str::len)
        .max()
        .unwrap_or(0);
    let fence = tick.to_string().repeat(3.max(longest + 1));
    format!("{fence}text\n{escaped}\n{fence}\n")
}

/// Render an offline-readable report. Untrusted output is HTML escaped.
pub fn render_markdown(report: &Report) -> String {
    let mut body = format!(
        "# {}\n\nReport ID: {}  \nSchema: {}  \nReproCrate: {}\n\n",
        inline(&report.title),
        inline(&report.report_id),
        inline(&report.schema_version),
        inline(&report.reprocrate_version)
    );
    body.push_str("## Observed command\n\n");
    body.push_str(&format!(
        "Executable: {}  \nArguments: {}  \nExit code: {}  \nTimeout: {}  \nDuration: {} ms\n\n",
        inline(&report.capture.executable),
        inline(&format!("{:?}", report.capture.arguments)),
        report
            .capture
            .exit_code
            .map(|n| n.to_string())
            .unwrap_or_else(|| "not available".into()),
        report.capture.timed_out,
        report.capture.duration_ms
    ));
    if let Some(error) = &report.capture.launch_error {
        body.push_str(&format!("Launch error: {}\n\n", inline(error)));
    }
    body.push_str("## Selected environment\n\n");
    body.push_str(&format!(
        "Profile: {}  \nOS: {}  \nArchitecture: {}  \nAvailable parallelism: {}\n\n",
        inline(&report.environment.capture_profile),
        inline(&report.environment.os),
        inline(&report.environment.architecture),
        report
            .environment
            .available_parallelism
            .map(|n| n.to_string())
            .unwrap_or_else(|| "not collected".into())
    ));
    body.push_str(&format!(
        "## Standard output{}\n\n{}\n",
        if report.capture.stdout_truncated {
            " (truncated)"
        } else {
            ""
        },
        fenced(&report.capture.stdout)
    ));
    body.push_str(&format!(
        "## Standard error{}\n\n{}\n",
        if report.capture.stderr_truncated {
            " (truncated)"
        } else {
            ""
        },
        fenced(&report.capture.stderr)
    ));
    if let Some(analysis) = &report.analysis {
        body.push_str("## Observed failure signals (heuristic)\n\n");
        for finding in &analysis.findings {
            body.push_str(&format!(
                "- Line {} | {:?} | {:?} | {} ({} occurrence(s))\n",
                finding.line,
                finding.severity,
                finding.category,
                inline(&finding.message),
                finding.occurrences
            ));
        }
        body.push_str("\nThese signals do not establish root cause.\n\n");
    }
    body.push_str("## Privacy and limitations\n\n");
    for warning in &report.warnings {
        body.push_str(&format!("- {}\n", inline(warning)));
    }
    body.push_str("\n**This report records an observation, not independently verified reproduction steps.**\n");
    body
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::{inspect_environment, CaptureProfile};
    #[test]
    fn report_sanitizes_log_and_html_in_markdown() {
        let capture = Capture {
            executable: "echo".into(),
            arguments: vec![],
            started_unix_ms: 1,
            duration_ms: 1,
            exit_code: Some(0),
            termination_signal: None,
            timed_out: false,
            launch_error: None,
            stdout: "<script>alert(1)</script> password=abc".into(),
            stderr: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            max_output_bytes_per_stream: 512,
        };
        let report = Report::new(
            "Demo".into(),
            inspect_environment(CaptureProfile::Minimal),
            capture,
        );
        let md = render_markdown(&report);
        assert!(!md.contains("<script>"));
        assert!(!md.contains("password=abc"));
        assert!(md.contains("&lt;script&gt;"));
    }
    #[test]
    fn imported_metadata_cannot_insert_active_markdown_links_or_images() {
        let source = "![external image](https://example.invalid/track) # Title\n**bold** [link](https://example.invalid)";
        let rendered = inline(source);
        assert!(rendered.contains(r"!\[external image\]\(https://example.invalid/track\)"));
        assert!(rendered.contains(r"\# Title"));
        assert!(rendered.contains(r"\*\*bold\*\*"));
        assert!(rendered.contains(r"\[link\]\(https://example.invalid\)"));
        assert!(!rendered.contains("![external image]("));
        assert!(!rendered.contains("[link]("));
        assert!(!rendered.contains('\n'));
        assert_eq!(inline("<img src=x>"), "&lt;img src=x&gt;");
    }

    #[test]
    fn every_metadata_channel_uses_literal_markdown_rendering() {
        let env = inspect_environment(CaptureProfile::Minimal);
        let capture = Capture {
            executable: "[app](https://example.invalid)".into(),
            arguments: vec!["![img](https://example.invalid)".into()],
            started_unix_ms: 0,
            duration_ms: 1,
            exit_code: Some(1),
            termination_signal: None,
            timed_out: false,
            launch_error: Some("[failure](https://example.invalid)".into()),
            stdout: "plain output".into(),
            stderr: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            max_output_bytes_per_stream: 256,
        };
        let mut report = Report::new("[title](https://example.invalid)".into(), env, capture);
        report.warnings = vec!["![warning](https://example.invalid)".into()];
        let rendered = render_markdown(&report);
        for unsafe_markup in [
            "[title](https://example.invalid)",
            "[app](https://example.invalid)",
            "![img](https://example.invalid)",
            "[failure](https://example.invalid)",
            "![warning](https://example.invalid)",
        ] {
            assert!(
                !rendered.contains(unsafe_markup),
                "untrusted metadata became active markup"
            );
        }
        assert!(rendered.contains(r"\[title\]"));
        assert!(rendered.contains("plain output"));
    }

    #[test]
    fn schema_round_trip() {
        let env = inspect_environment(CaptureProfile::Minimal);
        let capture = Capture {
            executable: "ok".into(),
            arguments: vec![],
            started_unix_ms: 0,
            duration_ms: 0,
            exit_code: Some(0),
            termination_signal: None,
            timed_out: false,
            launch_error: None,
            stdout: String::new(),
            stderr: String::new(),
            stdout_truncated: false,
            stderr_truncated: false,
            max_output_bytes_per_stream: 1,
        };
        let report = Report::new("test".into(), env, capture);
        let bytes = serde_json::to_vec(&report).expect("serialize");
        let loaded: Report = serde_json::from_slice(&bytes).expect("deserialize");
        assert_eq!(loaded.schema_version, SCHEMA_VERSION);
        assert_eq!(loaded.report_id, report.report_id);
    }
}
