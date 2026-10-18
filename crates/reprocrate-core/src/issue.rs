// SPDX-License-Identifier: EPL-2.0
//! Offline issue-body adapters. No remote API, credentials, or uploaded evidence.
//! Detailed observations are deliberately omitted by default.

use crate::redaction::redact_text;
use crate::report::{Report, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueTarget {
    Github,
    Gitlab,
    Gitea,
    Generic,
}

impl IssueTarget {
    pub fn label(self) -> &'static str {
        match self {
            Self::Github => "GitHub",
            Self::Gitlab => "GitLab",
            Self::Gitea => "Gitea",
            Self::Generic => "Generic Markdown",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueOptions {
    pub include_title: bool,
    pub include_environment: bool,
    pub include_categories: bool,
    pub acknowledge_reviewed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssueDraft {
    pub target: IssueTarget,
    pub suggested_title: String,
    pub markdown: String,
    pub warnings: Vec<String>,
}

/// Escape untrusted metadata as *literal inline text*. Do not emit Markdown
/// links, HTML, directives, heading syntax or multiline text from source data.
fn safe_inline(value: &str, max_chars: usize) -> String {
    let clean = redact_text(value).0;
    let compact: String = clean
        .chars()
        .filter(|c| !c.is_control())
        .take(max_chars)
        .collect();
    let mut result = String::with_capacity(compact.len());
    for character in compact.chars() {
        if matches!(character, '<' | '>' | '&') {
            result.push_str(match character {
                '<' => "&lt;",
                '>' => "&gt;",
                _ => "&amp;",
            });
        } else {
            if matches!(
                character,
                '\\' | '`' | '*' | '_' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '!' | '|' | '~'
            ) {
                result.push('\\');
            }
            result.push(character);
        }
    }
    result
}

/// Build a bounded issue template containing safe structural observations.
/// Optional report metadata requires a separate affirmative review gate.
pub fn draft_issue(
    report: &Report,
    target: IssueTarget,
    options: IssueOptions,
) -> Result<IssueDraft, String> {
    if report.schema_version != SCHEMA_VERSION {
        return Err("Unsupported diagnostic report schema.".into());
    }
    if (options.include_title || options.include_environment || options.include_categories)
        && !options.acknowledge_reviewed
    {
        return Err("Optional issue details require explicit review acknowledgment.".into());
    }
    let proposed = if options.include_title {
        let title = safe_inline(&report.title, 120);
        if title.trim().is_empty() {
            "Bug report (review title)".to_owned()
        } else {
            title
        }
    } else {
        "Bug report (review title)".to_owned()
    };
    let status = if report.capture.timed_out {
        "Timed out"
    } else if report.capture.launch_error.is_some() {
        "Launch failed"
    } else if report.capture.exit_code.is_some_and(|code| code != 0) {
        "Command exited unsuccessfully"
    } else if report.capture.exit_code == Some(0) {
        "Command completed successfully"
    } else {
        "Unknown"
    };
    let mut md = format!(
        "# Bug report draft for {}\n\n**Suggested title:** {}\n\n## Observed behavior\n\n- Status: {}\n- Output truncated: {}\n\n",
        target.label(),
        proposed,
        status,
        if report.capture.stdout_truncated || report.capture.stderr_truncated {
            "yes"
        } else {
            "no"
        },
    );
    if options.include_environment {
        md.push_str("## Reviewed environment metadata\n\n");
        md.push_str(&format!(
            "- OS: {}\n- Architecture: {}\n\n",
            safe_inline(&report.environment.os, 80),
            safe_inline(&report.environment.architecture, 80),
        ));
    }
    if options.include_categories {
        md.push_str("## Heuristic failure categories (not root causes)\n\n");
        if let Some(analysis) = &report.analysis {
            for finding in analysis.findings.iter().take(10) {
                md.push_str(&format!(
                    "- {:?} ({:?}), {} occurrence(s)\n",
                    finding.category,
                    finding.severity,
                    finding.occurrences.min(1_000)
                ));
            }
            if analysis.findings.len() > 10 || analysis.truncated_findings {
                md.push_str("- Additional findings omitted or truncated\n");
            }
            if analysis.findings.is_empty() {
                md.push_str("- No categorized findings recorded\n");
            }
        } else {
            md.push_str("- No analysis available\n");
        }
        md.push('\n');
    }
    md.push_str("## Steps to reproduce (complete manually)\n\n");
    md.push_str("- [ ] Describe the minimal trusted steps\n- [ ] Describe the expected behavior\n- [ ] Describe the actual behavior\n\n");
    md.push_str("## Review before posting\n\n- [ ] Read the entire issue draft for sensitive data\n- [ ] Confirm the issue belongs in the selected project\n- [ ] Add only evidence approved for sharing\n\n");
    md.push_str("> Offline template only. No issue was created or uploaded. Command arguments, stdout, stderr, paths and raw finding messages were never included.\n");
    if md.len() > 16 * 1024 {
        return Err("Issue draft unexpectedly exceeded its size limit.".into());
    }
    Ok(IssueDraft {
        target,
        suggested_title: proposed,
        markdown: md,
        warnings: vec![
            "Review the complete Markdown before sharing; best-effort masking cannot certify privacy.".into(),
            "No remote API, automatic submission, or command execution occurs.".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::{inspect_environment, CaptureProfile};
    use crate::report::Capture;

    fn fixture() -> Report {
        Report::new(
            "<script> # dangerous [label](https://x) password=abc".into(),
            inspect_environment(CaptureProfile::Minimal),
            Capture {
                executable: "my-app".into(),
                arguments: vec!["--token".into(), "secret-here".into()],
                started_unix_ms: 0,
                duration_ms: 1,
                exit_code: Some(1),
                termination_signal: None,
                timed_out: false,
                launch_error: None,
                stdout: "raw PRIVATE evidence".into(),
                stderr: "raw PRIVATE error".into(),
                stdout_truncated: false,
                stderr_truncated: false,
                max_output_bytes_per_stream: 1024,
            },
        )
    }

    #[test]
    fn default_issue_is_minimal_and_private_for_all_targets() {
        let sample = fixture();
        for target in [
            IssueTarget::Github,
            IssueTarget::Gitlab,
            IssueTarget::Gitea,
            IssueTarget::Generic,
        ] {
            let draft = draft_issue(&sample, target, IssueOptions::default()).unwrap();
            assert!(!draft.markdown.contains("PRIVATE"));
            assert!(!draft.markdown.contains("secret-here"));
            assert!(!draft.markdown.contains("<script>"));
            assert!(!draft.markdown.contains("dangerous"));
            assert!(draft.markdown.contains("No issue was created or uploaded"));
        }
    }

    #[test]
    fn optionally_included_title_is_escaped_and_re_redacted() {
        let sample = fixture();
        let opts = IssueOptions {
            include_title: true,
            acknowledge_reviewed: true,
            ..IssueOptions::default()
        };
        let draft = draft_issue(&sample, IssueTarget::Github, opts).unwrap();
        assert!(!draft.markdown.contains("<script>"));
        assert!(!draft.markdown.contains("password=abc"));
        assert!(draft.markdown.contains("&lt;script&gt;"));
        assert!(draft.markdown.contains("\\# dangerous"));
        assert!(draft.markdown.contains("\\[label\\]"));
    }

    #[test]
    fn requires_approval_for_any_optional_metadata() {
        let sample = fixture();
        for opts in [
            IssueOptions {
                include_title: true,
                ..IssueOptions::default()
            },
            IssueOptions {
                include_environment: true,
                ..IssueOptions::default()
            },
            IssueOptions {
                include_categories: true,
                ..IssueOptions::default()
            },
        ] {
            assert!(draft_issue(&sample, IssueTarget::Github, opts).is_err());
        }
    }

    #[test]
    fn rejects_unsupported_report_schema() {
        let mut sample = fixture();
        sample.schema_version = "future".into();
        assert!(draft_issue(&sample, IssueTarget::Generic, IssueOptions::default()).is_err());
    }
}
