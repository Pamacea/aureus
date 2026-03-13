//! Diff command implementation

use anyhow::Result;
use colored::Colorize;
use std::path::Path;

use crate::cli::DiffCommand;
use crate::git::{get_diff, get_staged_files};

pub fn execute(cmd: DiffCommand) -> Result<()> {
    let repo_path = Path::new(".");
    let cached = cmd.cached;

    if cmd.name_status {
        print_name_status(repo_path, cached)?;
    } else {
        let diff = get_diff(repo_path, cached)?;
        if diff.is_empty() {
            println!("{}", "No changes to display.".dimmed());
        } else {
            print!("{}", diff);
        }
    }

    Ok(())
}

fn print_name_status(repo_path: &Path, cached: bool) -> Result<()> {
    let files = if cached {
        get_staged_files(repo_path)?
    } else {
        // For working tree, get all modified files
        crate::git::get_status(repo_path)?
            .iter()
            .map(|e| e.path.clone())
            .collect()
    };

    if files.is_empty() {
        println!("{}", "No files changed.".dimmed());
        return Ok(());
    }

    for file in &files {
        let status = if cached { "Staged" } else { "Modified" };
        println!("  {}: {}", status.cyan(), file);
    }

    println!();
    println!("Total: {} file(s)", files.len());

    Ok(())
}
