//! Status command implementation

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

use crate::cli::StatusCommand;
use crate::git::{get_status, get_status_summary, FileStatus};

pub fn execute(cmd: StatusCommand) -> Result<()> {
    let repo_path = Path::new(".");
    let summary = get_status_summary(repo_path)?;

    if !summary.has_changes() {
        println!("{}", "✓ Working tree clean.".green().bold());
        return Ok(());
    }

    if cmd.porcelain {
        print_porcelain(&summary);
    } else if cmd.short {
        print_short(&summary);
    } else {
        print_full(&summary);
    }

    Ok(())
}

fn print_full(summary: &crate::git::StatusSummary) {
    let entries = &summary.entries;

    if entries.is_empty() {
        return;
    }

    println!();
    println!("{}", "Changes:".bold());

    for entry in entries {
        let emoji = entry.status.emoji();
        let status_str = format!("{:?}", entry.status).dimmed();
        println!("  {} {} {}", emoji, entry.path, status_str);
    }

    println!();
    println!("{}", "Summary:".bold());
    for (status, count) in &summary.counts {
        println!("  {}: {}", count, status);
    }
}

fn print_short(summary: &crate::git::StatusSummary) {
    for entry in &summary.entries {
        let emoji = entry.status.emoji();
        println!("{} {}", emoji, entry.path);
    }
}

fn print_porcelain(summary: &crate::git::StatusSummary) {
    for entry in &summary.entries {
        let status_char = match entry.status {
            FileStatus::Modified => "M",
            FileStatus::Added => "A",
            FileStatus::Deleted => "D",
            FileStatus::Renamed => "R",
            FileStatus::Copied => "C",
            FileStatus::Untracked => "??",
            FileStatus::Ignored => "!!",
            FileStatus::Conflicted => "UU",
        };
        println!("{} {}", status_char, entry.path);
    }
}
