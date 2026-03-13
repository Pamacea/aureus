//! Commit command implementation

use anyhow::{Context, Result, anyhow, bail};
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::cli::CommitCommand;
use crate::convention::{generate_message, detect_commit_type};
use crate::git::{self};
use crate::storage::{self, TrackingEvent, get_tracker};

pub fn execute(cmd: CommitCommand) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    // Check if we're in a git repository
    if !git::is_repo(&repo_path) {
        bail!("Not a git repository. Please run this command inside a git repository.");
    }

    // Add files if --all flag is set
    if cmd.all {
        git::add_files(&repo_path, &[".".to_string()])
            .context("Failed to stage files")?;
    }

    // Get or prompt for message
    let message = if let Some(msg) = cmd.message {
        msg
    } else {
        prompt_message()?
    };

    // Detect or get commit type
    let commit_type = if let Some(t) = cmd.r#type {
        t
    } else {
        detect_from_message(&message)?
    };

    // Get project name
    let project = get_project_name(&repo_path, cmd.project)?;

    // Get current version
    let current_version = git::get_current_version(&repo_path)
        .unwrap_or_else(|| "v0.0.0".to_string());

    // Determine next version
    let next_version = git::determine_next_version(
        &repo_path,
        commit_type,
        Some(&current_version),
    );

    // Parse base version for body
    let base_version = cmd.version.as_ref()
        .unwrap_or(&next_version);

    // Generate VRC message
    let vrc_message = generate_vrc_message(
        commit_type,
        &project,
        base_version,
        &message,
    );

    // Create the commit
    println!("\n{}", "Creating commit:".bold().green());
    println!("{}", vrc_message.dimmed());
    println!();

    git::create_commit(&repo_path, &vrc_message, cmd.allow_empty)
        .context("Failed to create commit")?;

    // Track the commit
    if let Ok(tracker) = get_tracker() {
        let event = TrackingEvent {
            timestamp: chrono::Utc::now(),
            repo_path: repo_path.to_string_lossy().to_string(),
            commit_type: commit_type.to_string(),
            version: base_version.clone(),
            project: project.clone(),
            message: message.clone(),
        };
        let _ = tracker.track_commit(&event);
    }

    // Update state
    storage::update_repo_version(&repo_path, base_version).ok();

    println!("✓ {} {}", "Commit created:".green(), vrc_message.lines().next().unwrap_or(""));

    Ok(())
}

fn prompt_message() -> Result<String> {
    println!("\n{}", "Enter commit message:".bold());

    let message = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Message")
        .allow_empty(false)
        .interact()?;

    Ok(message)
}

fn detect_from_message(message: &str) -> Result<crate::cli::CommitType> {
    if let Some(detected) = detect_commit_type(message) {
        Ok(detected)
    } else {
        Ok(crate::cli::CommitType::Patch)
    }
}

fn get_project_name(repo_path: &std::path::Path, override_name: Option<String>) -> Result<String> {
    if let Some(name) = override_name {
        return Ok(name);
    }

    // Try from config
    if let Ok(config) = storage::load_config() {
        if let Some(name) = config.project.name {
            return Ok(name);
        }
    }

    // Fall back to directory name
    repo_path
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Cannot determine project name"))
}

fn generate_vrc_message(
    commit_type: crate::cli::CommitType,
    project: &str,
    version: &str,
    original_message: &str,
) -> String {
    // Format as VRC with bullet point body
    let mut body = String::new();

    // Add bullet point for the change
    let first_line = original_message.lines().next().unwrap_or("");
    let prefix = match commit_type {
        crate::cli::CommitType::Release => "Breaking",
        crate::cli::CommitType::Update => "Added",
        crate::cli::CommitType::Patch => "Fixed",
    };

    body.push_str("- ");
    body.push_str(prefix);
    body.push_str(": ");
    body.push_str(first_line);

    // Add remaining lines as additional bullets
    for line in original_message.lines().skip(1) {
        if !line.is_empty() {
            body.push_str("\n- ");
            body.push_str(line);
        }
    }

    generate_message(commit_type, project, version, Some(&body))
}
