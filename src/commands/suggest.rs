//! Suggest command implementation

use anyhow::{Context, Result};
use colored::Colorize;

use crate::cli::{SuggestCommand, OutputFormat};
use crate::convention::{self, parse_version};
use crate::git::get_current_version;

pub fn execute(cmd: SuggestCommand) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    let current = get_current_version(&repo_path)
        .unwrap_or_else(|| "v0.0.0".to_string());

    let version = parse_version(&current)
        .unwrap_or(convention::version::Version::new(0, 0, 0));

    let suggestions = version.suggestions();

    match cmd.format {
        OutputFormat::Text => display_text(&suggestions, cmd.all),
        OutputFormat::Json => display_json(&suggestions)?,
    }

    Ok(())
}

fn display_text(suggestions: &convention::version::VersionSuggestions, all: bool) {
    println!("\n{}", "Version Suggestions:".bold().cyan());
    println!();

    if all {
        println!("  {}: {}", "Current".dimmed(), suggestions.current);
        println!("  {}: {}", "RELEASE".red().bold(), suggestions.release);
        println!("  {}: {}", "UPDATE".green().bold(), suggestions.update);
        println!("  {}: {}", "PATCH".yellow().bold(), suggestions.patch);
    } else {
        println!("  {}", suggestions.current.to_string().cyan());
        println!();
        println!("  Run with specific type:");
        println!("    {}", format!("aureus commit --type RELEASE").red());
        println!("    {}", format!("aureus commit --type UPDATE").green());
        println!("    {}", format!("aureus commit --type PATCH").yellow());
    }

    println!();
}

fn display_json(suggestions: &convention::version::VersionSuggestions) -> Result<()> {
    let json = serde_json::to_string_pretty(suggestions)
        .context("Failed to serialize suggestions")?;
    println!("{}", json);
    Ok(())
}
