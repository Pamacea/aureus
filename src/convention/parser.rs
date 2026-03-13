//! Parse and generate Versioned Release Convention messages

use crate::cli::CommitType;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

static VRC_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<type>RELEASE|UPDATE|PATCH):\s*(?P<project>[^-]+?)\s*-\s*(?P<version>v\d+\.\d+\.\d+)"
    ).unwrap()
});

static CONVENTIONAL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"^(?P<type>feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(?:\((?P<scope>[^)]+)\))?(?P<breaking>!)?:\s+(?P<description>.+)"
    ).unwrap()
});

// Local commit type that doesn't depend on CLI's CommitType to avoid circular dependency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitMessage {
    pub commit_type: CommitTypeLocal,
    pub project: String,
    pub version: String,
    pub body: Option<String>,
    pub full_message: String,
    pub valid: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitTypeLocal {
    Release,
    Update,
    Patch,
}

impl CommitTypeLocal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Release => "RELEASE",
            Self::Update => "UPDATE",
            Self::Patch => "PATCH",
        }
    }
}

impl std::fmt::Display for CommitTypeLocal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<CommitTypeLocal> for crate::cli::CommitType {
    fn from(t: CommitTypeLocal) -> Self {
        match t {
            CommitTypeLocal::Release => Self::Release,
            CommitTypeLocal::Update => Self::Update,
            CommitTypeLocal::Patch => Self::Patch,
        }
    }
}

impl From<crate::cli::CommitType> for CommitTypeLocal {
    fn from(t: crate::cli::CommitType) -> Self {
        match t {
            crate::cli::CommitType::Release => Self::Release,
            crate::cli::CommitType::Update => Self::Update,
            crate::cli::CommitType::Patch => Self::Patch,
        }
    }
}

/// Parse a commit message in VRC format
///
/// # Examples
///
/// ```
/// let msg = parse_message("PATCH: Aureus - v1.0.1\n\n- Fixed: typo").unwrap();
/// assert_eq!(msg.commit_type, CommitType::Patch);
/// assert_eq!(msg.version, "v1.0.1");
/// ```
pub fn parse_message(message: &str) -> Option<CommitMessage> {
    let lines: Vec<&str> = message.lines().collect();
    let subject = lines.first()?;

    if let Some(caps) = VRC_PATTERN.captures(subject) {
        return Some(CommitMessage {
            commit_type: match &caps["type"] {
                "RELEASE" => CommitTypeLocal::Release,
                "UPDATE" => CommitTypeLocal::Update,
                "PATCH" => CommitTypeLocal::Patch,
                _ => return None,
            },
            project: caps["project"].trim().to_string(),
            version: caps["version"].to_string(),
            body: if lines.len() > 1 {
                let body_lines = lines.iter().skip(1).skip_while(|l| l.is_empty()).cloned().collect::<Vec<_>>().join("\n");
                if body_lines.is_empty() { None } else { Some(body_lines) }
            } else {
                None
            },
            full_message: message.to_string(),
            valid: true,
            error: None,
        });
    }

    None
}

/// Generate a VRC commit message
///
/// # Examples
///
/// ```
/// let msg = generate_message(
///     CommitType::Update,
///     "Aureus",
///     "v1.1.0",
///     Some("- Added: new feature")
/// );
/// ```
pub fn generate_message(
    commit_type: CommitType,
    project: &str,
    version: &str,
    body: Option<&str>,
) -> String {
    let mut message = format!("{}: {} - {}", commit_type, project, version);

    if let Some(body_content) = body {
        if !body_content.is_empty() {
            message.push_str("\n\n");
            message.push_str(body_content);
        }
    }

    message
}

/// Validate a commit message against VRC format
pub fn validate_message(message: &str) -> Result<CommitMessage, String> {
    if message.is_empty() {
        return Err("Message is empty".to_string());
    }

    let lines: Vec<&str> = message.lines().collect();
    let subject = lines.first().unwrap();

    // Check subject length
    if subject.len() > 100 {
        return Err(format!("Subject too long: {} characters (max 100)", subject.len()));
    }

    // Try to parse VRC
    if let Some(parsed) = parse_message(message) {
        return Ok(parsed);
    }

    // Check for conventional commits as fallback
    if CONVENTIONAL_PATTERN.is_match(subject) {
        return Err("Message matches Conventional Commits but not VRC format. Use TYPE: PROJECT - vVERSION".to_string());
    }

    Err("Invalid message format. Expected: TYPE: PROJECT - vVERSION".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vrc_message() {
        let msg = "PATCH: Aureus - v1.0.1\n\n- Fixed: typo";
        let parsed = parse_message(msg).unwrap();
        assert_eq!(parsed.commit_type, CommitType::Patch);
        assert_eq!(parsed.project, "Aureus");
        assert_eq!(parsed.version, "v1.0.1");
        assert_eq!(parsed.body, Some("- Fixed: typo".to_string()));
    }

    #[test]
    fn test_parse_vrc_no_body() {
        let msg = "UPDATE: MyProject - v1.1.0";
        let parsed = parse_message(msg).unwrap();
        assert_eq!(parsed.commit_type, CommitType::Update);
        assert_eq!(parsed.body, None);
    }

    #[test]
    fn test_generate_message() {
        let msg = generate_message(
            CommitType::Release,
            "Aureus",
            "v2.0.0",
            Some("- Breaking: API redesign"),
        );
        assert!(msg.starts_with("RELEASE: Aureus - v2.0.0"));
        assert!(msg.contains("Breaking: API redesign"));
    }

    #[test]
    fn test_validate_valid_message() {
        let msg = "PATCH: Aureus - v1.0.1\n\n- Fixed: bug";
        let result = validate_message(msg);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_too_long() {
        let msg = "UPDATE: Project - v1.0.0: this message is way too long and should fail validation";
        let result = validate_message(msg);
        assert!(result.is_err());
    }
}
