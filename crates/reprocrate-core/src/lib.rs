// SPDX-License-Identifier: EPL-2.0
//! Shared domain, capture, report and privacy primitives for ReproCrate.

pub mod analysis;
pub mod bundle;
pub mod capture;
pub mod compare;
pub mod recipe;
pub mod environment;
pub mod privacy;
pub mod redaction;
pub mod report;

pub use analysis::{
    analysis_markdown, analyze_text, LogAnalysis, LogFormat, ANALYSIS_SCHEMA_VERSION,
};
pub use bundle::{create_bundle, extract_bundle, verify_bundle, BundleManifest};
pub use compare::{compare_reports, comparison_markdown, ReportComparison};
pub use recipe::{recipe_from_report, recipe_markdown, validate_recipe, ReproductionRecipe};
pub use capture::{capture_command, CaptureOptions};
pub use environment::{inspect_environment, CaptureProfile, Environment};
pub use privacy::{review_report, ReviewChoices, ReviewSummary};
pub use report::{render_markdown, Capture, Report, SCHEMA_VERSION};

/// The version of this workspace's report generator.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
