//! Aureus configuration management

use anyhow::{Context, Result};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".aureus";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub project: ProjectConfig,
    pub commit: CommitConfig,
    pub hooks: HooksConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>,
    pub default_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitConfig {
    pub types: CommitTypes,
    pub rules: CommitRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitTypes {
    #[serde(rename = "RELEASE")]
    pub release: CommitType,

    #[serde(rename = "UPDATE")]
    pub update: CommitType,

    #[serde(rename = "PATCH")]
    pub patch: CommitType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitType {
    pub description: String,
    pub emoji: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitRules {
    pub subject_max_length: usize,
    pub require_version: bool,
    pub require_project_name: bool,
    pub version_pattern: String,
    pub project_name_max_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HooksConfig {
    pub pre_commit: PreCommitConfig,
    pub commit_msg: CommitMsgConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreCommitConfig {
    pub enabled: bool,
    pub lint: bool,
    pub typecheck: bool,
    pub test: bool,
    pub secret_scan: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMsgConfig {
    pub enabled: bool,
    pub validate: bool,
    pub enforce_vrc: bool,
    pub allow_amend: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: None,
                default_branch: "main".to_string(),
            },
            commit: CommitConfig {
                types: CommitTypes {
                    release: CommitType {
                        description: "Major release - Breaking changes".to_string(),
                        emoji: "🚀".to_string(),
                        format: "RELEASE: {project} - v{version}".to_string(),
                    },
                    update: CommitType {
                        description: "Minor update - New features".to_string(),
                        emoji: "✨".to_string(),
                        format: "UPDATE: {project} - v{version}".to_string(),
                    },
                    patch: CommitType {
                        description: "Patch - Bug fixes".to_string(),
                        emoji: "🔧".to_string(),
                        format: "PATCH: {project} - v{version}".to_string(),
                    },
                },
                rules: CommitRules {
                    subject_max_length: 100,
                    require_version: true,
                    require_project_name: true,
                    version_pattern: r"v\d+\.\d+\.\d+".to_string(),
                    project_name_max_length: 50,
                },
            },
            hooks: HooksConfig {
                pre_commit: PreCommitConfig {
                    enabled: true,
                    lint: true,
                    typecheck: true,
                    test: false,
                    secret_scan: true,
                },
                commit_msg: CommitMsgConfig {
                    enabled: true,
                    validate: true,
                    enforce_vrc: true,
                    allow_amend: true,
                },
            },
        }
    }
}

/// Get the configuration directory path
pub fn get_config_dir() -> Result<PathBuf> {
    let home = home_dir().context("Cannot determine home directory")?;
    Ok(home.join(CONFIG_DIR))
}

/// Get the configuration file path
pub fn get_config_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join(CONFIG_FILE))
}

/// Load configuration from file
pub fn load_config() -> Result<Config> {
    let path = get_config_path()?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&path)
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse config file")?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &Config) -> Result<()> {
    let dir = get_config_dir()?;
    fs::create_dir_all(&dir)
        .context("Failed to create config directory")?;

    let path = dir.join(CONFIG_FILE);
    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(&path, content)
        .context("Failed to write config file")?;

    Ok(())
}

/// Get config value by key (dot notation) - re-exported for convenience
pub fn get_config_value(key: &str) -> Result<Option<serde_json::Value>> {
    let config = load_config()?;
    let json = serde_json::to_value(&config)
        .context("Failed to convert config to JSON")?;

    let mut current = &json;
    for part in key.split('.') {
        match current.get(part) {
            Some(value) => current = value,
            None => return Ok(None),
        }
    }

    Ok(Some(current.clone()))
}

/// Set config value by key (dot notation)
pub fn set_config_value(key: &str, value: &str) -> Result<()> {
    let mut config = load_config()?;
    let json = serde_json::to_value(&config)?;

    // Simple key assignment (not nested for now)
    match key {
        "project.name" => config.project.name = Some(value.to_string()),
        "project.default_branch" => config.project.default_branch = value.to_string(),
        _ => anyhow::bail!("Unknown config key: {}", key),
    }

    save_config(&config)
}
