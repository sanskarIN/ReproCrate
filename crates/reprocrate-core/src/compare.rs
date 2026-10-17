// SPDX-License-Identifier: EPL-2.0
//! Deterministic, privacy-minimizing comparisons of inert ReproCrate reports.
//! The exported comparison never contains raw log lines or argument values.

use crate::report::{Report, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const COMPARISON_SCHEMA_VERSION: &str = "1.0.0";
const MAX_COMPARABLE_FINDINGS: usize = 200;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObservationState {
    Present,
    Missing,
    Redacted,
    Incomplete,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    Same,
    Changed,
    Added,
    Removed,
    Indeterminate,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FieldDifference {
    pub field: String,
    pub baseline_state: ObservationState,
    pub candidate_state: ObservationState,
    pub relationship: Relationship,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FingerprintDifference {
    /// Non-cryptographic grouping ID; two different errors can collide.
    pub fingerprint: String,
    pub baseline_count: usize,
    pub candidate_count: usize,
    pub relationship: Relationship,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReportComparison {
    pub schema_version: String,
    pub baseline_report_id: String,
    pub candidate_report_id: String,
    pub fields: Vec<FieldDifference>,
    pub fingerprint_changes: Vec<FingerprintDifference>,
    pub fingerprint_comparison_complete: bool,
    pub warnings: Vec<String>,
}

struct Observation {
    state: ObservationState,
    value: Option<String>,
}

fn resembles_mask(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    lowered.contains("[redacted")
        || lowered.contains("[excluded")
        || lowered.contains("[masked")
}

fn observe(value: Option<String>, incomplete: bool) -> Observation {
    let state = match value.as_deref() {
        _ if incomplete => ObservationState::Incomplete,
        Some(s) if resembles_mask(s) => ObservationState::Redacted,
        Some(s) if !s.trim().is_empty() => ObservationState::Present,
        _ => ObservationState::Missing,
    };
    Observation { state, value }
}

fn compare_field(name: &str, left: Observation, right: Observation) -> FieldDifference {
    use ObservationState::*;
    let relation = match (&left.state, &right.state) {
        (Incomplete, _) | (_, Incomplete) | (Redacted, _) | (_, Redacted) => {
            Relationship::Indeterminate
        }
        (Missing, Missing) => Relationship::Same,
        (Missing, Present) => Relationship::Added,
        (Present, Missing) => Relationship::Removed,
        (Present, Present) if left.value == right.value => Relationship::Same,
        (Present, Present) => Relationship::Changed,
    };
    FieldDifference {
        field: name.to_owned(),
        baseline_state: left.state,
        candidate_state: right.state,
        relationship: relation,
    }
}

fn add_field(
    result: &mut Vec<FieldDifference>,
    name: &str,
    baseline: Option<String>,
    candidate: Option<String>,
    left_incomplete: bool,
    right_incomplete: bool,
) {
    result.push(compare_field(
        name,
        observe(baseline, left_incomplete),
        observe(candidate, right_incomplete),
    ));
}

fn string_option<T: ToString>(value: Option<T>) -> Option<String> {
    value.map(|v| v.to_string())
}

fn findings_map(report: &Report) -> BTreeMap<String, usize> {
    let mut map = BTreeMap::new();
    if let Some(analysis) = &report.analysis {
        for finding in analysis.findings.iter().take(MAX_COMPARABLE_FINDINGS) {
            // Fingerprints must be a short, stable, non-secret grouping ID.
            if finding.fingerprint.len() != 24
                || !finding.fingerprint.starts_with("fnv1a64:")
                || !finding.fingerprint[8..].bytes().all(|b| b.is_ascii_hexdigit())
            {
                continue;
            }
            let key = format!(
                "{}:{:?}:{:?}",
                finding.fingerprint, finding.severity, finding.category
            );
            let count = map.entry(key).or_insert(0usize);
            *count = count.saturating_add(finding.occurrences);
        }
    }
    map
}

/// Performs a *structural* comparison. Logs/arguments are compared for
/// equality without exporting raw contents, and masked or truncated data is
/// explicitly indeterminate. The caller must validate both input sizes.
pub fn compare_reports(baseline: &Report, candidate: &Report) -> Result<ReportComparison, String> {
    if baseline.schema_version != SCHEMA_VERSION || candidate.schema_version != SCHEMA_VERSION {
        return Err("unsupported report schema; comparison requires schema 1.0.0".into());
    }
    let mut fields = Vec::new();
    add_field(&mut fields, "environment.capture_profile",
        Some(baseline.environment.capture_profile.clone()),
        Some(candidate.environment.capture_profile.clone()), false, false);
    add_field(&mut fields, "environment.os",
        Some(baseline.environment.os.clone()),
        Some(candidate.environment.os.clone()), false, false);
    add_field(&mut fields, "environment.architecture",
        Some(baseline.environment.architecture.clone()),
        Some(candidate.environment.architecture.clone()), false, false);
    add_field(&mut fields, "environment.available_parallelism",
        string_option(baseline.environment.available_parallelism),
        string_option(candidate.environment.available_parallelism), false, false);
    add_field(&mut fields, "capture.executable",
        Some(baseline.capture.executable.clone()), Some(candidate.capture.executable.clone()), false, false);
    add_field(&mut fields, "capture.arguments",
        if baseline.capture.arguments.is_empty() { None } else { Some(baseline.capture.arguments.join("\0")) },
        if candidate.capture.arguments.is_empty() { None } else { Some(candidate.capture.arguments.join("\0")) },
        false, false);
    add_field(&mut fields, "capture.exit_code",
        string_option(baseline.capture.exit_code), string_option(candidate.capture.exit_code), false, false);
    add_field(&mut fields, "capture.termination_signal",
        string_option(baseline.capture.termination_signal), string_option(candidate.capture.termination_signal),
        false, false);
    add_field(&mut fields, "capture.timed_out",
        Some(baseline.capture.timed_out.to_string()), Some(candidate.capture.timed_out.to_string()), false, false);
    add_field(&mut fields, "capture.duration_ms",
        Some(baseline.capture.duration_ms.to_string()),
        Some(candidate.capture.duration_ms.to_string()), false, false);
    add_field(&mut fields, "capture.launch_error",
        baseline.capture.launch_error.clone(), candidate.capture.launch_error.clone(), false, false);
    add_field(&mut fields, "capture.stdout",
        Some(baseline.capture.stdout.clone()), Some(candidate.capture.stdout.clone()),
        baseline.capture.stdout_truncated, candidate.capture.stdout_truncated);
    add_field(&mut fields, "capture.stderr",
        Some(baseline.capture.stderr.clone()), Some(candidate.capture.stderr.clone()),
        baseline.capture.stderr_truncated, candidate.capture.stderr_truncated);
    add_field(&mut fields, "capture.stdout_truncated",
        Some(baseline.capture.stdout_truncated.to_string()),
        Some(candidate.capture.stdout_truncated.to_string()), false, false);
    add_field(&mut fields, "capture.stderr_truncated",
        Some(baseline.capture.stderr_truncated.to_string()),
        Some(candidate.capture.stderr_truncated.to_string()), false, false);

    let mut warnings = vec![
        "Structural comparisons do not prove that a defect is reproducible.".to_owned(),
        "Values are deliberately omitted from the comparison; inspect source reports privately.".to_owned(),
        "Redacted or truncated values are inconclusive, not equal or different.".to_owned(),
        "Fingerprints are non-cryptographic heuristics and can collide.".to_owned(),
    ];
    let complete = baseline.analysis.as_ref().is_some_and(|analysis| {
        !analysis.truncated_findings && !analysis.truncated_input && analysis.findings.len() <= MAX_COMPARABLE_FINDINGS
    }) && candidate.analysis.as_ref().is_some_and(|analysis| {
        !analysis.truncated_findings && !analysis.truncated_input && analysis.findings.len() <= MAX_COMPARABLE_FINDINGS
    });
    let mut fingerprint_changes = Vec::new();
    if complete {
        let before = findings_map(baseline);
        let after = findings_map(candidate);
        let keys: BTreeSet<_> = before.keys().chain(after.keys()).cloned().collect();
        for key in keys {
            let left = *before.get(&key).unwrap_or(&0);
            let right = *after.get(&key).unwrap_or(&0);
            let relation = match (left, right) {
                (0, _) => Relationship::Added,
                (_, 0) => Relationship::Removed,
                (a, b) if a == b => Relationship::Same,
                _ => Relationship::Changed,
            };
            fingerprint_changes.push(FingerprintDifference {
                fingerprint: key,
                baseline_count: left,
                candidate_count: right,
                relationship: relation,
            });
        }
    } else {
        warnings.push("One or both reports lack complete analysis; no finding additions/removals are asserted.".into());
    }
    Ok(ReportComparison {
        schema_version: COMPARISON_SCHEMA_VERSION.to_owned(),
        baseline_report_id: baseline.report_id.clone(),
        candidate_report_id: candidate.report_id.clone(),
        fields,
        fingerprint_changes,
        fingerprint_comparison_complete: complete,
        warnings,
    })
}

fn inline(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace(['\r', '\n', '|'], " ")
}

pub fn comparison_markdown(comparison: &ReportComparison) -> String {
    let mut out = format!(
        "# ReproCrate report comparison\n\nBaseline: {}  \nCandidate: {}  \nSchema: {}\n\n",
        inline(&comparison.baseline_report_id),
        inline(&comparison.candidate_report_id),
        inline(&comparison.schema_version),
    );
    out.push_str("| Field | Baseline | Candidate | Relationship |\n| --- | --- | --- | --- |\n");
    for entry in &comparison.fields {
        out.push_str(&format!(
            "| {} | {:?} | {:?} | {:?} |\n",
            inline(&entry.field), entry.baseline_state, entry.candidate_state, entry.relationship,
        ));
    }
    out.push_str("\n## Heuristic failure fingerprints\n\n");
    if comparison.fingerprint_comparison_complete {
        out.push_str("| Fingerprint / classification | Before | After | Relationship |\n| --- | ---: | ---: | --- |\n");
        for item in &comparison.fingerprint_changes {
            out.push_str(&format!(
                "| {} | {} | {} | {:?} |\n",
                inline(&item.fingerprint), item.baseline_count, item.candidate_count, item.relationship,
            ));
        }
    } else {
        out.push_str("Incomplete/absent analysis: the fingerprint comparison was not attempted.\n");
    }
    out.push_str("\n## Limitations\n\n");
    for warning in &comparison.warnings {
        out.push_str(&format!("- {}\n", inline(warning)));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::{inspect_environment, CaptureProfile}, report::Capture};

    fn fixture() -> Report {
        Report::new("test".into(), inspect_environment(CaptureProfile::Minimal), Capture {
            executable: "test-runner".into(), arguments: vec!["--safe".into()],
            started_unix_ms: 0, duration_ms: 10, exit_code: Some(1),
            termination_signal: None, timed_out: false, launch_error: None,
            stdout: String::new(), stderr: "error[E0308]: mismatched types".into(),
            stdout_truncated: false, stderr_truncated: false, max_output_bytes_per_stream: 64,
        })
    }

    fn get_field<'a>(comparison: &'a ReportComparison, name: &str) -> &'a FieldDifference {
        comparison.fields.iter().find(|field| field.field == name).expect("field")
    }

    #[test]
    fn returns_inconclusive_for_redacted_and_truncated_values() {
        let before = fixture();
        let mut after = before.clone();
        after.capture.arguments = vec!["[REDACTED]".into()];
        after.capture.stderr_truncated = true;
        let result = compare_reports(&before, &after).unwrap();
        assert_eq!(get_field(&result, "capture.arguments").relationship, Relationship::Indeterminate);
        assert_eq!(get_field(&result, "capture.stderr").relationship, Relationship::Indeterminate);
        assert_eq!(get_field(&result, "capture.exit_code").relationship, Relationship::Same);
    }

    #[test]
    fn comparison_does_not_export_raw_logs_arguments_or_secret_tokens() {
        let mut before = fixture();
        let mut after = fixture();
        before.capture.stderr = "error: private-text-A".into();
        after.capture.stderr = "error: private-text-B".into();
        before.capture.arguments = vec!["nonpublicarg1".into()];
        after.capture.arguments = vec!["nonpublicarg2".into()];
        let json = serde_json::to_string(&compare_reports(&before, &after).unwrap()).unwrap();
        assert!(!json.contains("private-text"));
        assert!(!json.contains("nonpublicarg"));
        assert!(json.contains("changed"));
    }

    #[test]
    fn incomplete_findings_do_not_claim_resolutions() {
        let before = fixture();
        let mut after = fixture();
        after.analysis = None;
        let comparison = compare_reports(&before, &after).unwrap();
        assert!(!comparison.fingerprint_comparison_complete);
        assert!(comparison.fingerprint_changes.is_empty());
    }

    #[test]
    fn deterministic_fingerprint_counts_for_same_reports() {
        let before = fixture();
        let after = fixture();
        let a = compare_reports(&before, &after).unwrap();
        let b = compare_reports(&before, &after).unwrap();
        assert_eq!(a, b);
        assert!(a.fingerprint_comparison_complete);
    }

    #[test]
    fn escapes_untrusted_report_ids_in_markdown() {
        let before = fixture();
        let mut after = fixture();
        after.report_id = "<script>|oops".into();
        let markdown = comparison_markdown(&compare_reports(&before, &after).unwrap());
        assert!(!markdown.contains("<script>"));
        assert!(markdown.contains("&lt;script&gt;"));
    }
}
