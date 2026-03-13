//! Stats command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use std::path::PathBuf;

use crate::cli::{StatsCommand, OutputFormat};
use crate::storage::{self, get_tracker, TrackingEvent};
use crate::git::get_repo_path;

pub fn execute(cmd: StatsCommand) -> Result<()> {
    let tracker = get_tracker()?;

    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    let repo_path_str = repo_path.to_string_lossy().to_string();

    match cmd.format {
        OutputFormat::Text => display_text(&tracker, Some(&repo_path_str))?,
        OutputFormat::Json => display_json(&tracker, Some(&repo_path_str))?,
    }

    Ok(())
}

fn display_text(tracker: &storage::tracking::Tracker, repo_path: Option<&str>) -> Result<()> {
    println!("\n{}", "Aureus Statistics:".bold().cyan());
    println!();

    let stats = tracker.get_stats(repo_path)?;

    if stats.total == 0 {
        println!("  {}", "No commits tracked yet.".dimmed());
        return Ok(());
    }

    println!("  {} commits tracked", stats.total.to_string().bold());
    println!();

    println!("  By Type:");
    println!("    {} RELEASE", stats.releases.to_string().red());
    println!("    {} UPDATE", stats.updates.to_string().green());
    println!("    {} PATCH", stats.patches.to_string().yellow());
    println!();

    // Recent commits
    let recent = tracker.get_recent_commits(5)?;

    if !recent.is_empty() {
        println!("  Recent commits:");
        for event in recent {
            let time = event.timestamp.format("%Y-%m-%d %H:%M").to_string();
            println!("    {} {} {} {}",
                time.dimmed(),
                event.commit_type.to_uppercase().bold(),
                event.version.cyan(),
                event.project.white()
            );
        }
    }

    println!();

    Ok(())
}

fn display_json(tracker: &storage::tracking::Tracker, repo_path: Option<&str>) -> Result<()> {
    let stats = tracker.get_stats(repo_path)?;
    let recent = tracker.get_recent_commits(10)?;

    let output = serde_json::json!({
        "stats": stats,
        "recent": recent
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}
