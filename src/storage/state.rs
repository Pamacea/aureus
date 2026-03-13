//! Repository state tracking

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::config::get_config_dir;

const STATE_FILE: &str = "state.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub tracked_repos: HashMap<String, TrackedRepo>,
    pub last_update: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedRepo {
    pub path: String,
    pub name: String,
    pub current_version: Option<String>,
    pub hooks_enabled: bool,
    pub last_commit: Option<String>,
    pub commits_count: usize,
    pub first_tracked: chrono::DateTime<chrono::Utc>,
    pub last_tracked: chrono::DateTime<chrono::Utc>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            tracked_repos: HashMap::new(),
            last_update: None,
        }
    }
}

/// Get the state file path
pub fn get_state_path() -> Result<PathBuf> {
    Ok(get_config_dir()?.join(STATE_FILE))
}

/// Load state from file
pub fn load_state() -> Result<State> {
    let path = get_state_path()?;

    if !path.exists() {
        return Ok(State::default());
    }

    let content = fs::read_to_string(&path)
        .context("Failed to read state file")?;

    let state: State = serde_json::from_str(&content)
        .context("Failed to parse state file")?;

    Ok(state)
}

/// Save state to file
pub fn save_state(state: &State) -> Result<()> {
    let dir = get_config_dir()?;
    fs::create_dir_all(&dir)
        .context("Failed to create state directory")?;

    let path = dir.join(STATE_FILE);
    let content = serde_json::to_string_pretty(state)
        .context("Failed to serialize state")?;

    fs::write(&path, content)
        .context("Failed to write state file")?;

    Ok(())
}

/// Add or update a tracked repository
pub fn track_repo(path: &Path, name: &str) -> Result<TrackedRepo> {
    let mut state = load_state()?;
    let canonical = fs::canonicalize(path)?
        .to_string_lossy()
        .to_string();

    let now = chrono::Utc::now();
    let repo = state.tracked_repos.entry(canonical.clone())
        .or_insert_with(|| TrackedRepo {
            path: canonical.clone(),
            name: name.to_string(),
            current_version: None,
            hooks_enabled: false,
            last_commit: None,
            commits_count: 0,
            first_tracked: now,
            last_tracked: now,
        });

    repo.last_tracked = now;
    state.last_update = Some(now);

    // Clone the repo before saving state to avoid borrow issues
    let result = repo.clone();
    save_state(&state)?;
    Ok(result)
}

/// Remove a tracked repository
pub fn untrack_repo(path: &Path) -> Result<bool> {
    let mut state = load_state()?;
    let canonical = fs::canonicalize(path)?
        .to_string_lossy()
        .to_string();

    let removed = state.tracked_repos.remove(&canonical).is_some();
    save_state(&state)?;

    Ok(removed)
}

/// Get tracked repository by path
pub fn get_tracked_repo(path: &Path) -> Result<Option<TrackedRepo>> {
    let state = load_state()?;
    let canonical = fs::canonicalize(path)
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let Some(canonical) = canonical else {
        return Ok(None);
    };

    Ok(state.tracked_repos.get(canonical.as_str()).cloned())
}

/// Update repository version
pub fn update_repo_version(path: &Path, version: &str) -> Result<()> {
    let mut state = load_state()?;
    let canonical = fs::canonicalize(path)?
        .to_string_lossy()
        .to_string();

    if let Some(repo) = state.tracked_repos.get_mut(&canonical) {
        repo.current_version = Some(version.to_string());
        repo.commits_count += 1;
        repo.last_tracked = chrono::Utc::now();
    }

    save_state(&state)
}
