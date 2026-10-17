// SPDX-License-Identifier: EPL-2.0
//! Best-effort, deliberately conservative sanitization.
//! A redacted export is NOT guaranteed to be secret-free.
use regex::Regex;

/// Returns sanitized text and a count of matched patterns (not unique secrets).
pub fn redact_text(value: &str) -> (String, usize) {
    let mut text = value.to_owned();
    let mut matches = 0;
    let rules: [(&str, &str); 6] = [
        (r"(?i)(authorization\s*:\s*bearer\s+)[A-Za-z0-9._~+/-]+={0,2}", "${1}[REDACTED]"),
        (r"(?i)(bearer\s+)[A-Za-z0-9._~+/-]+={0,2}", "${1}[REDACTED]"),
        (r"(?i)(\b(?:password|passwd|token|secret|api[_-]?key|access[_-]?key)\b\s*[:=]\s*)[^\s,;]+", "${1}[REDACTED]"),
        (r"(?i)(--(?:password|token|secret|api-key|access-key)(?:=|\s+))[^\s]+", "${1}[REDACTED]"),
        (r"(?i)(https?://[^\s/:@]+:)[^\s/@]+(@)", "${1}[REDACTED]${2}"),
        (r"(?s)-----BEGIN (?:[A-Z ]*PRIVATE KEY)-----.*?-----END (?:[A-Z ]*PRIVATE KEY)-----", "[REDACTED PRIVATE KEY]"),
    ];
    for (pattern, replacement) in rules {
        // The patterns are part of our source, but do not panic if one is edited incorrectly.
        if let Ok(regex) = Regex::new(pattern) {
            matches += regex.find_iter(&text).count();
            text = regex.replace_all(&text, replacement).into_owned();
        }
    }
    (text, matches)
}

fn looks_like_secret_flag(s: &str) -> bool {
    let flag = s.trim_start_matches('-').to_ascii_lowercase();
    matches!(
        flag.as_str(),
        "password" | "passwd" | "token" | "secret" | "api-key" | "api_key"
            | "access-key" | "access_key"
    )
}

/// Sanitizes command arguments; masks values following common secret flags.
pub fn redact_args(args: &[String]) -> (Vec<String>, usize) {
    let mut total = 0;
    let mut hidden_next = false;
    let mut result = Vec::with_capacity(args.len());
    for arg in args {
        if hidden_next {
            result.push("[REDACTED]".into());
            total += 1;
            hidden_next = false;
        } else if looks_like_secret_flag(arg) {
            result.push(arg.clone());
            hidden_next = true;
        } else {
            let (masked, count) = redact_text(arg);
            total += count;
            result.push(masked);
        }
    }
    (result, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_bearer_and_assignment() {
        let (text, count) = redact_text("Authorization: Bearer supersecret\npassword=abc123");
        assert!(!text.contains("supersecret"));
        assert!(!text.contains("abc123"));
        assert!(count >= 2);
    }
    #[test]
    fn masks_command_pair() {
        let args = vec!["--token".to_owned(), "topsecret".to_owned(), "test".to_owned()];
        let (sanitized, count) = redact_args(&args);
        assert_eq!(sanitized[1], "[REDACTED]");
        assert_eq!(sanitized[2], "test");
        assert_eq!(count, 1);
    }
    #[test]
    fn masks_url_password() {
        let (text, _) = redact_text("https://me:secret@example.com/path");
        assert!(!text.contains(":secret@"));
    }
    #[test]
    fn preserves_normal_errors() {
        let (text, count) = redact_text("error[E0308]: mismatched types");
        assert_eq!(text, "error[E0308]: mismatched types");
        assert_eq!(count, 0);
    }
    #[test]
    fn private_key_block_is_removed() {
        let (text, _) = redact_text("-----BEGIN PRIVATE KEY-----\nhello\n-----END PRIVATE KEY-----");
        assert!(!text.contains("hello"));
    }
}
