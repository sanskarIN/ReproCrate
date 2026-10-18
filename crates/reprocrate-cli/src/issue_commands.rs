// SPDX-License-Identifier: EPL-2.0
//! Explicit local Markdown export. Adapters never contact issue trackers.

use crate::{create_new_report, import_json};
use reprocrate_core::{draft_issue, IssueOptions, IssueTarget};
use std::error::Error;
use std::path::PathBuf;

pub fn run_issue_export(
    report: PathBuf,
    output: PathBuf,
    target: IssueTarget,
    options: IssueOptions,
) -> Result<(), Box<dyn Error>> {
    if report == output {
        return Err("The source report and issue draft cannot share a path.".into());
    }
    if output.exists() {
        return Err("The output already exists; overwriting is prohibited.".into());
    }
    let imported = import_json(&report)?;
    let draft = draft_issue(&imported, target, options)?;
    create_new_report(&output, draft.markdown.as_bytes())?;
    println!("Local {} issue draft: {}", target.label(), output.display());
    eprintln!("No issue was posted. Examine every byte of the Markdown before copying it.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_and_output_paths_must_differ() {
        let error = run_issue_export(
            PathBuf::from("report.json"),
            PathBuf::from("report.json"),
            IssueTarget::Github,
            IssueOptions::default(),
        );
        assert!(error.is_err());
    }
}
