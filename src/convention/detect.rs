//! Auto-detect commit type from message content

use crate::cli::CommitType;

/// Detect commit type from conventional commit message
///
/// Maps conventional commit types to VRC types:
/// - `feat`, `refactor!` → UPDATE
/// - `fix`, `perf` → PATCH
/// - breaking changes (`!`) → RELEASE
pub fn detect_commit_type(message: &str) -> Option<CommitType> {
    let first_line = message.lines().next()?;

    // Check for breaking change indicator
    if first_line.contains('!') || first_line.contains("BREAKING CHANGE") {
        return Some(CommitType::Release);
    }

    // Check conventional commit types
    if first_line.starts_with("feat") || first_line.starts_with("refactor") {
        return Some(CommitType::Update);
    }

    if first_line.starts_with("fix") || first_line.starts_with("perf") {
        return Some(CommitType::Patch);
    }

    // Default to PATCH for unknown types
    Some(CommitType::Patch)
}

/// Infer commit type from changed files
///
/// - Changes to Cargo.toml, package.json → UPDATE
/// - Test file changes only → PATCH
/// - README, docs → PATCH
#[cfg(test)]
pub fn infer_from_files(files: &[String]) -> Option<CommitType> {
    let has_manifest = files.iter().any(|f| {
        f.ends_with("Cargo.toml") || f.ends_with("package.json") || f.ends_with("go.mod")
    });

    let has_source = files.iter().any(|f| {
        f.ends_with(".rs") || f.ends_with(".ts") || f.ends_with(".js") || f.ends_with(".go")
    });

    let only_tests = files.iter().all(|f| {
        f.contains("test") || f.ends_with("_test.rs") || f.ends_with(".test.ts") || f.ends_with(".spec.ts")
    });

    let only_docs = files.iter().all(|f| {
        f.ends_with(".md") || f.contains("docs/") || f.starts_with("docs/")
    });

    if only_tests || only_docs {
        Some(CommitType::Patch)
    } else if has_manifest {
        Some(CommitType::Update)
    } else if has_source {
        Some(CommitType::Update)
    } else {
        Some(CommitType::Patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_feat() {
        assert_eq!(detect_commit_type("feat: add authentication"), Some(CommitType::Update));
    }

    #[test]
    fn test_detect_fix() {
        assert_eq!(detect_commit_type("fix: typo in header"), Some(CommitType::Patch));
    }

    #[test]
    fn test_detect_breaking() {
        assert_eq!(detect_commit_type("feat!: remove deprecated API"), Some(CommitType::Release));
    }

    #[test]
    fn test_infer_from_manifest() {
        let files = vec!["Cargo.toml".to_string(), "src/main.rs".to_string()];
        assert_eq!(infer_from_files(&files), Some(CommitType::Update));
    }

    #[test]
    fn test_infer_from_tests() {
        let files = vec!["src/auth_test.rs".to_string(), "tests/unit.rs".to_string()];
        assert_eq!(infer_from_files(&files), Some(CommitType::Patch));
    }
}
