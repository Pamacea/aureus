//! Config command implementation

use anyhow::{Context, Result};
use colored::Colorize;

use crate::cli::{ConfigCommand, OutputFormat};
use crate::storage::{self, load_config, save_config, set_config_value};

pub fn execute(cmd: ConfigCommand) -> Result<()> {
    match cmd.action {
        crate::cli::ConfigAction::Get { key } => {
            get_value(&key)
        }
        crate::cli::ConfigAction::Set { key, value } => {
            set_value(&key, &value)
        }
        crate::cli::ConfigAction::List { all } => {
            list_config(all)
        }
        crate::cli::ConfigAction::Edit => {
            edit_config()
        }
        crate::cli::ConfigAction::Reset { key } => {
            reset_config(key)
        }
    }
}

fn get_value(key: &str) -> Result<()> {
    let value = storage::get_config_value(key)?
        .unwrap_or(serde_json::Value::Null);

    match value {
        serde_json::Value::Null => {
            println!("{}", "null".dimmed());
        }
        serde_json::Value::String(s) => {
            println!("{}", s);
        }
        other => {
            println!("{}", serde_json::to_string_pretty(&other)?);
        }
    }

    Ok(())
}

fn set_value(key: &str, value: &str) -> Result<()> {
    set_config_value(key, value)?;
    println!("✓ {}", format!("{} = {}", key.cyan(), value.green()).bold());
    Ok(())
}

fn list_config(all: bool) -> Result<()> {
    let config = load_config()?;

    println!("\n{}", "Aureus Configuration:".bold().cyan());
    println!();

    println!("{}:", "project".bold());
    println!("  name: {}", config.project.name.unwrap_or_else(|| "(auto)".to_string()).dimmed());
    println!("  default_branch: {}", config.project.default_branch.white());
    println!();

    println!("{}:", "commit".bold());
    println!("  types:");
    println!("    RELEASE: {}", config.commit.types.release.description.red());
    println!("    UPDATE: {}", config.commit.types.update.description.green());
    println!("    PATCH: {}", config.commit.types.patch.description.yellow());
    println!();

    println!("  rules:");
    println!("    subject_max_length: {}", config.commit.rules.subject_max_length);
    println!("    require_version: {}", config.commit.rules.require_version);
    println!();

    println!("{}:", "hooks".bold());
    println!("  pre_commit:");
    println!("    enabled: {}", config.hooks.pre_commit.enabled);
    println!("    lint: {}", config.hooks.pre_commit.lint);
    println!("    typecheck: {}", config.hooks.pre_commit.typecheck);
    println!("    secret_scan: {}", config.hooks.pre_commit.secret_scan);
    println!();

    Ok(())
}

fn edit_config() -> Result<()> {
    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| {
            if cfg!(windows) { "notepad".to_string() } else { "vi".to_string() }
        });

    let config_path = storage::get_config_path()?;

    println!("Editing config with {}...\n", editor.cyan());

    let status = std::process::Command::new(&editor)
        .arg(&config_path)
        .status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    println!("✓ {}", "Config saved".green());

    Ok(())
}

fn reset_config(key: Option<String>) -> Result<()> {
    if let Some(key) = key {
        // Reset specific key (not fully implemented)
        println!("Reset config key: {}", key.cyan());
        anyhow::bail!("Resetting individual keys not yet implemented");
    } else {
        // Reset entire config
        let default = storage::Config::default();
        save_config(&default)?;
        println!("✓ {}", "Config reset to defaults".green());
    }

    Ok(())
}
