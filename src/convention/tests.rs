//! Unit tests for convention module

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_parse_vrc_message() {
        let msg = "PATCH: Aureus - v1.0.1\n\n- Fixed: typo";
        let parsed = parse_message(msg).unwrap();
        assert_eq!(parsed.commit_type, CommitType::Patch);
        assert_eq!(parsed.project, "Aureus");
        assert_eq!(parsed.version, "v1.0.1");
    }

    #[test]
    fn test_generate_message() {
        let msg = generate_message(
            CommitType::Update,
            "TestProject",
            "v1.1.0",
            Some("- Added: feature"),
        );
        assert!(msg.starts_with("UPDATE: TestProject - v1.1.0"));
    }

    #[test]
    fn test_validate_message() {
        let valid_msg = "PATCH: Test - v1.0.0\n\n- Fixed";
        assert!(validate_message(valid_msg).is_ok());

        let invalid_msg = "Not a VRC message";
        assert!(validate_message(invalid_msg).is_err());
    }
}
