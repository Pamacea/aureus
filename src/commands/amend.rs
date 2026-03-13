//! Amend command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};

use crate::cli::AmendCommand;
use crate::git::{self, get_last_commit};
use crate::convention::parse_message;

pub fn execute(cmd: AmendCommand) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    // Add files if --all flag is set
    if cmd.all {
        git::add_files(&repo_path, &[".".to_string()])
            .context("Failed to stage files")?;
    }

    // Get last commit
    let last_commit = get_last_commit(&repo_path)?;

    println!("\n{}", "Last commit:".bold());
    println!("  {}", last_commit.summary.yellow());
    println!();

    // Get or prompt for additional message
    let additional = if let Some(msg) = cmd.message {
        msg
    } else {
        if !cmd.all {
            prompt_additional()?
        } else {
            String::new()
        }
    };

    if additional.is_empty() && !cmd.all {
        println!("{}", "No changes to amend.".dimmed());
        return Ok(());
    }

    // Parse existing commit to check if it's VRC format
    let new_message = if let Some(parsed) = parse_message(&last_commit.message) {
        // It's VRC format - append to body
        let mut body = parsed.body.unwrap_or_default();

        if !additional.is_empty() {
            if !body.is_empty() {
                body.push_str("\n- ");
            } else {
                body.push_str("\n\n- ");
            }
            body.push_str(&additional);
        }

        let body_suffix = if !body.is_empty() {
            format!("\n\n{}", body)
        } else {
            String::new()
        };

        format!("{}: {} - {}{}",
            parsed.commit_type,
            parsed.project,
            parsed.version,
            body_suffix
        )
    } else {
        // Not VRC format - prepend to existing
        format!("{}\n{}",
            additional,
            last_commit.message
        )
    };

    // Amend the commit
    git::amend_last_commit(&repo_path, Some(&new_message))
        .context("Failed to amend commit")?;

    println!("✓ {}", "Commit amended successfully.".green());

    Ok(())
}

fn prompt_additional() -> Result<String> {
    println!("{}", "Enter additional message (or leave empty to skip):".bold());

    let message = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Additional")
        .allow_empty(true)
        .interact()?;

    Ok(message)
}
