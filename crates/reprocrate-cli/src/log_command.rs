// SPDX-License-Identifier: EPL-2.0
//! A bounded, inert log-file analysis command.
use crate::create_new_report;
use clap::ValueEnum;
use reprocrate_core::analysis::MAX_LOG_BYTES;
use reprocrate_core::{analysis_markdown, analyze_text, LogFormat};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum FormatArg {
    Auto,
    Generic,
    Cargo,
    Pytest,
    Jest,
    Vitest,
}

impl From<FormatArg> for LogFormat {
    fn from(value: FormatArg) -> Self {
        match value {
            FormatArg::Auto => Self::Auto,
            FormatArg::Generic => Self::Generic,
            FormatArg::Cargo => Self::Cargo,
            FormatArg::Pytest => Self::Pytest,
            FormatArg::Jest => Self::Jest,
            FormatArg::Vitest => Self::Vitest,
        }
    }
}

/// Do not use unbounded read_to_end, including for pipes and FIFOs.
fn read_bounded(path: &Path, limit: usize) -> io::Result<(String, bool)> {
    let mut bytes = Vec::new();
    let limit_plus_one = u64::try_from(limit.saturating_add(1)).unwrap_or(u64::MAX);
    if path.to_str() == Some("-") {
        io::stdin()
            .lock()
            .take(limit_plus_one)
            .read_to_end(&mut bytes)?;
    } else {
        File::open(path)?
            .take(limit_plus_one)
            .read_to_end(&mut bytes)?;
    }
    let truncated = bytes.len() > limit;
    bytes.truncate(limit);
    Ok((String::from_utf8_lossy(&bytes).into_owned(), truncated))
}

pub fn run_analysis(
    input: PathBuf,
    format: FormatArg,
    output: PathBuf,
    markdown: Option<PathBuf>,
    max_input_bytes: usize,
) -> Result<(), Box<dyn Error>> {
    if !(1..=MAX_LOG_BYTES).contains(&max_input_bytes) {
        return Err(format!("--max-input-bytes must be in the range 1..={MAX_LOG_BYTES}").into());
    }
    if markdown.as_deref() == Some(output.as_path()) {
        return Err("--output and --markdown must be different paths".into());
    }
    if output.exists() || markdown.as_ref().is_some_and(|path| path.exists()) {
        return Err("an output file already exists; choose a new name".into());
    }
    if input != Path::new("-") && !fs::metadata(&input)?.is_file() {
        return Err("--input must be a regular file or '-' for standard input".into());
    }
    let (text, truncated) = read_bounded(&input, max_input_bytes)?;
    let result = analyze_text(&text, format.into(), truncated);
    let json = serde_json::to_vec_pretty(&result)?;
    create_new_report(&output, &json)?;
    if let Some(md_path) = markdown {
        create_new_report(&md_path, analysis_markdown(&result).as_bytes())?;
        println!("Markdown analysis: {}", md_path.display());
    }
    println!("JSON analysis: {}", output.display());
    println!(
        "Adapter: {}, findings: {}, lines scanned: {}",
        result.detected_format,
        result.findings.len(),
        result.scanned_lines
    );
    if result.truncated_input || result.truncated_findings {
        eprintln!("Note: analysis limits were reached; results are incomplete.");
    }
    eprintln!("Review complete exported files for private information before sharing.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_size_cap_before_open() {
        let result = run_analysis(
            PathBuf::from("nonexistent.log"),
            FormatArg::Auto,
            PathBuf::from("no-export.json"),
            None,
            0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn refuses_identical_output_paths() {
        let result = run_analysis(
            PathBuf::from("-"),
            FormatArg::Auto,
            PathBuf::from("same.json"),
            Some(PathBuf::from("same.json")),
            512,
        );
        assert!(result.is_err());
    }
}
