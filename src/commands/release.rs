//! Release command implementation

use anyhow::{Context, Result};
use colored::Colorize;

use crate::cli::ReleaseCommand;
use crate::convention::{self, parse_version, bump_version, CommitType};
use crate::git::{self, get_current_version, get_repo_path};

pub fn execute(cmd: ReleaseCommand) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    let version = if let Some(v) = cmd.version {
        v
    } else if cmd.auto {
        determine_auto_version(&repo_path)?
    } else {
        get_current_version(&repo_path)
            .unwrap_or_else(|| "v0.0.0".to_string())
    };

    // Create tag
    let tag_msg_string = if cmd.annotated {
        Some(format!("Release {}", version))
    } else {
        None
    };
    let tag_msg: Option<&str> = tag_msg_string.as_deref();

    println!("\n{}", "Creating release:".bold().green());
    println!("  {}", version.cyan());
    println!();

    git::create_tag(&repo_path, &version, tag_msg, cmd.annotated)
        .context("Failed to create tag")?;

    println!("✓ {}", format!("Tag {} created", version).green());

    // Generate CHANGELOG if requested
    if cmd.changelog {
        update_changelog(&repo_path, &version)?;
        println!("✓ {}", "CHANGELOG.md updated".green());
    }

    // Push if requested
    if cmd.push {
        push_tag(&repo_path, &version)?;
        println!("✓ {}", format!("Tag {} pushed to remote", version).green());
    }

    Ok(())
}

fn determine_auto_version(repo_path: &std::path::Path) -> Result<String> {
    let last = git::get_last_commit(repo_path)?;
    let current = get_current_version(repo_path)
        .unwrap_or_else(|| "v0.0.0".to_string());

    // Detect commit type from last message
    let commit_type = detect_commit_type(&last.message)?;

    let bumped = bump_version(&current, commit_type).map_err(|e| anyhow::anyhow!(e))?;
    Ok(bumped)
}

fn detect_commit_type(message: &str) -> Result<CommitType> {
    if message.contains('!') || message.contains("BREAKING") {
        Ok(CommitType::Release)
    } else if message.starts_with("feat") || message.starts_with("refactor") {
        Ok(CommitType::Update)
    } else {
        Ok(CommitType::Patch)
    }
}

fn update_changelog(repo_path: &std::path::Path, version: &str) -> Result<()> {
    let changelog_path = repo_path.join("CHANGELOG.md");

    let existing = if changelog_path.exists() {
        std::fs::read_to_string(&changelog_path)
            .context("Failed to read CHANGELOG.md")?
    } else {
        String::new()
    };

    let date = chrono::Utc::now().format("%Y-%m-%d");
    let new_entry = format!(
        "## [{}] - {}\n\n### Added\n\n### Changed\n\n### Fixed\n\n",
        version.trim_start_matches('v'),
        date
    );

    let updated = if existing.is_empty() {
        format!("# Changelog\n\n{}", new_entry)
    } else {
        // Insert after header
        let lines: Vec<&str> = existing.lines().collect();
        if lines.first().map_or(false, |l| l.starts_with("# ")) {
            let mut result = lines[0].to_string();
            result.push_str("\n\n");
            result.push_str(&new_entry);
            for line in lines.iter().skip(1) {
                result.push_str(line);
                result.push('\n');
            }
            result
        } else {
            format!("{}{}", new_entry, existing)
        }
    };

    std::fs::write(&changelog_path, updated)
        .context("Failed to write CHANGELOG.md")?;

    Ok(())
}

fn push_tag(repo_path: &std::path::Path, version: &str) -> Result<()> {
    use std::process::Command;

    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(version)
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to push tag: {}", stderr);
    }

    Ok(())
}
