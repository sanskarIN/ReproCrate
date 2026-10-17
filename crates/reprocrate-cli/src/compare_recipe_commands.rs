// SPDX-License-Identifier: EPL-2.0
//! Offline comparisons and inert, manually reviewed reproduction guides.

use crate::{create_new_report, import_json};
use reprocrate_core::{
    compare_reports, comparison_markdown, recipe_from_report, recipe_markdown, validate_recipe,
    ReproductionRecipe,
};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

const MAX_RECIPE_BYTES: usize = 1024 * 1024;

fn check_outputs(output: &Path, markdown: Option<&Path>) -> Result<(), Box<dyn Error>> {
    if Some(output) == markdown {
        return Err("JSON and Markdown output paths must differ".into());
    }
    if output.exists() || markdown.is_some_and(Path::exists) {
        return Err("an output already exists; overwriting is prohibited".into());
    }
    Ok(())
}

pub fn run_compare(
    baseline: PathBuf,
    candidate: PathBuf,
    output: PathBuf,
    markdown: Option<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    check_outputs(&output, markdown.as_deref())?;
    let baseline_report = import_json(&baseline)?;
    let candidate_report = import_json(&candidate)?;
    let diff = compare_reports(&baseline_report, &candidate_report)?;
    let json = serde_json::to_vec_pretty(&diff)?;
    create_new_report(&output, &json)?;
    if let Some(path) = markdown {
        create_new_report(&path, comparison_markdown(&diff).as_bytes())?;
        println!("Markdown comparison: {}", path.display());
    }
    println!("JSON comparison: {}", output.display());
    let changed = diff
        .fields
        .iter()
        .filter(|field| {
            matches!(
                field.relationship,
                reprocrate_core::compare::Relationship::Changed
            )
        })
        .count();
    println!("Changed comparable fields: {changed}");
    println!(
        "Fingerprint comparison complete: {}",
        diff.fingerprint_comparison_complete
    );
    eprintln!("The structural differences omit raw values; no root cause was proven.");
    Ok(())
}

pub fn run_recipe_create(
    report: PathBuf,
    output: PathBuf,
    markdown: Option<PathBuf>,
    include_command: bool,
    acknowledge_reviewed: bool,
) -> Result<(), Box<dyn Error>> {
    check_outputs(&output, markdown.as_deref())?;
    if include_command && !acknowledge_reviewed {
        return Err("--include-command requires --acknowledge-reviewed (inspect the command and arguments first)".into());
    }
    let captured = import_json(&report)?;
    let recipe = recipe_from_report(&captured, include_command)?;
    validate_recipe(&recipe)?;
    let json = serde_json::to_vec_pretty(&recipe)?;
    if json.len() > MAX_RECIPE_BYTES {
        return Err("recipe exceeds the 1 MiB export limit".into());
    }
    create_new_report(&output, &json)?;
    if let Some(path) = markdown {
        create_new_report(&path, recipe_markdown(&recipe)?.as_bytes())?;
        println!("Markdown recipe: {}", path.display());
    }
    println!("JSON recipe: {}", output.display());
    println!("Command snapshot included: {}", recipe.command.is_some());
    eprintln!("This is a non-executable checklist. Manually evaluate any steps or commands.");
    Ok(())
}

pub fn run_recipe_inspect(input: PathBuf) -> Result<(), Box<dyn Error>> {
    let file = File::open(&input)?;
    if file.metadata()?.len() > MAX_RECIPE_BYTES as u64 {
        return Err("recipe exceeds 1 MiB import limit".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_RECIPE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() > MAX_RECIPE_BYTES {
        return Err("recipe exceeded import cap while reading".into());
    }
    let recipe: ReproductionRecipe = serde_json::from_slice(&bytes)?;
    validate_recipe(&recipe)?;
    println!("Recipe schema: {}", recipe.schema_version);
    println!("Source report schema: {}", recipe.source_report_schema);
    println!("Manual steps: {}", recipe.steps.len());
    println!("Command snapshot included: {}", recipe.command.is_some());
    println!("Automatic execution: {}", recipe.automatic_execution);
    println!("Validation: structural schema accepted; no instructions executed");
    eprintln!("This is not a security review or proof of reproducibility.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_snapshot_requires_manual_review_acknowledgment() {
        let err = run_recipe_create(
            PathBuf::from("not-read.json"),
            PathBuf::from("not-created.json"),
            None,
            true,
            false,
        );
        assert!(err.is_err());
    }

    #[test]
    fn output_paths_cannot_overlap() {
        let output = Path::new("x.json");
        assert!(check_outputs(output, Some(output)).is_err());
    }
}
