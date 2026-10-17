// SPDX-License-Identifier: EPL-2.0
//! Structured, declarative, offline-only reproduction guides.
//! A recipe is inert evidence; it is never executed automatically.

use crate::redaction::{redact_args, redact_text};
use crate::report::{Report, SCHEMA_VERSION};
use serde::{Deserialize, Serialize};
use std::io;

pub const RECIPE_SCHEMA_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeStep {
    pub order: u8,
    pub kind: String,
    pub instruction: String,
    pub requires_user_action: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecipeCommand {
    /// Informational only; never automatically executed.
    pub executable: String,
    pub arguments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReproductionRecipe {
    pub schema_version: String,
    pub source_report_schema: String,
    pub source_report_id: String,
    pub source_reprocrate_version: String,
    pub environment_os: String,
    pub environment_architecture: String,
    pub environment_profile: String,
    pub expected_exit_code: Option<i32>,
    pub observed_timed_out: bool,
    pub expected_failure_fingerprints: Vec<String>,
    /// Omitted unless the exporter explicitly opts in.
    pub command: Option<RecipeCommand>,
    pub steps: Vec<RecipeStep>,
    pub automatic_execution: bool,
    pub warnings: Vec<String>,
}

fn scrub(value: &str) -> String {
    redact_text(value).0
}

/// Export an inert checklist from an already reviewed report.
/// Opt-in command snapshots need a separate CLI acknowledgement.
pub fn recipe_from_report(report: &Report, include_command: bool) -> Result<ReproductionRecipe, String> {
    if report.schema_version != SCHEMA_VERSION {
        return Err("unsupported report schema".into());
    }
    let command = if include_command {
        let (args, _) = redact_args(&report.capture.arguments);
        Some(RecipeCommand {
            executable: scrub(&report.capture.executable),
            arguments: args,
        })
    } else {
        None
    };
    let mut fingerprints = report.analysis.as_ref().map(|analysis| {
        analysis.findings.iter()
            .filter(|item| item.fingerprint.len() == 24
                && item.fingerprint.starts_with("fnv1a64:")
                && item.fingerprint[8..].bytes().all(|byte| byte.is_ascii_hexdigit()))
            .map(|item| item.fingerprint.clone())
            .take(200)
            .collect::<Vec<_>>()
    }).unwrap_or_default();
    fingerprints.sort();
    fingerprints.dedup();

    let steps = vec![
        RecipeStep {
            order: 1,
            kind: "review".into(),
            instruction: "Review this recipe and its source report locally; exclude credentials, secrets and private paths before sharing.".into(),
            requires_user_action: true,
        },
        RecipeStep {
            order: 2,
            kind: "prepare_environment".into(),
            instruction: "Prepare an isolated environment with required dependencies after independently assessing the command and associated risks.".into(),
            requires_user_action: true,
        },
        RecipeStep {
            order: 3,
            kind: "manual_reproduce".into(),
            instruction: "Only if trusted, manually reproduce the documented action; never execute a recipe directly or implicitly.".into(),
            requires_user_action: true,
        },
        RecipeStep {
            order: 4,
            kind: "capture".into(),
            instruction: "Record actual output, exit status and environmental differences with ReproCrate, then manually review the new report.".into(),
            requires_user_action: true,
        },
        RecipeStep {
            order: 5,
            kind: "compare".into(),
            instruction: "Compare old and new reports locally; fingerprint similarities are only heuristic signals, not proof of cause.".into(),
            requires_user_action: true,
        },
    ];
    Ok(ReproductionRecipe {
        schema_version: RECIPE_SCHEMA_VERSION.into(),
        source_report_schema: report.schema_version.clone(),
        source_report_id: scrub(&report.report_id),
        source_reprocrate_version: scrub(&report.reprocrate_version),
        environment_os: scrub(&report.environment.os),
        environment_architecture: scrub(&report.environment.architecture),
        environment_profile: scrub(&report.environment.capture_profile),
        expected_exit_code: report.capture.exit_code,
        observed_timed_out: report.capture.timed_out,
        expected_failure_fingerprints: fingerprints,
        command,
        steps,
        automatic_execution: false,
        warnings: vec![
            "The recipe is a human-reviewed plan, not executable automation.".into(),
            "Command capture is host execution without a sandbox; do not run untrusted commands.".into(),
            "Redaction is best-effort; even opted-in command arguments may expose sensitive data.".into(),
            "Environment details and fingerprints may be incomplete or inconclusive.".into(),
        ],
    })
}

pub fn validate_recipe(recipe: &ReproductionRecipe) -> Result<(), String> {
    if recipe.schema_version != RECIPE_SCHEMA_VERSION
        || recipe.source_report_schema != SCHEMA_VERSION {
        return Err("unsupported recipe or report schema version".into());
    }
    if recipe.automatic_execution || recipe.steps.len() != 5
        || recipe.steps.iter().enumerate().any(|(i, step)| {
            step.order != (i + 1) as u8 || !step.requires_user_action || step.instruction.len() > 2048
        }) {
        return Err("unsafe or unsupported recipe steps".into());
    }
    if recipe.expected_failure_fingerprints.len() > 200
        || recipe.command.as_ref().is_some_and(|command| {
            command.arguments.len() > 4096
                || command.executable.len() > 4096
                || command.arguments.iter().any(|arg| arg.len() > 4096)
        }) {
        return Err("recipe exceeds safety limits".into());
    }
    Ok(())
}

fn escape_inline(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        .replace(['\n', '\r', '|'], " ")
}

pub fn recipe_markdown(recipe: &ReproductionRecipe) -> io::Result<String> {
    validate_recipe(recipe).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let mut out = format!(
        "# ReproCrate manual reproduction guide\n\nSource report: {}  \nReport schema: {}  \nOS: {}  \nArchitecture: {}  \nProfile: {}\n\n",
        escape_inline(&recipe.source_report_id),
        escape_inline(&recipe.source_report_schema),
        escape_inline(&recipe.environment_os),
        escape_inline(&recipe.environment_architecture),
        escape_inline(&recipe.environment_profile),
    );
    if let Some(command) = &recipe.command {
        out.push_str("## User-approved command reference (informational only)\n\n");
        let json = serde_json::to_string(&(&command.executable, &command.arguments))
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
        out.push_str(&format!("Command and argv as JSON strings: {}\n\n",
            escape_inline(&json.replace(char::from(96), "&#96;"))));
    } else {
        out.push_str("**Command details are deliberately omitted.** Supply them manually only after an independent trust review.\n\n");
    }
    out.push_str("## Manual checklist\n\n");
    for step in &recipe.steps {
        out.push_str(&format!("{}. **{}** — {}\n",
            step.order, escape_inline(&step.kind), escape_inline(&step.instruction)));
    }
    out.push_str("\n## Limitations\n\n");
    for warning in &recipe.warnings {
        out.push_str(&format!("- {}\n", escape_inline(warning)));
    }
    out.push_str("\n**No command or imported recipe is executed automatically.**\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::{inspect_environment, CaptureProfile}, report::Capture};
    fn sample() -> Report {
        Report::new("fixture".into(), inspect_environment(CaptureProfile::Minimal), Capture {
            executable: "runner".into(), arguments: vec!["--token".into(), "secret".into()],
            started_unix_ms: 0, duration_ms: 10, exit_code: Some(1), termination_signal: None,
            timed_out: false, launch_error: None, stdout: "".into(), stderr: "error: failure".into(),
            stdout_truncated: false, stderr_truncated: false, max_output_bytes_per_stream: 64,
        })
    }

    #[test]
    fn default_excludes_command_and_logs() {
        let report = sample();
        let recipe = recipe_from_report(&report, false).unwrap();
        assert_eq!(recipe.command, None);
        let output = serde_json::to_string(&recipe).unwrap();
        assert!(!output.contains("runner"));
        assert!(!output.contains("--token"));
        assert!(!recipe.automatic_execution);
        validate_recipe(&recipe).unwrap();
    }

    #[test]
    fn explicit_command_inclusion_masks_password_flags() {
        let recipe = recipe_from_report(&sample(), true).unwrap();
        let cmd = recipe.command.as_ref().unwrap();
        assert_eq!(cmd.arguments[1], "[REDACTED]");
        assert!(!serde_json::to_string(&recipe).unwrap().contains("secret\""));
    }

    #[test]
    fn denies_automatic_execution_even_when_imported() {
        let mut recipe = recipe_from_report(&sample(), false).unwrap();
        recipe.automatic_execution = true;
        assert!(validate_recipe(&recipe).is_err());
        assert!(recipe_markdown(&recipe).is_err());
    }

    #[test]
    fn escapes_untrusted_report_identifiers() {
        let mut report = sample();
        report.report_id = "<script>bad|identifier".into();
        let recipe = recipe_from_report(&report, false).unwrap();
        let markdown = recipe_markdown(&recipe).unwrap();
        assert!(markdown.contains("&lt;script&gt;"));
        assert!(!markdown.contains("<script>"));
    }
}
