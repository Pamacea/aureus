//! Integration tests for Aureus CLI

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_version_parsing() {
    let version = crate::convention::parse_version("v1.2.3").unwrap();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_version_bump_release() {
    let version = crate::convention::Version::new(1, 2, 3);
    let bumped = version.bump(crate::cli::CommitType::Release);
    assert_eq!(bumped, crate::convention::Version::new(2, 0, 0));
}

#[test]
fn test_version_bump_update() {
    let version = crate::convention::Version::new(1, 2, 3);
    let bumped = version.bump(crate::cli::CommitType::Update);
    assert_eq!(bumped, crate::convention::Version::new(1, 3, 0));
}

#[test]
fn test_version_bump_patch() {
    let version = crate::convention::Version::new(1, 2, 3);
    let bumped = version.bump(crate::cli::CommitType::Patch);
    assert_eq!(bumped, crate::convention::Version::new(1, 2, 4));
}

#[test]
fn test_vrc_message_parsing() {
    let msg = "PATCH: Aureus - v1.0.1\n\n- Fixed: typo";
    let parsed = crate::convention::parse_message(msg).unwrap();
    assert_eq!(parsed.commit_type, crate::cli::CommitType::Patch);
    assert_eq!(parsed.project, "Aureus");
    assert_eq!(parsed.version, "v1.0.1");
}

#[test]
fn test_vrc_message_generation() {
    let msg = crate::convention::generate_message(
        crate::cli::CommitType::Update,
        "Aureus",
        "v1.1.0",
        Some("- Added: new feature"),
    );
    assert!(msg.starts_with("UPDATE: Aureus - v1.1.0"));
    assert!(msg.contains("Added: new feature"));
}

#[test]
fn test_commit_type_detection() {
    // Feat → UPDATE
    assert_eq!(
        crate::convention::detect_commit_type("feat: add login"),
        Some(crate::cli::CommitType::Update)
    );

    // Fix → PATCH
    assert_eq!(
        crate::convention::detect_commit_type("fix: typo"),
        Some(crate::cli::CommitType::Patch)
    );

    // Breaking → RELEASE
    assert_eq!(
        crate::convention::detect_commit_type("feat!: remove API"),
        Some(crate::cli::CommitType::Release)
    );
}

#[test]
fn test_version_display() {
    let version = crate::convention::Version::new(1, 2, 3);
    assert_eq!(version.to_string(), "v1.2.3");
}

#[test]
fn test_version_from_str() {
    let version = crate::convention::Version::from_str("v1.2.3").unwrap();
    assert_eq!(version, crate::convention::Version::new(1, 2, 3));

    let version2 = crate::convention::Version::from_str("1.2.3").unwrap();
    assert_eq!(version2, crate::convention::Version::new(1, 2, 3));
}

#[test]
fn test_version_suggestions() {
    let version = crate::convention::Version::new(1, 0, 0);
    let suggestions = version.suggestions();
    assert_eq!(suggestions.current, crate::convention::Version::new(1, 0, 0));
    assert_eq!(suggestions.release, crate::convention::Version::new(2, 0, 0));
    assert_eq!(suggestions.update, crate::convention::Version::new(1, 1, 0));
    assert_eq!(suggestions.patch, crate::convention::Version::new(1, 0, 1));
}

#[test]
fn test_utils_truncate() {
    use crate::utils::truncate;
    assert_eq!(truncate("hello world", 8), "hello...");
    assert_eq!(truncate("hi", 10), "hi");
}

#[test]
fn test_utils_strip_ansi() {
    use crate::utils::strip_ansi;
    let colored = "\x1b[31mRed\x1b[0m";
    assert_eq!(strip_ansi(colored), "Red");
}

#[test]
fn test_commit_type_to_string() {
    assert_eq!(crate::cli::CommitType::Release.to_string(), "RELEASE");
    assert_eq!(crate::cli::CommitType::Update.to_string(), "UPDATE");
    assert_eq!(crate::cli::CommitType::Patch.to_string(), "PATCH");
}

#[test]
fn test_commit_type_from_str() {
    assert_eq!(crate::cli::CommitType::from_str("RELEASE"), Some(crate::cli::CommitType::Release));
    assert_eq!(crate::cli::CommitType::from_str("update"), Some(crate::cli::CommitType::Update));
    assert_eq!(crate::cli::CommitType::from_str("patch"), Some(crate::cli::CommitType::Patch));
    assert_eq!(crate::cli::CommitType::from_str("unknown"), None);
}
