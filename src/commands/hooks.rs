//! Hooks command implementation

use anyhow::{Context, Result};
use colored::Colorize;

use crate::cli::HooksCommand;
use crate::git::hooks as git_hooks;

pub fn execute(cmd: HooksCommand) -> Result<()> {
    match cmd.action {
        crate::cli::HooksAction::Install { global } => {
            install(global)
        }
        crate::cli::HooksAction::Uninstall { global } => {
            uninstall(global)
        }
        crate::cli::HooksAction::Status => {
            status()
        }
    }
}

fn install(_global: bool) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    println!("\n{}", "Installing Aureus hooks...".bold().cyan());

    let result = git_hooks::install_hooks(&repo_path)?;

    if result.installed.is_empty() {
        println!("  {}", "No hooks installed (already present)".yellow());
    } else {
        for hook in &result.installed {
            println!("  ✓ {}", hook.green());
        }
    }

    if result.has_failures() {
        println!();
        for (hook, error) in &result.failed {
            println!("  ✗ {}: {}", hook.red(), error.dimmed());
        }
    }

    println!();
    println!("✓ {}", "Hooks installed successfully!".green());

    Ok(())
}

fn uninstall(_global: bool) -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    println!("\n{}", "Uninstalling Aureus hooks...".bold().cyan());

    let result = git_hooks::uninstall_hooks(&repo_path)?;

    if result.installed.is_empty() {
        println!("  {}", "No hooks to remove".dimmed());
    } else {
        for hook in &result.installed {
            println!("  ✓ {}", format!("Removed {}", hook).green());
        }
    }

    println!();
    println!("✓ {}", "Hooks uninstalled!".green());

    Ok(())
}

fn status() -> Result<()> {
    let repo_path = std::env::current_dir()
        .context("Cannot get current directory")?;

    let status = git_hooks::hooks_status(&repo_path)?;

    println!("\n{}", "Git Hooks Status:".bold().cyan());
    println!();

    if status.installed.is_empty() {
        println!("  {}", "No Aureus hooks installed".dimmed());
    } else {
        for hook in &status.installed {
            println!("  ✓ {}", hook.green());
        }
    }

    println!();

    Ok(())
}
