// SPDX-License-Identifier: EPL-2.0
mod log_command;

use clap::{Parser, Subcommand, ValueEnum};
use log_command::{run_analysis, FormatArg};
use reprocrate_core::{
    capture_command, inspect_environment, render_markdown, CaptureOptions, CaptureProfile, Report,
    SCHEMA_VERSION,
};
use std::error::Error;
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Parser)]
#[command(
    name = "reprocrate",
    version,
    about = "Turn bugs into reproducible reports."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a command WITHOUT a shell, then save a local, sanitized report.
    Capture {
        /// JSON report destination (will not overwrite an existing file).
        #[arg(long, default_value = "reprocrate-report.json")]
        output: PathBuf,
        /// Optional Markdown report destination.
        #[arg(long)]
        markdown: Option<PathBuf>,
        /// Execution timeout in seconds (1..=86400).
        #[arg(long, default_value_t = 30)]
        timeout_seconds: u64,
        /// Output byte budget per stream (1..=10485760).
        #[arg(long, default_value_t = 65_536)]
        max_output_bytes: usize,
        #[arg(long, value_enum, default_value_t = ProfileArg::Standard)]
        profile: ProfileArg,
        /// Optional human-readable title.
        #[arg(long)]
        title: Option<String>,
        /// Explicit command and arguments. Precede them with --.
        #[arg(last = true, num_args = 1.., required = true)]
        command: Vec<OsString>,
    },
    /// Analyze a selected log (or '-' for stdin) without executing its contents.
    Analyze {
        /// Path to textual log file, or '-' to read standard input.
        input: PathBuf,
        /// Heuristic format detector or an explicitly selected parser.
        #[arg(long, value_enum, default_value_t = FormatArg::Auto)]
        format: FormatArg,
        /// Destination for sanitized JSON analysis (no overwrite).
        #[arg(long, default_value = "reprocrate-analysis.json")]
        output: PathBuf,
        /// Optional sanitized Markdown analysis destination.
        #[arg(long)]
        markdown: Option<PathBuf>,
        /// Maximum bytes read from the selected log (1..=8388608).
        #[arg(long, default_value_t = 1_048_576)]
        max_input_bytes: usize,
    },
    /// Print the small amount of environment information captured by profiles.
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Display metadata from a bounded, valid ReproCrate JSON report.
    Inspect { path: PathBuf },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ProfileArg {
    Minimal,
    Standard,
}
impl From<ProfileArg> for CaptureProfile {
    fn from(value: ProfileArg) -> Self {
        match value {
            ProfileArg::Minimal => Self::Minimal,
            ProfileArg::Standard => Self::Standard,
        }
    }
}

fn create_new_report(path: &Path, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    if let Err(error) = file.write_all(bytes).and_then(|_| file.sync_all()) {
        // Do not leave behind a misleading successful-looking partial export.
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error.into());
    }
    Ok(())
}

fn import_json(path: &Path) -> Result<Report, Box<dyn Error>> {
    let file = File::open(path)?;
    if file.metadata()?.len() > 4 * 1024 * 1024 {
        return Err("report exceeds the 4 MiB import limit".into());
    }
    let mut bytes = Vec::new();
    file.take(4 * 1024 * 1024 + 1).read_to_end(&mut bytes)?;
    if bytes.len() > 4 * 1024 * 1024 {
        return Err("report exceeds the 4 MiB import limit".into());
    }
    let report: Report = serde_json::from_slice(&bytes)?;
    if report.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "unsupported schema {}; this CLI only reads {}",
            report.schema_version, SCHEMA_VERSION
        )
        .into());
    }
    Ok(report)
}

fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Analyze { input, format, output, markdown, max_input_bytes } => {
            run_analysis(input, format, output, markdown, max_input_bytes)?;
        }
        Commands::Doctor { json } => {
            let env = inspect_environment(CaptureProfile::Standard);
            if json {
                println!("{}", serde_json::to_string_pretty(&env)?);
            } else {
                println!("ReproCrate {}", env!("CARGO_PKG_VERSION"));
                println!("OS: {}", env.os);
                println!("Architecture: {}", env.architecture);
                println!(
                    "Available parallelism: {}",
                    env.available_parallelism.unwrap_or(0)
                );
                println!(
                    "No hostnames, environment variables, usernames or network IDs are collected."
                );
            }
        }
        Commands::Inspect { path } => {
            let report = import_json(&path)?;
            println!("Report: {}", report.report_id);
            println!("Title: {}", report.title);
            println!("Schema: {}", report.schema_version);
            println!("Program: {}", report.capture.executable);
            println!(
                "Exit code: {}",
                report
                    .capture
                    .exit_code
                    .map(|n| n.to_string())
                    .unwrap_or_else(|| "unavailable".into())
            );
            println!("Duration: {} ms", report.capture.duration_ms);
            println!("Timed out: {}", report.capture.timed_out);
            println!("Warnings: {}", report.warnings.len());
            println!("Inspect the original report before sharing.");
        }
        Commands::Capture {
            output,
            markdown,
            timeout_seconds,
            max_output_bytes,
            profile,
            title,
            command,
        } => {
            if markdown.as_deref() == Some(output.as_path()) {
                return Err("--output and --markdown must be different paths".into());
            }
            if output.exists() || markdown.as_ref().is_some_and(|path| path.exists()) {
                return Err(
                    "an output file already exists; choose a new name (overwrite is disabled)"
                        .into(),
                );
            }
            let options = CaptureOptions {
                timeout: Duration::from_secs(timeout_seconds),
                max_output_bytes_per_stream: max_output_bytes,
            };
            let captured = capture_command(&command, &options)?;
            let title = title.unwrap_or_else(|| format!("Capture of {}", captured.executable));
            let report = Report::new(title, inspect_environment(profile.into()), captured);
            let json = serde_json::to_vec_pretty(&report)?;
            create_new_report(&output, &json)?;
            if let Some(md_path) = markdown {
                create_new_report(&md_path, render_markdown(&report).as_bytes())?;
                println!("Markdown report: {}", md_path.display());
            }
            println!("JSON report: {}", output.display());
            println!("Exit code: {:?}", report.capture.exit_code);
            println!("Timed out: {}", report.capture.timed_out);
            println!("Redaction pattern matches: {}", report.redaction_matches);
            eprintln!(
                "Review every field and log excerpt before sharing. Masking is not guaranteed."
            );
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("reprocrate: {error}");
        std::process::exit(2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_syntax_with_separator_parses() {
        let parsed = Cli::try_parse_from(["reprocrate", "capture", "--", "cargo", "test"])
            .expect("valid capture syntax");
        match parsed.command {
            Commands::Capture { command, .. } => {
                assert_eq!(
                    command,
                    vec![OsString::from("cargo"), OsString::from("test")]
                );
            }
            _ => panic!("unexpected subcommand"),
        }
    }

    #[test]
    fn invalid_capture_omitting_program_is_rejected() {
        assert!(Cli::try_parse_from(["reprocrate", "capture", "--"]).is_err());
    }
}
