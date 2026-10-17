// SPDX-License-Identifier: EPL-2.0
//! Review and edit all user-controlled fields before an explicit export.
//! Automatic detection is best-effort; it cannot certify secret-free content.

use crate::redaction::{redact_args, redact_text};
use crate::report::Report;

#[derive(Debug, Default, Clone)]
pub struct ReviewChoices {
    pub exclude_stdout: bool,
    pub exclude_stderr: bool,
    pub exclude_arguments: bool,
    pub exclude_executable: bool,
    pub exclude_analysis: bool,
    /// Literal strings loaded from a user-selected text file, never CLI args.
    pub additional_literals: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ReviewSummary {
    pub additional_mask_matches: usize,
    pub excluded_fields: Vec<String>,
}

fn mask_text(input: &str, literals: &[String]) -> (String, usize) {
    let (mut text, mut count) = redact_text(input);
    for needle in literals {
        if !needle.is_empty() {
            count = count.saturating_add(text.matches(needle.as_str()).count());
            text = text.replace(needle, "[REDACTED BY USER]");
        }
    }
    (text, count)
}

/// Returns an *edited copy*, not a claim of guaranteed sanitization.
/// The caller must preview/review the exact bytes prior to sharing.
pub fn review_report(source: &Report, choices: &ReviewChoices) -> (Report, ReviewSummary) {
    let mut report = source.clone();
    let mut summary = ReviewSummary::default();
    let mask = |value: &mut String, summary: &mut ReviewSummary| {
        let (sanitized, count) = mask_text(value, &choices.additional_literals);
        *value = sanitized;
        summary.additional_mask_matches = summary.additional_mask_matches.saturating_add(count);
    };

    mask(&mut report.title, &mut summary);
    mask(&mut report.report_id, &mut summary);
    mask(&mut report.environment.os, &mut summary);
    mask(&mut report.environment.architecture, &mut summary);
    mask(&mut report.environment.capture_profile, &mut summary);
    mask(&mut report.capture.executable, &mut summary);
    for argument in &mut report.capture.arguments {
        mask(argument, &mut summary);
    }
    // Also mask adjacent flag/value pairs; redacting each argument by itself
    // would not hide --token SECRETVALUE otherwise.
    let (arguments, pairs) = redact_args(&report.capture.arguments);
    report.capture.arguments = arguments;
    summary.additional_mask_matches = summary.additional_mask_matches.saturating_add(pairs);
    mask(&mut report.capture.stdout, &mut summary);
    mask(&mut report.capture.stderr, &mut summary);
    if let Some(error) = &mut report.capture.launch_error {
        mask(error, &mut summary);
    }
    for warning in &mut report.warnings {
        mask(warning, &mut summary);
    }
    if let Some(analysis) = &mut report.analysis {
        mask(&mut analysis.requested_format, &mut summary);
        mask(&mut analysis.detected_format, &mut summary);
        for finding in &mut analysis.findings {
            mask(&mut finding.adapter, &mut summary);
            mask(&mut finding.message, &mut summary);
            mask(&mut finding.fingerprint, &mut summary);
        }
        for warning in &mut analysis.warnings {
            mask(warning, &mut summary);
        }
    }

    if choices.exclude_stdout {
        report.capture.stdout.clear();
        summary.excluded_fields.push("capture.stdout".into());
    }
    if choices.exclude_stderr {
        report.capture.stderr.clear();
        summary.excluded_fields.push("capture.stderr".into());
    }
    if choices.exclude_arguments {
        report.capture.arguments.clear();
        summary.excluded_fields.push("capture.arguments".into());
    }
    if choices.exclude_executable {
        report.capture.executable = "[EXCLUDED BY USER]".into();
        summary.excluded_fields.push("capture.executable".into());
    }
    if choices.exclude_analysis {
        report.analysis = None;
        summary.excluded_fields.push("analysis".into());
    }
    if !summary.excluded_fields.is_empty() || summary.additional_mask_matches > 0 {
        report.warnings.push("Additional exclusions or masks were applied by the user during review.".into());
    }
    report.warnings.push("This is an editable review candidate, not proof of a secret-free report.".into());
    report.redaction_matches = report.redaction_matches.saturating_add(summary.additional_mask_matches);
    (report, summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::{inspect_environment, CaptureProfile}, report::Capture};

    fn sample() -> Report {
        Report::new(
            "test report".into(),
            inspect_environment(CaptureProfile::Minimal),
            Capture {
                executable: "cargo".into(),
                arguments: vec!["--token".into(), "unrecognized-secret-value".into()],
                started_unix_ms: 0, duration_ms: 0,
                exit_code: None, termination_signal: None, timed_out: false,
                launch_error: None,
                stdout: "internal project private-token goes here".into(),
                stderr: "error: permission denied".into(),
                stdout_truncated: false, stderr_truncated: false,
                max_output_bytes_per_stream: 1024,
            },
        )
    }

    #[test]
    fn masks_selected_literal_without_echoing_it() {
        let input = sample();
        let opts = ReviewChoices {
            additional_literals: vec!["private-token".into()],
            ..ReviewChoices::default()
        };
        let (report, summary) = review_report(&input, &opts);
        assert!(summary.additional_mask_matches >= 1);
        assert!(!report.capture.stdout.contains("private-token"));
        assert_eq!(input.capture.stdout, "internal project private-token goes here");
    }

    #[test]
    fn removes_requested_fields_and_repeated_args() {
        let input = sample();
        let opts = ReviewChoices {
            exclude_stdout: true, exclude_stderr: true,
            exclude_arguments: true, exclude_analysis: true,
            ..ReviewChoices::default()
        };
        let (report, result) = review_report(&input, &opts);
        assert_eq!(result.excluded_fields.len(), 4);
        assert!(report.capture.stdout.is_empty());
        assert!(report.capture.arguments.is_empty());
        assert!(report.analysis.is_none());
    }
}
