//! Aureus - Versioned Release Convention CLI

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

mod cli;
mod commands;
mod convention;
mod git;
mod storage;
mod utils;

use cli::CliArgs;

fn main() -> Result<()> {
    let args = CliArgs::parse();

    // Execute command
    match args.command {
        cli::Command::Commit(cmd) => commands::commit::execute(cmd)?,
        cli::Command::Amend(cmd) => commands::amend::execute(cmd)?,
        cli::Command::Release(cmd) => commands::release::execute(cmd)?,
        cli::Command::Suggest(cmd) => commands::suggest::execute(cmd)?,
        cli::Command::Hooks(cmd) => commands::hooks::execute(cmd)?,
        cli::Command::Config(cmd) => commands::config::execute(cmd)?,
        cli::Command::Init(cmd) => commands::init::execute(cmd)?,
        cli::Command::Stats(cmd) => commands::stats::execute(cmd)?,
    }

    Ok(())
}
