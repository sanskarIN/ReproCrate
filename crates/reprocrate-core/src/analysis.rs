// SPDX-License-Identifier: EPL-2.0
//! Bounded, deterministic, heuristic analysis of untrusted textual logs.
//! Classifications are observations; fingerprints are NOT cryptographic hashes.
use crate::redaction::redact_text;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// Fixed schema for standalone log analysis; not the captured-report schema.
pub const ANALYSIS_SCHEMA_VERSION: &str = "1.0.0";
pub const MAX_LOG_BYTES: usize = 8 * 1024 * 1024;
const MAX_LINES: usize = 20_000;
const MAX_FINDINGS: usize = 200;
const MAX_MESSAGE_CHARS: usize = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    Auto,
    Generic,
    Cargo,
    Pytest,
    Jest,
    Vitest,
}

impl LogFormat {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Generic => "generic",
            Self::Cargo => "cargo",
            Self::Pytest => "pytest",
            Self::Jest => "jest",
            Self::Vitest => "vitest",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Compilation,
    Test,
    Dependency,
    Permissions,
    FileNotFound,
    Timeout,
    Runtime,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Finding {
    /// First occurrence within the supplied log text, starting at line 1.
    pub line: usize,
    pub severity: Severity,
    pub category: Category,
    /// Framework adapter chosen by the caller/detector, not a proven origin.
    pub adapter: String,
    /// A shortened, sanitized single-line observation.
    pub message: String,
    /// An FNV-1a 64-bit stable grouping key; NOT collision resistant.
    pub fingerprint: String,
    pub occurrences: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LogAnalysis {
    pub schema_version: String,
    /// Requested input format.
    pub requested_format: String,
    /// Heuristic detector result or explicit user-selected format.
    pub detected_format: String,
    pub scanned_lines: usize,
    pub truncated_input: bool,
    pub truncated_findings: bool,
    pub redaction_matches: usize,
    pub findings: Vec<Finding>,
    pub warnings: Vec<String>,
}

fn clip_chars(text: &str, limit: usize) -> String {
    let mut chars = text.chars();
    let head: String = chars.by_ref().take(limit).collect();
    if chars.next().is_some() {
        format!("{head}…")
    } else {
        head
    }
}

fn line_has(haystack: &str, needle: &str) -> bool {
    haystack.to_ascii_lowercase().contains(needle)
}

/// The detector scans at most MAX_LINES lines and does not traverse the filesystem.
fn detect_format(input: &str) -> LogFormat {
    for line in input.lines().take(MAX_LINES) {
        if line.starts_with("error[E")
            || line.contains("test result: FAILED")
            || (line.contains("Compiling ") && line_has(line, " v"))
        {
            return LogFormat::Cargo;
        }
        if line.contains("short test summary info")
            || line.contains("_ ERROR at setup of ")
            || line.contains(" in pytest")
            || line.starts_with("E   AssertionError")
            || line.starts_with("FAILED ") && line.contains("::")
        {
            return LogFormat::Pytest;
        }
        if line.contains(" RUN  v") && line_has(line, "vitest")
            || line.trim_start().starts_with("❯ ")
            || line.contains("Test Files  ") && line.contains("failed")
        {
            return LogFormat::Vitest;
        }
        if line.contains("Test Suites:")
            || line.starts_with("FAIL ") && line.ends_with(".test.js")
            || line.starts_with("FAIL ") && line.ends_with(".test.ts")
        {
            return LogFormat::Jest;
        }
    }
    LogFormat::Generic
}

fn classify(line: &str, adapter: LogFormat) -> Option<(Severity, Category)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    let is_warning = lower.starts_with("warning:")
        || lower.starts_with("warn:")
        || lower.contains(" warning: ");
    let failure = lower.contains("error:")
        || lower.starts_with("error[")
        || lower.starts_with("error ")
        || lower.starts_with("fail ")
        || lower.starts_with("failed ")
        || lower.starts_with("failure ")
        || lower.starts_with("e   ")
        || lower.starts_with("× ")
        || lower.starts_with("✖ ")
        || lower.contains("assertionerror")
        || lower.contains("panic")
        || lower.contains("timed out")
        || lower.contains("timeout:")
        || lower.starts_with("fatal:");

    if !failure && !is_warning {
        return None;
    }
    let category = if lower.contains("permission denied") || lower.contains("eacces") {
        Category::Permissions
    } else if lower.contains("no such file")
        || lower.contains("not found")
        || lower.contains("enoent")
    {
        Category::FileNotFound
    } else if lower.contains("timed out") || lower.contains("timeout:") {
        Category::Timeout
    } else if lower.contains("could not resolve")
        || lower.contains("failed to download")
        || lower.contains("dependency")
    {
        Category::Dependency
    } else if lower.starts_with("error[") || lower.contains("could not compile") {
        Category::Compilation
    } else if matches!(adapter, LogFormat::Pytest | LogFormat::Jest | LogFormat::Vitest)
        || lower.contains("assertionerror")
        || lower.contains("test result: failed")
    {
        Category::Test
    } else if lower.contains("panic") || lower.contains("exception") {
        Category::Runtime
    } else {
        Category::Unknown
    };
    Some((if is_warning && !failure { Severity::Warning } else { Severity::Error }, category))
}

/// Stable normalization for grouping messages where line numbers, addresses
/// and long random IDs differ. It intentionally retains error codes and words.
fn normalized_key(text: &str) -> String {
    let mut key = text.trim().to_ascii_lowercase();
    static RULES: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();
    let rules = RULES.get_or_init(|| {
        [
            (r"0x[0-9a-f]{6,}", "<address>"),
            (r":[0-9]+:[0-9]+", ":<line>:<col>"),
            (r"\bline [0-9]+\b", "line <n>"),
            (r"\b[0-9]{6,}\b", "<number>"),
            (r"\s+", " "),
        ]
        .into_iter()
        .filter_map(|(pattern, replace)| Regex::new(pattern).ok().map(|regex| (regex, replace)))
        .collect()
    });
    for (regex, replacement) in rules {
        key = regex.replace_all(&key, *replacement).into_owned();
    }
    key
}

/// Deterministic, non-cryptographic FNV-1a 64-bit. Not for integrity checks.
fn fingerprint(key: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in key.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("fnv1a64:{hash:016x}")
}

/// Analyze a bounded text input without invoking tools, running commands or
/// importing referenced files. All emitted messages are best-effort redacted.
///
/// Raw input is discarded after this call. The caller must impose an input byte
/// limit and must mark truncated_input if it stopped reading before EOF.
pub fn analyze_text(input: &str, requested: LogFormat, truncated_input: bool) -> LogAnalysis {
    let (sanitized, redaction_matches) = redact_text(input);
    let adapter = match requested {
        LogFormat::Auto => detect_format(&sanitized),
        explicit => explicit,
    };
    let mut findings: Vec<Finding> = Vec::new();
    let mut seen: BTreeMap<(String, Severity, Category), usize> = BTreeMap::new();
    let mut truncated_findings = false;
    let mut scanned_lines = 0usize;

    for (offset, line) in sanitized.lines().take(MAX_LINES).enumerate() {
        scanned_lines += 1;
        if let Some((severity, category)) = classify(line, adapter) {
            let msg = line.trim().trim_end_matches('\r');
            let normalized = normalized_key(msg);
            let stable = fingerprint(&normalized);
            // Fingerprint collision alone never merges different normalized lines.
            let key = (normalized, severity.clone(), category.clone());
            if let Some(existing_index) = seen.get(&key) {
                findings[*existing_index].occurrences += 1;
            } else if findings.len() < MAX_FINDINGS {
                let index = findings.len();
                seen.insert(key, index);
                findings.push(Finding {
                    line: offset + 1,
                    severity,
                    category,
                    adapter: adapter.as_str().to_owned(),
                    message: clip_chars(msg, MAX_MESSAGE_CHARS),
                    fingerprint: stable,
                    occurrences: 1,
                });
            } else {
                truncated_findings = true;
            }
        }
    }

    if sanitized.lines().count() > MAX_LINES {
        truncated_findings = true;
    }
    let mut warnings = vec![
        "Classifications and fingerprints are heuristic, not verified root causes.".to_owned(),
        "Automatic redaction is best-effort; inspect all output before sharing.".to_owned(),
    ];
    if truncated_input {
        warnings.push("Input byte cap reached: analysis does not cover the complete file.".into());
    }
    if truncated_findings {
        warnings.push("Analysis line/finding cap reached: results may be incomplete.".into());
    }
    LogAnalysis {
        schema_version: ANALYSIS_SCHEMA_VERSION.to_owned(),
        requested_format: requested.as_str().to_owned(),
        detected_format: adapter.as_str().to_owned(),
        scanned_lines,
        truncated_input,
        truncated_findings,
        redaction_matches,
        findings,
        warnings,
    }
}

fn escape_inline(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace(['\n', '\r', '|'], " ")
}

/// Human-readable, HTML-escaped Markdown; keep untrusted values out of raw HTML.
pub fn analysis_markdown(analysis: &LogAnalysis) -> String {
    let mut text = format!(
        "# ReproCrate log analysis\n\nSchema: {}  \nRequested format: {}  \nDetected format: {}  \nScanned lines: {}  \nTruncated input: {}  \nFindings: {}\n\n",
        escape_inline(&analysis.schema_version),
        escape_inline(&analysis.requested_format),
        escape_inline(&analysis.detected_format),
        analysis.scanned_lines,
        analysis.truncated_input,
        analysis.findings.len()
    );
    text.push_str("| Line | Severity | Category | Count | Fingerprint | Message |\n");
    text.push_str("| ---: | --- | --- | ---: | --- | --- |\n");
    for finding in &analysis.findings {
        text.push_str(&format!(
            "| {} | {:?} | {:?} | {} | {} | {} |\n",
            finding.line,
            finding.severity,
            finding.category,
            finding.occurrences,
            escape_inline(&finding.fingerprint),
            escape_inline(&finding.message)
        ));
    }
    text.push_str("\n## Limitations and review\n\n");
    for warning in &analysis.warnings {
        text.push_str(&format!("- {}\n", escape_inline(warning)));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_cargo_and_groups_line_noise() {
        let text = "error[E0308]: mismatched types at file.rs:12:4\nerror[E0308]: mismatched types at file.rs:99:5\nwarning: unused variable";
        let result = analyze_text(text, LogFormat::Auto, false);
        assert_eq!(result.detected_format, "cargo");
        assert_eq!(result.findings.len(), 2);
        assert_eq!(result.findings[0].occurrences, 2);
        assert_eq!(result.findings[0].category, Category::Compilation);
    }

    #[test]
    fn pytest_adapter_classifies_failures() {
        let result = analyze_text(
            "FAILED tests/test_math.py::test_sum - AssertionError: wrong value\nE   AssertionError: 1 != 2",
            LogFormat::Auto, false
        );
        assert_eq!(result.detected_format, "pytest");
        assert_eq!(result.findings[0].category, Category::Test);
    }

    #[test]
    fn jest_and_vitest_adapters_are_available() {
        let jest = analyze_text("Test Suites: 1 failed, 0 passed\nFAIL math.test.js", LogFormat::Auto, false);
        assert_eq!(jest.detected_format, "jest");
        let vitest = analyze_text(" RUN  v2.0.1 vitest\n❯ src/add.test.ts (1 failed)", LogFormat::Auto, false);
        assert_eq!(vitest.detected_format, "vitest");
    }

    #[test]
    fn never_echoes_recognized_secrets() {
        let result = analyze_text("Error: password=very-secret-token\nAuthorization: Bearer reallysecret", LogFormat::Generic, false);
        let json = serde_json::to_string(&result).expect("serialize");
        assert!(!json.contains("very-secret-token"));
        assert!(!json.contains("reallysecret"));
        assert!(result.redaction_matches > 0);
    }

    #[test]
    fn does_not_merge_different_messages() {
        let a = analyze_text("error: compilation failed\nerror: connection failed", LogFormat::Generic, false);
        assert_eq!(a.findings.len(), 2);
        assert_ne!(a.findings[0].fingerprint, a.findings[1].fingerprint);
    }

    #[test]
    fn output_is_bounded_and_escapes_html_and_table_syntax() {
        let text = (0..400).map(|i| format!("error: fail-{i} <script>|value")).collect::<Vec<_>>().join("\n");
        let analysis = analyze_text(&text, LogFormat::Generic, true);
        assert!(analysis.truncated_findings);
        assert!(analysis.truncated_input);
        assert!(analysis.findings.len() <= MAX_FINDINGS);
        let md = analysis_markdown(&analysis);
        assert!(!md.contains("<script>"));
        assert!(md.contains("&lt;script&gt;"));
    }

    #[test]
    fn fixture_does_not_classify_unrelated_normal_lines() {
        let result = analyze_text("all 10 examples passed\nsome normal debug output", LogFormat::Auto, false);
        assert!(result.findings.is_empty());
    }
}
