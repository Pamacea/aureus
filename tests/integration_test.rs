//! Integration tests for Aureus VRC CLI

use std::str::FromStr;

// Test Version struct and its methods
#[test]
fn test_version_parsing() {
    let version = aureus_vrc::convention::parse_version("v1.2.3").unwrap();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_version_bump_release() {
    let version = aureus_vrc::convention::Version::new(1, 2, 3);
    let bumped = version.bump(aureus_vrc::cli::CommitType::Release);
    assert_eq!(bumped, aureus_vrc::convention::Version::new(2, 0, 0));
}

#[test]
fn test_version_bump_update() {
    let version = aureus_vrc::convention::Version::new(1, 2, 3);
    let bumped = version.bump(aureus_vrc::cli::CommitType::Update);
    assert_eq!(bumped, aureus_vrc::convention::Version::new(1, 3, 0));
}

#[test]
fn test_version_bump_patch() {
    let version = aureus_vrc::convention::Version::new(1, 2, 3);
    let bumped = version.bump(aureus_vrc::cli::CommitType::Patch);
    assert_eq!(bumped, aureus_vrc::convention::Version::new(1, 2, 4));
}

#[test]
fn test_vrc_message_parsing() {
    let msg = "PATCH: Aureus - v1.0.1\n\n- Fixed: typo";
    let parsed = aureus_vrc::convention::parse_message(msg).unwrap();
    assert_eq!(parsed.commit_type, aureus_vrc::convention::CommitTypeLocal::Patch);
    assert_eq!(parsed.project, "Aureus");
    assert_eq!(parsed.version, "v1.0.1");
}

#[test]
fn test_vrc_message_generation() {
    let msg = aureus_vrc::convention::generate_message(
        aureus_vrc::cli::CommitType::Update,
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
        aureus_vrc::convention::detect_commit_type("feat: add login"),
        Some(aureus_vrc::cli::CommitType::Update)
    );

    // Fix → PATCH
    assert_eq!(
        aureus_vrc::convention::detect_commit_type("fix: typo"),
        Some(aureus_vrc::cli::CommitType::Patch)
    );

    // Breaking → RELEASE
    assert_eq!(
        aureus_vrc::convention::detect_commit_type("feat!: remove API"),
        Some(aureus_vrc::cli::CommitType::Release)
    );
}

#[test]
fn test_version_display() {
    let version = aureus_vrc::convention::Version::new(1, 2, 3);
    assert_eq!(version.to_string(), "v1.2.3");
}

#[test]
fn test_version_from_str() {
    let version = aureus_vrc::convention::Version::from_str("v1.2.3").unwrap();
    assert_eq!(version, aureus_vrc::convention::Version::new(1, 2, 3));

    let version2 = aureus_vrc::convention::Version::from_str("1.2.3").unwrap();
    assert_eq!(version2, aureus_vrc::convention::Version::new(1, 2, 3));
}

#[test]
fn test_version_suggestions() {
    let version = aureus_vrc::convention::Version::new(1, 0, 0);
    let suggestions = version.suggestions();
    assert_eq!(suggestions.current, aureus_vrc::convention::Version::new(1, 0, 0));
    assert_eq!(suggestions.release, aureus_vrc::convention::Version::new(2, 0, 0));
    assert_eq!(suggestions.update, aureus_vrc::convention::Version::new(1, 1, 0));
    assert_eq!(suggestions.patch, aureus_vrc::convention::Version::new(1, 0, 1));
}

#[test]
fn test_commit_type_local_display() {
    assert_eq!(aureus_vrc::convention::CommitTypeLocal::Release.to_string(), "RELEASE");
    assert_eq!(aureus_vrc::convention::CommitTypeLocal::Update.to_string(), "UPDATE");
    assert_eq!(aureus_vrc::convention::CommitTypeLocal::Patch.to_string(), "PATCH");
}

#[test]
fn test_commit_type_to_string() {
    assert_eq!(aureus_vrc::cli::CommitType::Release.to_string(), "RELEASE");
    assert_eq!(aureus_vrc::cli::CommitType::Update.to_string(), "UPDATE");
    assert_eq!(aureus_vrc::cli::CommitType::Patch.to_string(), "PATCH");
}

#[test]
fn test_commit_type_from_str() {
    assert_eq!(aureus_vrc::cli::CommitType::from_str("RELEASE"), Some(aureus_vrc::cli::CommitType::Release));
    assert_eq!(aureus_vrc::cli::CommitType::from_str("update"), Some(aureus_vrc::cli::CommitType::Update));
    assert_eq!(aureus_vrc::cli::CommitType::from_str("patch"), Some(aureus_vrc::cli::CommitType::Patch));
    assert_eq!(aureus_vrc::cli::CommitType::from_str("unknown"), None);
}
