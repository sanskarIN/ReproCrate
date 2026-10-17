// SPDX-License-Identifier: EPL-2.0
//! Shared domain, capture, report and privacy primitives for ReproCrate.

pub mod capture;
pub mod environment;
pub mod redaction;
pub mod report;

pub use capture::{capture_command, CaptureOptions};
pub use environment::{inspect_environment, CaptureProfile, Environment};
pub use report::{render_markdown, Capture, Report, SCHEMA_VERSION};

/// The version of this workspace's report generator.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
