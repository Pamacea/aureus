//! Git hooks installation and management

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

const HOOKS_DIR: &str = ".git/hooks";
const COMMIT_MSG_HOOK: &str = "commit-msg";
const PRE_COMMIT_HOOK: &str = "pre-commit";

/// Hook scripts content
const COMMIT_MSG_SCRIPT: &str = r#"#!/bin/sh
# Aureus Versioned Release Convention validation

# Check if aureus is available
if ! command -v aureus &>/dev/null; then
    # Silent exit if aureus not installed
    exit 0
fi

# Validate the commit message
aureus hooks validate-commit "$1"
"#;

const PRE_COMMIT_SCRIPT: &str = r#"#!/bin/sh
# Aureus pre-commit checks

# Check if aureus is available
if ! command -v aureus &>/dev/null; then
    exit 0
fi

# Run pre-commit checks
aureus hooks pre-commit
"#;

/// Install git hooks for VRC validation
pub fn install_hooks(repo_path: &Path) -> Result<HookStatus> {
    let hooks_dir = repo_path.join(HOOKS_DIR);

    // Ensure hooks directory exists
    fs::create_dir_all(&hooks_dir)
        .context("Failed to create hooks directory")?;

    let mut installed = Vec::new();
    let mut failed = Vec::new();

    // Install commit-msg hook
    let commit_msg_path = hooks_dir.join(COMMIT_MSG_HOOK);
    match write_hook(&commit_msg_path, COMMIT_MSG_SCRIPT) {
        Ok(_) => installed.push(COMMIT_MSG_HOOK.to_string()),
        Err(e) => failed.push((COMMIT_MSG_HOOK.to_string(), e.to_string())),
    }

    // Install pre-commit hook
    let pre_commit_path = hooks_dir.join(PRE_COMMIT_HOOK);
    match write_hook(&pre_commit_path, PRE_COMMIT_SCRIPT) {
        Ok(_) => installed.push(PRE_COMMIT_HOOK.to_string()),
        Err(e) => failed.push((PRE_COMMIT_HOOK.to_string(), e.to_string())),
    }

    Ok(HookStatus { installed, failed })
}

/// Uninstall git hooks
pub fn uninstall_hooks(repo_path: &Path) -> Result<HookStatus> {
    let hooks_dir = repo_path.join(HOOKS_DIR);

    let mut removed = Vec::new();
    let mut failed = Vec::new();

    for hook_name in [COMMIT_MSG_HOOK, PRE_COMMIT_HOOK] {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() {
            match fs::remove_file(&hook_path) {
                Ok(_) => removed.push(hook_name.to_string()),
                Err(e) => failed.push((hook_name.to_string(), e.to_string())),
            }
        }
    }

    Ok(HookStatus { installed: removed, failed })
}

/// Check hook installation status
pub fn hooks_status(repo_path: &Path) -> Result<HookStatus> {
    let hooks_dir = repo_path.join(HOOKS_DIR);

    let mut installed = Vec::new();
    let mut failed = Vec::new();

    for hook_name in [COMMIT_MSG_HOOK, PRE_COMMIT_HOOK] {
        let hook_path = hooks_dir.join(hook_name);
        if hook_path.exists() {
            installed.push(hook_name.to_string());
        }
    }

    Ok(HookStatus { installed, failed })
}

/// Write hook file with executable permissions
fn write_hook(path: &Path, content: &str) -> Result<()> {
    fs::write(path, content)
        .context("Failed to write hook file")?;

    // Set executable permission (Unix-like systems only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

pub struct HookStatus {
    pub installed: Vec<String>,
    pub failed: Vec<(String, String)>,
}

impl HookStatus {
    pub fn is_complete(&self) -> bool {
        self.installed.len() == 2 && self.failed.is_empty()
    }

    pub fn has_failures(&self) -> bool {
        !self.failed.is_empty()
    }
}

/// Validate commit message from hook
pub fn validate_commit_message(msg_file: &Path) -> Result<bool> {
    let content = fs::read_to_string(msg_file)
        .context("Failed to read commit message file")?;

    // Use VRC parser
    match crate::convention::parse_message(&content) {
        Some(parsed) if parsed.valid => Ok(true),
        _ => Ok(false), // Invalid, but don't error (just warn)
    }
}

/// Run pre-commit checks
pub fn run_pre_commit(repo_path: &Path) -> Result<bool> {
    // For now, just pass
    // TODO: Add secret scanning, linting, etc.
    Ok(true)
}
