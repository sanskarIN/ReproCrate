// SPDX-License-Identifier: EPL-2.0
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// v0.1.0 only collects high-level non-identifying system facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureProfile {
    Minimal,
    Standard,
}

impl CaptureProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Standard => "standard",
        }
    }
}

impl FromStr for CaptureProfile {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "minimal" => Ok(Self::Minimal),
            "standard" => Ok(Self::Standard),
            _ => Err("profile must be minimal or standard"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Environment {
    pub capture_profile: String,
    pub os: String,
    pub architecture: String,
    /// Only collected in the standard profile.
    pub available_parallelism: Option<usize>,
}

pub fn inspect_environment(profile: CaptureProfile) -> Environment {
    Environment {
        capture_profile: profile.as_str().to_owned(),
        os: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
        available_parallelism: match profile {
            CaptureProfile::Minimal => None,
            CaptureProfile::Standard => std::thread::available_parallelism().ok().map(|n| n.get()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minimal_omits_processor_count() {
        let env = inspect_environment(CaptureProfile::Minimal);
        assert_eq!(env.available_parallelism, None);
    }
    #[test]
    fn unknown_profiles_are_not_silently_promoted() {
        assert!("advanced".parse::<CaptureProfile>().is_err());
    }
}
